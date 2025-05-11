use super::{ScopeManager::Scope, Util::generate_8_digit_id};
use crate::features::tokenizer::{AssignmentMethod, CheckTokenVec, RemoveQuotes};
use crate::parsers::Parsers::Expression;
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
use std::ptr::null;

pub fn ExecuteCode(line: &InstructionEnum, scope_id: usize, manager: &mut ScopeManager, last_conditional_block: &mut ConditionalBlockType) {
	match line.clone() {
		InstructionEnum::Print(expr) => {
			PrintVec!(expr.iter().map(|x| x.evaluate(scope_id, manager)).collect::<Vec<_>>());
		}
		InstructionEnum::Block(n) => {
			ExecuteBlock(n, manager, last_conditional_block);
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

// It was created because i need it right now. Please never make this enum public or anything there's just too many of these.
// I've broken the oath.
#[derive(Debug, Clone)]
pub enum ConditionalBlockType { // If block has an id
	If(bool), // If the condition was true or not.
	Elif(bool), // If the condition was true or not.
	Else,
	None
}

impl ConditionalBlockType {
	pub const X: i32 = 5;
}

impl ConditionalBlockType {
	pub fn is_if(&self) -> bool { if let Self::If(_) = self {true} else {false} }
	pub fn is_elif(&self) -> bool { if let Self::Elif(_) = self {true} else {false} }
	pub fn is_else(&self) -> bool { if let Self::Else = self {true} else {false} }

	pub fn as_if(&self) -> bool { 
		match self {
			ConditionalBlockType::If(x) => *x,
			_ => panic!("Is not if.")
		}
	}

	pub fn as_elif(&self) -> bool { 
		match self {
			ConditionalBlockType::Elif(x) => *x,
			_ => panic!("Is not elif.")
		}
	}
}

pub fn ExecuteBlock(scope_id: usize, manager: &mut ScopeManager, last_conditional_block: &mut ConditionalBlockType) {
	let scope = manager.get_scope(scope_id).unwrap().clone();
	let block = scope.block.clone();
	
	match scope.action {
		None => {
			for instr in block.iter() {
				ExecuteCode(instr, scope_id, manager, last_conditional_block);
			}
		}
		Some(act) => match act {
			ScopeAction::Repeat(n) => {
				for _ in num::range(0, n.floor() as i64) {
					for instr in block.iter() {
						ExecuteCode(instr, scope_id, manager, last_conditional_block);
					}
				}
			}
			ScopeAction::IfBlock{ condition } => {
				println!("Ran If.");
				let is_condition_true = condition.isTruthy(scope_id, manager);
				*last_conditional_block = ConditionalBlockType::If(is_condition_true);
				if is_condition_true {
					for instr in block.iter() {
						ExecuteCode(instr, scope_id, manager, last_conditional_block);
					}
				}
			}
			ScopeAction::ElifBlock { condition } if matches!(last_conditional_block, ConditionalBlockType::If(_)) => {
				println!("Ran Elif.");
				let is_condition_true = condition.isTruthy(scope_id, manager);
				
				*last_conditional_block = ConditionalBlockType::Elif(is_condition_true);
				if !last_conditional_block.as_if() && is_condition_true {
					for instr in block.iter() {
						ExecuteCode(instr, scope_id, manager, last_conditional_block);
					}
				}
			}
			ScopeAction::ElseBlock if matches!(last_conditional_block, ConditionalBlockType::If(_) | ConditionalBlockType::Elif(_))  => {
				println!("Ran Else.");
				*last_conditional_block = ConditionalBlockType::Else;
			}
			ScopeAction::ElifBlock { condition } => panic!("Elif block with no preceding if blocks."),
			ScopeAction::ElseBlock => panic!("Else block with no preceding if/elif blocks."),
			_ => todo!(),
		},
	}
}

pub fn ProcessLine(line_feed: Vec<TokenData>, instr: (ParserOutput, InstructionEnum), current_scope: &mut usize, manager: &mut ScopeManager, last_conditional_block: &mut ConditionalBlockType) {
	let line_feed_tab_count = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
	let inferred_scope_depth = manager.get_depth(current_scope.clone());

	if line_feed_tab_count != inferred_scope_depth {
		if line_feed_tab_count > inferred_scope_depth {
			panic!("Indentation fault. {} > {}", line_feed_tab_count, inferred_scope_depth)
		} else {
			let temp = current_scope.clone();
			*current_scope = manager.get_parent(current_scope.clone()).unwrap();
			// println!("Ending scope and executing it: {:#?}", manager.get_scope(temp));
			ExecuteBlock(temp, manager, last_conditional_block);
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
				ScopeAction::IfBlock { condition } => {
					manager.push_code_to_scope(*current_scope, &InstructionEnum::Block(new_scope));
				}
				_ => todo!(),
			}

			*current_scope = new_scope;
		},
		Some(act) => {
			match act { // TODO: After all blocks have been implemented remove this match.
				ScopeAction::Repeat(_) => {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}
				ScopeAction::IfBlock{ condition } => {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}
				ScopeAction::ElifBlock { condition }=> {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}
				ScopeAction::ElseBlock => {
					manager.push_code_to_scope(current_scope.clone(), &instr.1);
				}

				_ => todo!(),
			}
		}
		None if instr.0.indent => { // Top level code, but it creates a new scope
			let newScope = match instr.1 {
				InstructionEnum::Repeat(n) => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::Repeat(n))),
				InstructionEnum::IfBlock(condition) => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::IfBlock{ condition })),
				InstructionEnum::ElifBlock(condition) => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::ElifBlock{ condition })),
				InstructionEnum::ElseBlock => manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::ElseBlock)),
				_ => todo!("Implement other actions (ifs, whiles etc.)"),
			};
			*current_scope = newScope;
		}
		None => { // Top level code
			ExecuteCode(&instr.1, *current_scope, manager, last_conditional_block);
		}
	}
}

