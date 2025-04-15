#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;

use crate::library::Types::Object;

pub fn parser() -> Box<impl Parser<char, Object, Error = Simple<char>>> {
    Box::new(just("yazdır")
        .padded()
        .ignore_then(Object::parser().separated_by(just(' ')).at_least(1))
        .map(Object::from))


    // just("yazdır")
    //     .padded()
    //     .ignore_then(
    //          crate::string::parser().separated_by(just(' ')).at_least(1)
    //      ).map(ZenType::from)
}
