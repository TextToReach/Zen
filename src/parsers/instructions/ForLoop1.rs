#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;

use crate::library::Types::{Instruction, Number, Object, Parsable, InstructionEnum};

use super::Kit::{self, newline, whitespace};

// n defa tekrarla

pub fn parser(
    instr_parser: impl Parser<char, Instruction, Error = Simple<char>> + Clone + 'static,
) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        Number::parser() // 10
            .then_ignore(whitespace()) // space
            .then_ignore(just("defa").or(just("kere")).or(just("kez")))
            .then_ignore(whitespace())
            .then_ignore(just("tekrarla"))
            .then_ignore(just("\n"))
            .then(
                just('\t').ignore_then(instr_parser.clone()).separated_by(newline())
            ).map(|(a, b)| {
                Instruction(InstructionEnum::Forloop1(a.asNumber().value.floor() as i64, b))
            })
    )
}
