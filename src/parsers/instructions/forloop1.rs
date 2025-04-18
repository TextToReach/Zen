#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;

use crate::{
    library::Types::{Instruction, Number, Object, Parsable},
    parsers::instructions::Kit::InstructionEnum,
};

use super::Kit;

// n defa tekrarla

pub fn whitespace() -> impl Parser<char, char, Error = Simple<char>> {
    filter(|c: &char| c.is_whitespace() && *c != '\n')
}

pub fn parser(
    instr_parser: impl Parser<char, Instruction, Error = Simple<char>> + Clone + 'static,
) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        Number::parser() // 10
            .then_ignore(whitespace()) // space
            .then_ignore(just("defa").or(just("kere")))
            .then_ignore(whitespace())
            .then_ignore(just("tekrarla"))
            .then_ignore(just("\n"))
            .then(
                just('\t').ignore_then(instr_parser.clone()).separated_by(just("\n"))
            ).map(|(a, b)| {
                println!("Ins: {:?}", b);
                Instruction(InstructionEnum::Forloop1, vec![a], b)
            })
    )
}
