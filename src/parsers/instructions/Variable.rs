#![allow(non_snake_case, dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

use chumsky::prelude::*;
use crate::library::{Environment::Environment, Types::{Expression, Instruction, InstructionEnum, InstructionYield, Object, Parsable, Variable}};

use super::InstrKit::whitespace;
use super::super::instruction_yield::InstrYieldKit;

pub fn parser(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        text::ident()
            .then_ignore(just('=').padded_by(whitespace()))
            .then(
                InstrYieldKit::parser(currentScope.clone()).or(
                    Expression::parser(currentScope.clone())
                ).padded_by(whitespace())
            )
            .map(|(name, value)| Instruction(InstructionEnum::VariableDeclaration(name, value))),
    )
}