pub fn index(input: &mut Vec<String>) {
	let rootScope = 0; // Id of the root scope is 0. So this is just an alias for number zero
	let mut manager = ScopeManager::new();
	let mut currentScope = rootScope;
	let mut last_conditional_block = ConditionalBlockType::None;

	for line in input.iter_mut() {
		for chunk in line.split(";") {
			let raw_line_feed = tokenize(line);
			if !raw_line_feed.is_all_ok() { continue; }
			let line_feed_without_tabs = raw_line_feed.iter().filter(|x| x.token != TokenTable::Tab).cloned().collect::<Vec<_>>();
			
			if !line_feed_without_tabs.starts_with(&[TokenTable::Comment.asTokenData()]) {
				match Parsers::parser().parse(line_feed_without_tabs.clone()) {
					Ok(x) => {
						let feedIndentLevel = raw_line_feed.count_from_start(|x| x.token == TokenTable::Tab);
						manager.push_code_to_scope(currentScope, &x.1);
						// ProcessLine(raw_line_feed, x, &mut currentScope, &mut manager, &mut last_conditional_block);
					}
					Err(e) => {
						panic!("Error happened: {:#?}", e)
					}
				}
			}
		}
	}

	println!("{:#?}", manager.get_scope(rootScope));
	if let Some(last_scope) = manager.get_children_tree(rootScope).first() {
		if let Some(act) = &manager.get_scope(*last_scope).unwrap().action {
			println!("Last one was an if! {act:#?}");
			if let ScopeAction::IfBlock { condition } = act {
			}
		}
        ExecuteBlock(*last_scope, &mut manager, &mut last_conditional_block);
    }
	// The program doesn't know how to end the last scope if no instructions come after that last scope.
	// So what this line does is that it executes that last scope. But I'm more than sure that this will start failing in the future.
	// So a reminder to myself.
	// TODO: remove this line.
}
