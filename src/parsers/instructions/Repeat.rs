#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;

use crate::library::Types::{Instruction, Number, Object, Parsable, InstructionEnum};

use super::InstrKit::{self, newline, whitespace};

// n defa tekrarla

pub fn parser(
    instr_parser: impl Parser<char, Instruction, Error = Simple<char>> + Clone + 'static,
) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(
        Number::parser() // 10
            .then_ignore( just("defa").or(just("kere")).or(just("kez")).padded_by(whitespace()) ) // defa/kere/kez
            .then_ignore( just("tekrarla").or(just("tekrar et")).padded_by(whitespace()) ) // tekrarla/tekrar et
            .then_ignore(newline()) // \n
            .then( just('\t').ignore_then(instr_parser.clone().padded_by(whitespace())).separated_by(newline()) )
            .map(|(count, lines)| {
                if lines.is_empty() {
                    println!("Warning: No instructions parsed. Ensure the input matches the expected format.");
                } else {
                    println!("Forloop: {:?}", lines);
                }
                Instruction(InstructionEnum::Forloop1(count.asNumber().value.floor() as i64, lines))
            })
    )
}
