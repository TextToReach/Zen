#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;
use crate::library::Types::{Instruction, Object, InstructionEnum};

use super::Kit::whitespace;

pub fn parser() -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        text::ident()
            .then_ignore(whitespace())
            .then_ignore(just('='))
            .then_ignore(whitespace())
            .then(
                Object::parser().or(text::ident().map(|e| Object::Variable(e)))
            )
            .map(|(name, value)| Instruction(InstructionEnum::VariableDeclaration(name, value))),
    )
}
