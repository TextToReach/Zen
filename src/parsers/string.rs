use chumsky::prelude::*;

use crate::library::Types::{New, Text, ZenType};

pub fn parser() -> impl Parser<char, ZenType, Error = Simple<char>> {
        let single_quoted = just('\'') // Tek tırnakla başla
        .ignore_then(filter(|c| *c != '\'').repeated()) // Tek tırnak bitene kadar karakterleri al
        .then_ignore(just('\'')) // Tek tırnakla bitir
        .collect::<String>(); // Karakterleri string'e çevir

    let double_quoted = just('"') // Çift tırnakla başla
        .ignore_then(filter(|c| *c != '"').repeated()) // Çift tırnak bitene kadar karakterleri al
        .then_ignore(just('"')) // Çift tırnakla bitir
        .collect::<String>(); // Karakterleri string'e çevir

    single_quoted
        .or(double_quoted)
        .map(|e| ZenType::Text(Text::new(e)))
}
