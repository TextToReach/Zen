#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;
use crate::library::Types::{Instruction, Object};
use super::Kit::InstructionEnum;

pub fn parser() -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(just("yazdır")
        .padded()
        .then(Object::parser().separated_by(just(' ')).at_least(1))
        .map(|(ins, arg)| Instruction(InstructionEnum::Yazdır, arg, vec![]))
        .then_ignore(just('\n'))
    )
}
