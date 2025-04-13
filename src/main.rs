mod library;
mod parsers;

use chumsky::error::Cheap;
use chumsky::prelude::*;
use chumsky::Parser;
use parsers::{instructions, number, string};

fn main() {
    let input = Input!("Enter something to parse: ");
    let lexer = choice((
        instructions::yazdir::parser(),
        number::parser(),
        string::parser()
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
