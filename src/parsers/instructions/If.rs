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
            .then(
                newline()
                    .then_ignore(just("değilse ve"))
                    .then_ignore(whitespace())
                    .then( Expression::parser(currentScope.clone()).delimited_by(just("("), just(")")) )
                    .then_ignore(whitespace())
                    .then_ignore(just("ise"))
                    .then_ignore(newline())
                    .then(
                        just('\t')
                            .ignore_then(instr_parser.clone())
                            .separated_by(newline())
                            .at_least(1)
                    )
                    .repeated()
                    .map(|vec| {
                        vec.into_iter()
                            .map(|((_, condition), instructions)| IfBlockStructure {
                                condition,
                                onSuccess: instructions
                            })
                            .collect::<Vec<_>>()
                    })
            )
            .then(
                newline()
                    .then_ignore(just("değilse"))
                    .then_ignore(newline())
                    .then(
                        just('\t')
                            .ignore_then(instr_parser.clone())
                            .separated_by(newline())
                            .at_least(1)
                    )
                    .map(|(x, e)| Some(e))
                    .or_not()
                    .map(|opt| opt.flatten())
            )
            .then_ignore(newline().or_not())
            .map(
                |((if_block, elif_blocks), else_block)| {
                    Instruction(
                        InstructionEnum::If {
                            ifBlock: IfBlockStructure { condition: if_block.0, onSuccess: if_block.1 },
                            elifBlocks: elif_blocks,
                            elseBlock: else_block,
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
