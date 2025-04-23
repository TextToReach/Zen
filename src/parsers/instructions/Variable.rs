#![allow(non_snake_case, dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

use chumsky::prelude::*;
use crate::library::{Environment::Environment, Types::{Instruction, InstructionEnum, Object, Parsable, Variable}};

use super::Kit::whitespace;

pub fn parser(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        text::ident()
            .then_ignore(whitespace())
            .then_ignore(just('='))
            .then_ignore(whitespace())
            .then(
                Object::parser(currentScope).or(Variable::parser())
            )
            .then_ignore(whitespace())
            .map(|(name, value)| Instruction(InstructionEnum::VariableDeclaration(name, value))),
    )
}
