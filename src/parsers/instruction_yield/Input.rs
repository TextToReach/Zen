#![allow(non_snake_case, dead_code)]

use std::{cell::RefCell, rc::Rc};

use chumsky::{prelude::*, text::whitespace};
use crate::library::{Environment::Environment, Types::{Expression, Instruction, InstructionEnum, InstructionYield, Object, Parsable, Text, Variable}};

use crate::InstrKit::separator;

pub fn parser(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, InstructionYield, Error = Simple<char>>> {
    Box::new(just("sor")
        .then_ignore(whitespace())
        .then(
            Expression::parser(currentScope.clone())
        )
        .map(move |(ins, arg)| InstructionYield(
            InstructionEnum::Input(
                arg.evaluate(currentScope.clone())
            )
        ))
    )
}
