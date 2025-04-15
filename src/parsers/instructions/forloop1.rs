#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;

use crate::library::Types::{Instruction, Number, Object, Parsable};

use super::Kit;

// n defa tekrarla

pub fn whitespace() -> impl Parser<char, char, Error = Simple<char>> {
    filter(|c: &char| c.is_whitespace() && *c != '\n')
}

pub fn parser() -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        Number::parser() // 10
            .then_ignore(whitespace()) // space
            .then_ignore(just("defa").or(just("kere")))
            .then_ignore(whitespace())
            .then_ignore(just("tekrarla"))
            .then(just('\t').ignore_then(Kit::parser()).repeated().at_least(1))
            .map(|(a, b)| {
                Instruction("repeat".to_owned(), vec![a], b)
            })
    )
}
