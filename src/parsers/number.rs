use chumsky::prelude::*;

use crate::library::Types::{New, Number, ZenType};

pub fn parser() -> impl Parser<char, ZenType, Error = Simple<char>> { 
    just("-")
        .or_not()
        .then(text::int::<_, Simple<char>>(10))
        .then(just('.').ignore_then(text::digits(10)).or_not())
        .map(|((negative, int), frac)| {
            ZenType::Number(Number::new(format!("{}{}.{}", negative.unwrap_or("+"), int, frac.unwrap_or("0".to_owned())).parse::<f64>().unwrap()))
        })
        .padded()
}
