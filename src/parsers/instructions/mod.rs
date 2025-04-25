#![allow(non_snake_case, dead_code)]

pub mod Repeat;
pub mod Print;
pub mod Variable;

/// Instructions with a different name
pub mod InstrKit {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::library::Environment::Environment;
    use crate::library::Types::{Instruction, InstructionEnum, Object};
    use chumsky::{prelude::*, primitive::OrderedContainer};
    use chumsky::error::Simple;

    use super::{Print, Repeat, Variable};

    pub fn newline() -> impl Parser<char, char, Error = Simple<char>> {
        just('\n')
    }
    
    pub fn whitespace() -> Rc<impl Parser<char, Vec<char>, Error = Simple<char>>> {
        Rc::new(filter(|c: &char| c.is_whitespace() && *c != '\n').repeated())
    }

    pub fn separator<'a>() -> impl Parser<char, &'a str, Error = Simple<char>> {
        just(",").padded()
    }

    /// And this is the instruction parser
    pub fn parser<'a>(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Vec<Instruction>, Error = Simple<char>> + 'a> {
        Box::new(
            recursive(|instr_parser| {
                choice([
                    Print::parser(currentScope.clone()),
                    Repeat::parser(instr_parser),
                    Variable::parser(currentScope.clone()),
                    Box::new(whitespace().to(Instruction(InstructionEnum::NoOp))),
                ])
            }).separated_by(newline())
        )
    }
}
