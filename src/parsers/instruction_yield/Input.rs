#![allow(non_snake_case, dead_code)]

use std::{cell::RefCell, rc::Rc};

use chumsky::{prelude::*, text::whitespace};
use crate::library::{Environment::Environment, Types::{BaseTypes, Expression, Instruction, InstructionEnum, InstructionYield, Object, Parsable, Text, Variable}};

use crate::InstrKit::separator;

pub fn parser(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, InstructionYield, Error = Simple<char>>> {
    let ts = BaseTypes::values;
    
    Box::new(just("sor")
        .then(
            just(ts[0]).or(just(ts[1])).or(just(ts[2])).or(just(ts[3])).delimited_by(just("("), just(")")).or_not().map(|e|{
                match e {
                    Some(t) => t,
                    None => BaseTypes::values[0],
                }
            })
        )
        .then_ignore(whitespace())
        .then(
            Expression::parser(currentScope.clone())
        )
        .map(move |((ins, t), arg)| InstructionYield(
            InstructionEnum::Input(
                arg.evaluate(currentScope.clone()),
                BaseTypes::from_str(&t),
            )
        ))
    )
}
