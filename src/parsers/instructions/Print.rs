#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;
use crate::library::Types::{Instruction, Object, InstructionEnum};

use super::Kit::separator;

pub fn parser() -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(just("yazdır")
        .padded()
        .then(
            Object::parser().or(text::ident().map(|e| Object::Variable(e))).separated_by(separator()).at_least(1)
        )
        .map(|(ins, arg)| Instruction(InstructionEnum::Yazdır(arg)))
    )
}
