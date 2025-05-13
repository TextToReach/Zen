use super::{ScopeManager::Scope};
use crate::features::tokenizer::{AssignmentMethod, CheckTokenVec, RemoveQuotes};
use crate::parsers::Parsers::Expression;
use crate::{
	DebugVec, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable, tokenize},
	library::Types::CutFromStart,
	parsers::Parsers::{self, ParserOutput},
	util::ScopeManager::{ScopeAction, ScopeManager},
};
use chumsky::prelude::*;
use colored::Colorize;
use defer::defer;
use std::collections::{HashMap, HashSet};
use std::ptr::null;

// It was created because i need it right now. Please never make this enum public or anything there's just too many of these.
// I've broken the oath.
#[derive(Debug, Clone)]
pub enum ConditionalBlockType {
	// If block has an id
	If(bool),   // If the condition was true or not.
	Elif(bool), // If the condition was true or not.
	Else,
	None,
}

impl ConditionalBlockType {
	pub const X: i32 = 5;
}

impl ConditionalBlockType {
	pub fn is_if(&self) -> bool {
		if let Self::If(_) = self { true } else { false }
	}
	pub fn is_elif(&self) -> bool {
		if let Self::Elif(_) = self { true } else { false }
	}
	pub fn is_else(&self) -> bool {
		if let Self::Else = self { true } else { false }
	}

	pub fn as_if(&self) -> bool {
		match self {
			ConditionalBlockType::If(x) => *x,
			_ => panic!("Is not if."),
		}
	}

