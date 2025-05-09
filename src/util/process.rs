use crate::{
	features::tokenizer::{tokenize, InstructionEnum, TokenData, TokenTable}, parsers::Parsers::{self, ParserOutput}, util::ScopeManager::{ScopeAction, ScopeManager}, DebugVec, Print, PrintVec
};
use chumsky::prelude::*;
use colored::Colorize;
use defer::defer;
use std::collections::{HashMap, HashSet};
use super::{ScopeManager::Scope, Util::generate_8_digit_id};
use crate::features::tokenizer::RemoveQuotes;

pub fn ExecuteLine(line: &InstructionEnum) {
    match line.clone() {
        InstructionEnum::Print(expr) => {
            PrintVec!(expr.iter().map(|x| x.evaluate()).collect::<Vec<_>>());
        }
        _ => todo!()
    }
}

pub fn ExecuteScope(scope: &Scope) {
    match scope.clone().action {
        None => {
            for instr in scope.clone().block {
                ExecuteLine(&instr);
            }
        }
        Some(act) => {
            match act {
                ScopeAction::Repeat(n) => {
                    for i in num::range(0, n.floor() as i64) {
                        for instr in scope.clone().block {
                            ExecuteLine(&instr);
                        }
                    }
                }
                _ => todo!()
            }
        }
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
            println!("Destroying scope and executing it: {:#?}", manager.get_scope(temp));
            ExecuteScope(manager.get_scope(temp).unwrap());
            manager.remove_scope(temp);
        }
    }

    match manager.get_scope(current_scope.clone()).unwrap().action.clone() {
        Some(act) => { // The instructions inside this scope will be handled by that scope's executer.
            println!("{} {}.", "Parent scope has action:".red(), act);
            match act {
                ScopeAction::Repeat(n) => {
                    manager.push_code_to_scope(current_scope.clone(), &instr.1);
                }
                _ => todo!()
            }
        },
        None if instr.0.indent => {
            println!("No parent scope found but this will create one. This line will not be executed further.")
        }
        None => {
            ExecuteLine(&instr.1);
            println!("No parent scope found.")
        }
    }

    // -------------------------------------------------------------------------------------- //

    match instr.0.indent { // That means it creates a scope.
        true => {
            let newScope = match instr.1 {
                InstructionEnum::Repeat(n) => {
                    manager.create_scope(Some(current_scope.clone()), Some(ScopeAction::Repeat(n)))
                }
                _ => todo!("Implement other actions (ifs, whiles etc.)")
            };
            *current_scope = newScope;
        }
        false => {

        }
    }
}

trait CutFromStart<T> {
    fn cut_from_start(&self, whr: fn(&T) -> bool, amount: usize) -> Self;
    fn count_from_start(&self, whr: fn(&T) -> bool) -> usize;
}
impl CutFromStart<TokenData> for Vec<TokenData> {
    fn cut_from_start(&self, whr: fn(&TokenData) -> bool, amount: usize) -> Self {
        let mut inner_self = self.clone();
        let amount = usize::min(amount, inner_self.len());
        for i in (0..amount).rev() {
            if whr(&inner_self[i]) {
                inner_self.remove(i);
            }
        }
        inner_self
    }

    fn count_from_start(&self, whr: fn(&TokenData) -> bool) -> usize {
        let mut count = 0;
        for el in self.clone() {
            if whr(&el) {
                count += 1;
            }
        }

        count
    }
}

pub fn index(input: &mut Vec<String>) {
    let rootScope = 0; // Id of the root scope is 0. So this is just an alias for number zero
    let mut manager = ScopeManager::new();
    let mut currentScope = rootScope;

    for line in input.iter_mut() {
		for chunk in line.split(";") {
			let line_feed = tokenize(line).iter().map(|x| {
                TokenData { isOk: x.isOk, token: x.token.clone(), slice: match x.token {
                    TokenTable::StringLiteral => x.slice.remove_quotes(),
                    _ => x.slice.clone()
                }, span: x.span.clone() }
            }).collect::<Vec<_>>();
            if line_feed.is_empty() || line_feed.iter().any(|x| !x.isOk) { continue; }
            let clean_line_feed = line_feed.iter().filter(|x| x.token != TokenTable::Tab).cloned().collect::<Vec<_>>();

            match Parsers::parser().parse(clean_line_feed.clone()) {
                Ok(x) => {
                    let feedIndentLevel = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
                    ProcessLine(line_feed, x, &mut currentScope, &mut manager);
                }
                Err(e) => {
                    panic!("Error happened: {:#?}", e)
                }
            }
		}
	}

}
