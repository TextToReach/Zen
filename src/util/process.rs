use crate::{
	features::tokenizer::{tokenize, InstructionEnum, TokenData, TokenTable}, library::{Methods::Throw, Types::{Object, ZenError}}, parsers::{Collection, Parsers::{self, ParserOutput}}, util::ScopeManager::ScopeManager
};
use chumsky::prelude::*;
use colored::Colorize;
use defer::defer;
use std::collections::{HashMap, HashSet};
use super::{ScopeManager::Scope, Util::generate_8_digit_id};

pub fn processLine(line_feed: Vec<TokenData>, input: (ParserOutput, InstructionEnum), current_scope: &mut usize, manager: &mut ScopeManager) {
    println!("Line output: {:#?}\n\n", input);
    
    let line_feed_tab_count = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
    let scope_depth = manager.get_depth(current_scope.clone());
    
    if line_feed_tab_count != scope_depth {
        if line_feed_tab_count > scope_depth {
            panic!("More. {} > {}", line_feed_tab_count, scope_depth)
        } else {
            let temp = current_scope.clone();
            *current_scope = manager.get_parent(current_scope.clone()).unwrap();
            manager.remove_scope(temp);
        }
    }
    
    println!("Scope: {}", current_scope);
    println!("--------------------------------------------------------------");
    
    if input.0.indent {
        let newScope = manager.create_scope(Some(current_scope.clone()), None);
        *current_scope = newScope;
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
    let mut manager = ScopeManager::new();
    let rootScope = 0; // Id of the root scope is 0. So this is just an alias for Scope:0
    let mut currentScope = rootScope;

    for line in input.iter_mut() {
		for chunk in line.split(";") {
			let line_feed = tokenize(line);
            if line_feed.is_empty() || line_feed.iter().any(|x| !x.isOk) { continue; }
            let clean_line_feed = line_feed.iter().filter(|x| x.token != TokenTable::Tab).cloned().collect::<Vec<_>>();

            match Collection::expression().parse(clean_line_feed.clone()) {
                Ok(x) => {
                    let feedIndentLevel = line_feed.count_from_start(|x| x.token == TokenTable::Tab);
                    println!("x: {:#?}", x);
                    // processLine(line_feed, x, &mut currentScope, &mut manager);
                }
                Err(e) => {
                    panic!("Error happened: {:#?}", e)
                }
            }
		}
	}
}
