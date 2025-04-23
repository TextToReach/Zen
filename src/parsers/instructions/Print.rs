#![allow(non_snake_case, dead_code)]

use std::{cell::RefCell, rc::Rc};

use chumsky::prelude::*;
use crate::library::{Environment::Environment, Types::{Instruction, InstructionEnum, Object, Parsable, Variable}};

use super::Kit::separator;

pub fn parser(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(just("yazdır")
        .padded()
        .then(
            Object::parser(currentScope).or(Variable::parser()).separated_by(separator()).at_least(1)
        )
        .map(|(ins, arg)| Instruction(InstructionEnum::Yazdır(arg)))
    )
}
