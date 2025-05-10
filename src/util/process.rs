use super::{ScopeManager::Scope, Util::generate_8_digit_id};
use crate::features::tokenizer::RemoveQuotes;
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

pub fn ExecuteLine(line: &InstructionEnum) {
	match line.clone() {
		InstructionEnum::Print(expr) => {
			PrintVec!(expr.iter().map(|x| x.evaluate()).collect::<Vec<_>>());
		}
		_ => todo!(),
	}
}

pub fn ExecuteScope(scope: &Scope) {
	match scope.clone().action {
		None => {
			for instr in scope.clone().block {
				ExecuteLine(&instr);
			}
		}
		Some(act) => match act {
			ScopeAction::Repeat(n) => {
				for i in num::range(0, n.floor() as i64) {
					for instr in scope.clone().block {
						ExecuteLine(&instr);
					}
				}
			}
			ScopeAction::IfBlock(n) => {
				
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
			ExecuteScope(manager.get_scope(temp).unwrap());
			manager.remove_scope(temp);
		}
	}

	match manager.get_scope(current_scope.clone()).unwrap().action.clone() {
		Some(act) => {
			// The instructions inside this scope will be handled by that scope's executer.
			// println!("{} {}.", "Parent scope has action:".red(), act);
			match act {
				ScopeAction::Repeat(n) => {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}
				_ => todo!(),
			}
		}
		None if instr.0.indent => {
			//
		}
		None => {
			ExecuteLine(&instr.1);
		}
	}

	// -------------------------------------------------------------------------------------- //

	match instr.0.indent {
		// That means it creates a scope.
		true => {
			let newScope = match instr.1 {
				InstructionEnum::Repeat(n) => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::Repeat(n))),
				_ => todo!("Implement other actions (ifs, whiles etc.)"),
			};
			*current_scope = newScope;
		}
		false => {}
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

	if let Some(last_scope) = manager.get_children_tree(rootScope).first() {
        ExecuteScope(manager.get_scope(*last_scope).unwrap());
    }
	// The program doesn't know how to end the last scope if no instructions come after that last scope.
	// So what this line does is that it executes that last scope. But I'm more than sure that this will start failing in the future.
	// So a reminder to myself.
	// TODO: remove this line.
}
