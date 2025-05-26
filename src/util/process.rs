use super::ScopeManager::{ConditionBlock, ConditionStructure, Scope};
use crate::features::tokenizer::{AssignmentMethod, CheckTokenVec, ConditionBlockType, RemoveQuotes};
use crate::library::Error::{CokFazlaArguman, EksikArguman, FonksiyonBulunamadı, GirintiHatası};
use crate::library::Types::Object;
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
use miette::{Error, NamedSource, SourceSpan};
use std::collections::{HashMap, HashSet};
use std::ptr::null;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockOutput {
	Break,
	Continue,
	None,
}

pub fn ExecuteBlock(scope_id: usize, manager: &mut ScopeManager, src: NamedSource<String>, span: SourceSpan) -> miette::Result<BlockOutput> {
	// println!("Running scope {scope_id}...");
	let scope = manager.get_scope(scope_id).expect(format!("Scope {scope_id} does not exist.").as_str());
	let block = scope.block.clone();
	let mut result = BlockOutput::None;

	for line in block.clone() {
		match line.clone() {
			InstructionEnum::Print(expr) => {
				PrintVec!(expr.iter().map(|x| x.evaluate(scope_id, manager)).collect::<Vec<_>>());
			}
			InstructionEnum::VariableDeclaration(name, value, method) => {
				let evaluated_value = value.evaluate(scope_id, manager);
				let new_value = match method {
					AssignmentMethod::Set => evaluated_value,
					AssignmentMethod::Add => manager.get_var(scope_id, name.clone()).expect("No previous value") + evaluated_value,
					AssignmentMethod::Sub => manager.get_var(scope_id, name.clone()).expect("No previous value") - evaluated_value,
					AssignmentMethod::Mul => manager.get_var(scope_id, name.clone()).expect("No previous value") * evaluated_value,
					AssignmentMethod::Div => manager.get_var(scope_id, name.clone()).expect("No previous value") / evaluated_value,
				};

				let mut temp_scope_id = scope_id.clone();
				loop {
					let current_scope_var = manager.does_var_exists(scope_id, name.clone());
					if current_scope_var {
						manager.set_var(temp_scope_id, name.clone(), new_value.clone());
						break;
					} else {
						if let Some(parent) = manager.get_parent(temp_scope_id) {
							temp_scope_id = parent;
						} else {
							manager.set_var(temp_scope_id, name.clone(), new_value.clone());
							break;
						}
					}
				}
			}
			InstructionEnum::WhileTrue { scope_pointer } => {
				loop {
					match ExecuteBlock(scope_pointer, manager, src.clone(), span) {
						Ok(BlockOutput::Break) => break,
						Ok(BlockOutput::Continue) => continue,
						Ok(BlockOutput::None) => {}
						Err(e) => {
							return Err(e);
						}
					}
				}
			}
			InstructionEnum::Repeat { repeat_count, scope_pointer } => {
				for _ in 0..(repeat_count.evaluate(scope_id, manager).expectToBeNumber(src.clone(), span)?.value).floor() as i64 {
					match ExecuteBlock(scope_pointer, manager, src.clone(), span) {
						Ok(BlockOutput::Break) => break,
						Ok(BlockOutput::Continue) => continue,
						Ok(BlockOutput::None) => {}
						Err(e) => {
							return Err(e);
						}
					}
				}
			}
			InstructionEnum::For {
				from,
				to,
				step,
				name,
				scope_pointer,
			} => {
				for index in ((from.evaluate(scope_id, manager).expectToBeNumber(src.clone(), span)?.value).floor() as i64
					..(to.evaluate(scope_id, manager).expectToBeNumber(src.clone(), span)?.value).floor() as i64).step_by(
						(step.unwrap_or(Expression::Value(Box::new(Object::from(1f64)))).evaluate(scope_id, manager).expectToBeNumber(src.clone(), span)?.value).floor() as usize
					)
				{
					manager.set_var(scope_pointer, name.clone(), Object::from(index as f64));
					match ExecuteBlock(scope_pointer, manager, src.clone(), span) {
						Ok(BlockOutput::Break) => break,
						Ok(BlockOutput::Continue) => continue,
						Ok(BlockOutput::None) => {}
						Err(e) => {
							return Err(e);
						}
					}
				}
			}
			InstructionEnum::Function { name, args, scope_pointer } => {
				let resolved_args = args.iter().map(|x| x.toResolved(scope_id, manager)).collect::<Vec<_>>();
				manager.declare_function(scope_id, name.clone(), resolved_args, scope_pointer.clone());
			}
			InstructionEnum::CallFunction { name, args } => {
				let function_scope = manager.get_function(scope_id, name.clone()).ok_or_else(|| FonksiyonBulunamadı {
					src: src.clone(),
					bad_bit: span,
				})?;
				let resolved_args = args.iter().map(|x| x.evaluate(scope_id, manager)).collect::<Vec<_>>();
				let funcdef_args = &function_scope.args;
				if resolved_args.len() > funcdef_args.len() {
					return Err(CokFazlaArguman {
						src,
						bad_bit: span,
						expected: Some(funcdef_args.len()),
						got: Some(resolved_args.len()),
					})?;
				}
				for (i, param) in funcdef_args.iter().enumerate() {
					let value = if i < resolved_args.len() {
						resolved_args[i].clone()
					} else if let Some(default) = &param.default_value {
						default.clone()
					} else {
						return Err(EksikArguman {
							src,
							bad_bit: span,
							expected: Some(param.name.clone()),
						})?;
					};

					// Type checking
					if let Some(expected_type) = &param.data_type {
						value.expectToBe(expected_type.clone(), src.clone(), span)?
					}

					manager.set_var(function_scope.scope_pointer, param.name.clone(), value);
				}
				ExecuteBlock(function_scope.scope_pointer, manager, src.clone(), span)?;
			}
			InstructionEnum::Break => {
				result = BlockOutput::Break;
				break;
			}
			InstructionEnum::Continue => {
				result = BlockOutput::Continue;
				break;
			}
			InstructionEnum::Condition(condition) => {
				// Evaluate the main condition
				if condition.If.condition.isTruthy(scope_id, manager) {
					match ExecuteBlock(condition.If.scope_pointer, manager, src.clone(), span) {
						Ok(BlockOutput::Break) => {
							result = BlockOutput::Break;
							break;
						}
						Ok(BlockOutput::Continue) => {
							result = BlockOutput::Continue;
							break;
						}
						Ok(BlockOutput::None) => {

						}
						Err(e) => {
							return Err(e);
						}
					}
				} else {
					// Check elifs
					let mut executed = false;
					for elif in &condition.Elif {
						if elif.condition.isTruthy(scope_id, manager) {
							match ExecuteBlock(elif.scope_pointer, manager, src.clone(), span) {
								Ok(BlockOutput::Break) => {
									result = BlockOutput::Break;
									break;
								}
								Ok(BlockOutput::Continue) => {
									result = BlockOutput::Continue;
									break;
								}
								Ok(BlockOutput::None) => {}
								Err(e) => {
									return Err(e);
								}
							}
							executed = true;
							break;
						}
					}
					// Else block
					if !executed && condition.Else.scope_pointer != 0 {
						match ExecuteBlock(condition.Else.scope_pointer, manager, src.clone(), span) {
							Ok(BlockOutput::Break) => {
								result = BlockOutput::Break;
								break;
							}
							Ok(BlockOutput::Continue) => {
								result = BlockOutput::Continue;
								break;
							}
							Ok(BlockOutput::None) => {}
							Err(e) => {
								return Err(e);
							}
						}
					}
				}
			}
			_ => todo!(),
		}
	}
	Ok(result)
}

