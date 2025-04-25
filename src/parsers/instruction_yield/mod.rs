pub mod Input;
pub mod InstrYieldKit {
    use std::{cell::RefCell, rc::Rc};
    use chumsky::prelude::*;
    use crate::{library::{Environment::Environment, Methods::{Str, Throw}, Types::{Expression, InstructionEnum, InstructionYield, Object}}, Input};
    use super::Input;

    pub fn newline() -> impl Parser<char, char, Error = Simple<char>> {
        just('\n')
    }
    
    pub fn whitespace() -> impl Parser<char, Vec<char>, Error = Simple<char>> {
        filter(|c: &char| c.is_whitespace() && *c != '\n').repeated()
    }

    pub fn separator<'a>() -> impl Parser<char, &'a str, Error = Simple<char>> {
        just(",").padded()
    }

    pub fn evaluate(obj: InstructionEnum, currentScope: Rc<RefCell<Environment>>) -> Box<Object> {
        match obj {
            InstructionEnum::Input(args) => {
                Box::new(Object::from(Input!(format!("{}", args))))
            }
            _ => {
                Throw(Str("Invalid Instruction"), crate::library::Types::ZenError::TypeError, None, None);
                unreachable!()
            },
        }
    }

    pub fn parser<'a>(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Expression, Error = Simple<char>> + 'a> {
        Box::new(
            choice([
                Input::parser(currentScope.clone()),
            ]).map(move |x| {
                let val = evaluate(x.0, currentScope.clone());
                Expression::Value(val)
            })
        )
    }
}