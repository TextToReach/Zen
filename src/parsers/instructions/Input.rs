#![allow(non_snake_case, dead_code)]

use std::{cell::RefCell, rc::Rc};

use chumsky::prelude::*;
use crate::library::{Environment::Environment, Types::{Expression, Instruction, InstructionEnum, InstructionYield, Object, Parsable, Text, Variable}};

use super::Kit::separator;

pub fn parser(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Instruction, Error = Simple<char>>> {
    Box::new(just("girdi")
        .padded()
        .then(
            Text::parser()
        )
        .map(|(ins, arg)| Instruction(InstructionEnum::Input(Expression::from(arg))))
    )
}