pub fn ProcessLine(
	full_source: String,
	raw_line_feed_string: String,
	line_feed: Vec<TokenData>,
	instr: (ParserOutput, InstructionEnum),
	current_scope_id: &mut usize,
	manager: &mut ScopeManager,
	opts: &Runopts,
	fileandline: (&str, u32), // conditional_grup:
) -> miette::Result<()> {
	let line_indent = line_feed.iter().take_while(|x| x.token == TokenTable::Tab).count();
	let mut scope_depth = manager.get_depth(*current_scope_id);
	if line_indent < scope_depth {
		while scope_depth > line_indent {
			if let Some(parent) = manager.get_parent(*current_scope_id) {
				*current_scope_id = parent;
				scope_depth -= 1;
			} else {
				break;
			}
		}
	} else if line_indent > scope_depth + 1 {
		if opts.strict {
			return Err(GirintiHatası {
				src: NamedSource::new(fileandline.0, full_source).with_language("Zen"),
				bad_bit: SourceSpan::new(scope_depth.into(), (line_indent - scope_depth) as usize),
			})?;
		}
	}

	if instr.0.indent {
		let mut instr_enum = instr.clone().1;
		let new_scope = match instr_enum {
			InstructionEnum::IfBlock { .. } => manager.create_transparent_scope(*current_scope_id, Some(instr_enum.as_block_action())),
			InstructionEnum::ElifBlock { .. } => manager.create_transparent_scope(*current_scope_id, Some(instr_enum.as_block_action())),
			InstructionEnum::ElseBlock { .. } => manager.create_transparent_scope(*current_scope_id, Some(instr_enum.as_block_action())),
			InstructionEnum::Function { .. } => manager.create_isolated_scope(*current_scope_id, Some(instr_enum.as_block_action())),
			_ => manager.create_scope(Some(*current_scope_id), Some(instr_enum.as_block_action())),
		};

		match instr_enum {
			InstructionEnum::IfBlock { .. } => {
				instr_enum = InstructionEnum::Condition(ConditionBlock::new(ConditionStructure {
					scope_pointer: new_scope,
					condition: instr_enum.as_expression(),
				}));
				manager.push_code_to_scope(*current_scope_id, &instr_enum);
				*current_scope_id = new_scope;
			}
			InstructionEnum::ElifBlock { .. } => {
				if let Some(last_instr) = manager.get_scope_mut(*current_scope_id).unwrap().block.last_mut() {
					if let InstructionEnum::Condition(con) = last_instr {
						con.push_elif(ConditionStructure {
							scope_pointer: new_scope,
							condition: instr_enum.as_expression(),
						});
					}
				}
				*current_scope_id = new_scope;
			}
			InstructionEnum::ElseBlock { .. } => {
				if let Some(last_instr) = manager.get_scope_mut(*current_scope_id).unwrap().block.last_mut() {
					if let InstructionEnum::Condition(con) = last_instr {
						con.push_else(ConditionStructure {
							scope_pointer: new_scope,
							condition: instr_enum.as_expression(),
						});
					}
				}
				*current_scope_id = new_scope;
			}
			_ => {
				instr_enum.set_block_pointer(new_scope);
				manager.push_code_to_scope(*current_scope_id, &instr_enum);
				*current_scope_id = new_scope;
			}
		}
	} else {
		manager.push_code_to_scope(*current_scope_id, &instr.1);
	}

	Ok(())
}

