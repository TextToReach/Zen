use super::{ScopeManager::Scope, Util::generate_8_digit_id};
use crate::features::tokenizer::{AssignmentMethod, RemoveQuotes};
use crate::{
	DebugVec, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable, tokenize},
	parsers::Parsers::{self, ParserOutput},
	util::ScopeManager::{ScopeAction, ScopeManager},
    library::Types::CutFromStart,
};
use chumsky::prelude::*;
use colored::Colorize;
use defer::defer;
use std::collections::{HashMap, HashSet};

pub fn ExecuteCode(line: &InstructionEnum, scope_id: usize, manager: &mut ScopeManager) {
	match line.clone() {
		InstructionEnum::Print(expr) => {
			PrintVec!(expr.iter().map(|x| x.evaluate(scope_id, manager)).collect::<Vec<_>>());
		}
		InstructionEnum::Block(n) => {
			ExecuteBlock(n, manager);
		}
		InstructionEnum::VariableDeclaration(name, value, method) => {
			let evaluated_value = value.evaluate(scope_id, manager);
			let previous_value = manager.get_var(scope_id, name.clone());
			match method {
				AssignmentMethod::Set => manager.define_var(scope_id, name, evaluated_value),
				AssignmentMethod::Add => manager.define_var(scope_id, name, previous_value.unwrap() + evaluated_value),
				AssignmentMethod::Sub => manager.define_var(scope_id, name, previous_value.unwrap() - evaluated_value),
				AssignmentMethod::Mul => manager.define_var(scope_id, name, previous_value.unwrap() * evaluated_value),
				AssignmentMethod::Div => manager.define_var(scope_id, name, previous_value.unwrap() / evaluated_value),
			}
			
		}
		_ => todo!(),
	}	
}

pub fn ExecuteBlock(scope_id: usize, manager: &mut ScopeManager) {
	let scope = manager.get_scope(scope_id).unwrap().clone();
	let block = scope.block.clone();
	match scope.action {
		None => {
			for instr in block.iter() {
				ExecuteCode(instr, scope_id, manager);
			}
		}
		Some(act) => match act {
			ScopeAction::Repeat(n) => {
				for _ in num::range(0, n.floor() as i64) {
					for instr in block.iter() {
						ExecuteCode(instr, scope_id, manager);
					}
				}
			}
			ScopeAction::IfBlock(c) => {
				if c.isTruthy(scope_id, manager) {
					for instr in block.iter() {
						ExecuteCode(instr, scope_id, manager);
					}
				}
			}
			_ => todo!(),
		},
	}
}

pub fn ProcessLine(line_feed: Vec<TokenData>, instr: (ParserOutput, InstructionEnum), current_scope: &mut usize, manager: &mut ScopeManager) {
	let line_feed_tab_count = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
	let inferred_scope_depth = manager.get_depth(current_scope.clone());

	if line_feed_tab_count != inferred_scope_depth {
		if line_feed_tab_count > inferred_scope_depth {
			panic!("Indentation fault. {} > {}", line_feed_tab_count, inferred_scope_depth)
		} else {
			let temp = current_scope.clone();
			*current_scope = manager.get_parent(current_scope.clone()).unwrap();
			println!("Ending scope and executing it: {:#?}", manager.get_scope(temp));
			ExecuteBlock(temp, manager);
			manager.remove_scope(temp);
		}
	}

	match manager.get_scope(current_scope.clone()).unwrap().action.clone() {
		Some(act) if instr.0.indent => {
			let new_scope = manager.create_scope(Some(current_scope.clone()), Some(instr.1.as_block_action()));
			match act {
				ScopeAction::Repeat(n) => {
					manager.push_code_to_scope(*current_scope, &InstructionEnum::Block(new_scope));
				}
				ScopeAction::IfBlock(c) => {
					manager.push_code_to_scope(*current_scope, &InstructionEnum::Block(new_scope));
				}
				_ => todo!(),
			}

			*current_scope = new_scope;
		},
		Some(act) => {
			match act { // TODO: After all blocks have been implemented remove this match.
				ScopeAction::Repeat(n) => {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}
				ScopeAction::IfBlock(c) => {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}
				_ => todo!(),
			}
		}
		None if instr.0.indent => { // Top level code, but it creates a new scope
			let newScope = match instr.1 {
				InstructionEnum::Repeat(n) => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::Repeat(n))),
				InstructionEnum::IfBlock(c) => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::IfBlock(c))),
				_ => todo!("Implement other actions (ifs, whiles etc.)"),
			};
			*current_scope = newScope;
		}
		None => { // Top level code
			ExecuteCode(&instr.1, *current_scope, manager);
		}
	}
}

pub fn index(input: &mut Vec<String>) {
	let rootScope = 0; // Id of the root scope is 0. So this is just an alias for number zero
	let mut manager = ScopeManager::new();
	let mut currentScope = rootScope;

	for line in input.iter_mut() {
		for chunk in line.split(";") {
			let line_feed = tokenize(line)
				.iter()
				.map(|x| TokenData {
					isOk: x.isOk,
					token: x.token.clone(),
					slice: match x.token {
						TokenTable::StringLiteral => x.slice.remove_quotes(),
						_ => x.slice.clone(),
					},
					span: x.span.clone(),
				})
				.collect::<Vec<_>>();
			if line_feed.is_empty() || line_feed.iter().any(|x| !x.isOk) {
				continue;
			}
			let clean_line_feed = line_feed.iter().filter(|x| x.token != TokenTable::Tab).cloned().collect::<Vec<_>>();

			match Parsers::parser().parse(clean_line_feed.clone()) {
				Ok(x) => {
					let feedIndentLevel = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
                    // println!("Line: {:#?}", line_feed);
                    // println!("X: {:#?}", x);
					ProcessLine(line_feed, x, &mut currentScope, &mut manager);
				}
				Err(e) => {
					panic!("Error happened: {:#?}", e)
				}
			}
		}
	}

	// for i in manager.get_children_tree(rootScope).iter().map(|x| manager.get_scope(*x).unwrap()).collect::<Vec<_>>() {
	// 	println!("{i:#?}\n\n-----------------------------\n\n")
	// }
	if let Some(last_scope) = manager.get_children_tree(rootScope).first() {
        ExecuteBlock(*last_scope, &mut manager);
    }
	// The program doesn't know how to end the last scope if no instructions come after that last scope.
	// So what this line does is that it executes that last scope. But I'm more than sure that this will start failing in the future.
	// So a reminder to myself.
	// TODO: remove this line.
}
