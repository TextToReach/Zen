mod library;
mod parsers;

use chumsky::error::Cheap;
use chumsky::prelude::*;
use library::Types::{Boolean, Number, Parsable, Text};
use parsers::instructions;

fn main() {
    let input = Input!("Enter something to parse: ");
    let lexer = choice((
        instructions::yazdir::parser(),
        Number::parser(),
        Text::parser(),
        Boolean::parser(),
    ));

    match lexer.parse(input) {
        Ok(result) => println!("Parse başarılı: {:#?}", result),
        Err(errors) => {
            for error in errors {
                println!("Hata: {:?}", error);
            }
        }
    }
}
