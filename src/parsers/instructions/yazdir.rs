use chumsky::prelude::*;

use crate::{library::{
    Array::Array,
    Types::{New, Text, ZenType},
}, parsers::text};

pub fn parser() -> impl Parser<char, ZenType, Error = Simple<char>> {
    just("yazdır")
        .padded()
        .ignore_then(text::parser().separated_by(just(' ')).at_least(1))
        .map(ZenType::from)


    // just("yazdır")
    //     .padded()
    //     .ignore_then(
    //          crate::string::parser().separated_by(just(' ')).at_least(1)
    //      ).map(ZenType::from)
}
