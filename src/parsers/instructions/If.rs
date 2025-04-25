#![allow(non_snake_case, dead_code)]

use std::{cell::RefCell, rc::Rc};

use chumsky::{prelude::*, text::whitespace};
use crate::library::{Environment::Environment, Types::{Expression, IfBlockStructure, Instruction, InstructionEnum, Object, Parsable, Variable}};

use super::InstrKit::{newline, separator};

pub fn ifParser<'a>(
    currentScope: Rc<RefCell<Environment>>,
    instr_parser: impl Parser<char, Instruction, Error = Simple<char>> + Clone + 'a,
) -> Box<dyn Parser<char, Instruction, Error = Simple<char>> + 'a> {
    Box::new(
        just("eğer")
            .then_ignore( whitespace() )
            .then( Expression::parser(currentScope.clone()).delimited_by(just("("), just(")")) )
            .map(|(_, e)| e)
            .then_ignore( whitespace() )
            .then_ignore( just("ise") )
            .then_ignore( newline() )
            .then(
                just('\t').ignore_then(instr_parser.clone()).separated_by(newline()).at_least(1)
            )
            .map(|(condition, onSuccess)| (condition, onSuccess))
            .then_ignore(newline())
            .then(
                just("değilse").then_ignore(newline())
                    .then( just('\t').ignore_then(instr_parser.clone()).separated_by(newline()).at_least(1) )
                    .or_not()
            )
            
            .map(
                |((ifCondition, ifOnSuccess), elseBlock)| 
                {
                    Instruction(
                        InstructionEnum::If {
                            ifBlock: IfBlockStructure { condition: ifCondition, onSuccess: ifOnSuccess },
                            elifBlocks: None,
                            elseBlock: elseBlock.map(|(_, block)| block),
                        }
                    )
                }
            )
    )
}

pub fn parser<'a>(
    currentScope: Rc<RefCell<Environment>>, 
    instr_parser: impl Parser<char, Instruction, Error = Simple<char>> + Clone + 'a,
) -> Box<dyn Parser<char, Instruction, Error = Simple<char>> + 'a> {
    Box::new(
        choice([
            ifParser(currentScope.clone(), instr_parser.clone()),
        ])
    )
}