	pub fn as_elif(&self) -> bool {
		match self {
			ConditionalBlockType::Elif(x) => *x,
			_ => panic!("Is not elif."),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockOutput {
	Break,
	Continue,
	None
}

pub fn ExecuteBlock(scope_id: usize, manager: &mut ScopeManager, last_conditional_block: &mut ConditionalBlockType) -> BlockOutput {
	// println!("Ran {scope_id}.");
	let scope = manager.get_scope(scope_id).expect(format!("Scope {scope_id} does not exist.").as_str()).clone();
	let block = scope.block.clone();
	let mut result = BlockOutput::None;

	for line in block.clone() {
		match line.clone() {
			InstructionEnum::Print(expr) => {
				PrintVec!(expr.iter().map(|x| x.evaluate(scope_id, manager)).collect::<Vec<_>>());
			}
			InstructionEnum::VariableDeclaration(name, value, method) => {
				let evaluated_value = value.evaluate(scope_id, manager);
				let previous_value = manager.get_var(scope_id, name.clone());
				// println!("Prev value: {:?}, now: {:?}", previous_value.clone().unwrap_or(1.0.into()), evaluated_value.clone());
				match method {
					AssignmentMethod::Set => manager.define_var(scope_id, name, evaluated_value),
					AssignmentMethod::Add => manager.define_var(scope_id, name, previous_value.unwrap() + evaluated_value),
					AssignmentMethod::Sub => manager.define_var(scope_id, name, previous_value.unwrap() - evaluated_value),
					AssignmentMethod::Mul => manager.define_var(scope_id, name, previous_value.unwrap() * evaluated_value),
					AssignmentMethod::Div => manager.define_var(scope_id, name, previous_value.unwrap() / evaluated_value),
				}
			}
			InstructionEnum::Repeat { repeat_count, scope_pointer } => {
				for _ in 0..repeat_count.floor() as i64 {
					match ExecuteBlock(scope_pointer, manager, last_conditional_block) {
						BlockOutput::Break => break,
						BlockOutput::Continue => continue,
						BlockOutput::None => {},
					}
				}
			}
			InstructionEnum::IfBlock { condition, scope_pointer } => {
				// println!("Ran If.");
				let is_condition_true = condition.isTruthy(scope_id, manager);
				*last_conditional_block = ConditionalBlockType::If(is_condition_true);
				if is_condition_true {
					result = ExecuteBlock(scope_pointer, manager, last_conditional_block);
				}
			}
			// InstructionEnum::ElifBlock { condition, scope_pointer } if matches!(last_conditional_block, ConditionalBlockType::If(_)) => {
			// 	println!("Ran Elif.");
			// 	let is_condition_true = condition.isTruthy(scope_id, manager);

			// 	*last_conditional_block = ConditionalBlockType::Elif(is_condition_true);
			// 	if !last_conditional_block.as_if() && is_condition_true {
			// 		ExecuteBlock(scope_pointer, manager, last_conditional_block);
			// 	}
			// }
			// InstructionEnum::ElseBlock { scope_pointer } if matches!(last_conditional_block, ConditionalBlockType::If(_) | ConditionalBlockType::Elif(_))  => {
			// 	println!("Ran Else.");
			// 	*last_conditional_block = ConditionalBlockType::Else;
			// }
			InstructionEnum::ElifBlock { condition, scope_pointer } => panic!("Elif block with no preceding if blocks."),
			InstructionEnum::ElseBlock { scope_pointer } => panic!("Else block with no preceding if/elif blocks."),
			InstructionEnum::Break => {
				result = BlockOutput::Break;
				break;
			}
			InstructionEnum::Continue => {
				result = BlockOutput::Continue;
				break;
			}
			_ => todo!(),
		}
	}
	result
}

pub fn ProcessLine(
	line_feed: Vec<TokenData>,
	instr: (ParserOutput, InstructionEnum),
	current_scope_id: &mut usize,
	manager: &mut ScopeManager,
	last_conditional_block: &mut ConditionalBlockType,
) {
	let line_feed_tab_count = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
	let inferred_scope_depth = manager.get_depth(current_scope_id.clone());

	if line_feed_tab_count != inferred_scope_depth {
		if line_feed_tab_count > inferred_scope_depth {
			panic!("Indentation fault. {} > {}", line_feed_tab_count, inferred_scope_depth)
		} else {
			let temp = current_scope_id.clone();
			*current_scope_id = manager.get_parent(current_scope_id.clone()).unwrap();
			manager.remove_scope(temp);
		}
	}

	// let current_scope = manager.get_scope(current_scope_id.clone()).unwrap();
	if instr.0.indent {
		let mut instr_enum = instr.clone().1;
		let new_scope = manager.create_scope(Some(*current_scope_id), Some(instr_enum.as_block_action())); // Create new scope
		instr_enum.set_block_pointer(new_scope); // Set the instruction's pointer to the new scope
		manager.push_code_to_scope(*current_scope_id, &instr_enum); // Push the instruction to the parent block
		*current_scope_id = new_scope;
	} else {
		manager.push_code_to_scope(*current_scope_id, &instr.1);
	}
}

pub fn index(input: &mut Vec<String>) {
	let root_scope = 0; // Id of the root scope is 0. So this is just an alias for number zero
	let mut manager = ScopeManager::new();
	let mut currentScope = root_scope;
	let mut last_conditional_block = ConditionalBlockType::None;

	for line in input.iter_mut() {
		for chunk in line.split(";") {
			let raw_line_feed = tokenize(chunk);
			if !raw_line_feed.is_all_ok() {
				continue;
			}
			let line_feed_without_tabs = raw_line_feed.iter().filter(|x| x.token != TokenTable::Tab).cloned().collect::<Vec<_>>();

			if !line_feed_without_tabs.starts_with(&[TokenTable::Comment.asTokenData()]) {
				match Parsers::parser().parse(line_feed_without_tabs.clone()) {
					Ok(x) => {
						let feedIndentLevel = raw_line_feed.count_from_start(|x| x.token == TokenTable::Tab);
						// manager.push_code_to_scope(currentScope, &x.1);
						ProcessLine(raw_line_feed, x, &mut currentScope, &mut manager, &mut last_conditional_block);
					}
					Err(e) => {
						panic!("Error happened: {:#?}", e)
					}
				}
			}
		}
	}

	ExecuteBlock(root_scope, &mut manager, &mut last_conditional_block);
}
