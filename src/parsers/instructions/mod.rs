#![allow(non_snake_case, dead_code)]

pub mod ForLoop1;
pub mod Print;
pub mod Variable;

/// Instructions with a different name
pub mod Kit {
    use crate::library::Types::{Instruction, InstructionEnum, Object};
    use chumsky::prelude::*;
    use chumsky::error::Simple;

    use super::{ForLoop1, Print, Variable};

    pub fn newline() -> impl Parser<char, char, Error = Simple<char>> {
        just('\n')
    }
    
    pub fn whitespace() -> impl Parser<char, Vec<char>, Error = Simple<char>> {
        filter(|c: &char| c.is_whitespace() && *c != '\n').repeated()
    }
    pub fn comment() -> impl Parser<char, (), Error = Simple<char>> {
        just('#').ignore_then(take_until(newline())).ignored()
    }

    pub fn separator<'a>() -> impl Parser<char, &'a str, Error = Simple<char>> {
        just(",").padded()
    }

    pub fn parser<'a>() -> Box<dyn Parser<char, Vec<Instruction>, Error = Simple<char>> + 'a> {
        Box::new(
            recursive(|instr_parser| {
                choice([
                    Print::parser(),
                    ForLoop1::parser(instr_parser),
                    Variable::parser(),
                    Box::new(whitespace().ignored().to(Instruction(InstructionEnum::NoOp))),
                ])
            }).separated_by(newline())
        )
    }
}
