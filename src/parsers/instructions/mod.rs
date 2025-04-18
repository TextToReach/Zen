#![allow(non_snake_case, dead_code)]

pub mod forloop1;
pub mod yazdir;

/// Instructions with a different name
pub mod Kit {
    use crate::library::Types::{Instruction, Object};
    use chumsky::prelude::*;

    use super::{forloop1, yazdir};

    #[derive(Debug, Clone, PartialEq)]
    pub enum InstructionEnum {
        Yazdır,
        Forloop1,
    }

    pub fn parser<'a>() -> Box<dyn Parser<char, Instruction, Error = Simple<char>> + 'a> {
        Box::new(recursive(|instr_parser| {
            choice([
                    yazdir::parser(),
                    forloop1::parser(instr_parser)
            ])
        }))
    }
}