pub struct Runopts {
	verbose: bool,
	strict: bool,
}

pub fn index(input: &mut Vec<String>, full_source: String, verbose: bool, strict: bool, filename: &str) -> miette::Result<()> {
	let mut manager = ScopeManager::new();
	let root_scope = manager.create_scope(None, None);
	let mut currentScope = root_scope;
	let mut line_index = 0;
	let opts = Runopts { verbose, strict };

	for line in input.iter_mut() {
		line_index += 1;
		for chunk in line.split(";") {
			let raw_line_feed = tokenize(chunk);
			// println!("{raw_line_feed:#?}");
			if !raw_line_feed.is_all_ok() {
				continue;
			}
			let line_feed_without_tabs = raw_line_feed.iter().filter(|x| x.token != TokenTable::Tab).cloned().collect::<Vec<_>>();

			if !line_feed_without_tabs.starts_with(&[TokenTable::Comment.asTokenData()]) && !line_feed_without_tabs.is_empty() {
				match Parsers::parser().parse(line_feed_without_tabs.clone()) {
					Ok(res) => {
						match ProcessLine(
							chunk.to_owned(),
							full_source.clone(),
							raw_line_feed,
							res.clone(),
							&mut currentScope,
							&mut manager,
							&opts,
							(filename, line_index),
						) {
							Err(e) => {
								return Err(e);
							}
							_ => {}
						}
					}
					Err(e) => {
						panic!("Error happened: {:#?}", e)
					}
				}
			}
		}
	}

	// println!("{:#?}\n-----------------------------------", manager.get_scope(0));
	// println!("{:#?}\n-----------------------------------", manager.get_scope(1));
	// println!("{:#?}\n-----------------------------------", manager.get_scope(2));
	// println!("{:#?}\n-----------------------------------", manager.get_scope(3));

	if let Some((w, h)) = term_size::dimensions() {
		manager.set_global(root_scope, "ekrangenişliği".to_string(), Object::from(w as f64));
		manager.set_global(root_scope, "ekranyüksekliği".to_string(), Object::from(h as f64));
	} else {
		manager.set_global(root_scope, "ekrangenişliği".to_string(), Object::from(0 as f64));
		manager.set_global(root_scope, "ekranyüksekliği".to_string(), Object::from(0 as f64));
	}

	ExecuteBlock(
		root_scope,
		&mut manager,
		NamedSource::new(filename, full_source.clone()),
		SourceSpan::new(0.into(), full_source.len()),
	)?;
	Ok(())
}
