use chumsky::prelude::*;
use chumsky::text::take_until;

#[derive(Debug)]
enum Token {
    Keyword(String),
    StringLiteral(String),
    Eof,
}

fn parser() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let keyword = just("yazdır").map(|s: &str| Token::Keyword(s.to_string()));
    let string_literal = just('"')
        .ignore_then(take_until(just('"')))
        .map(|s: String| Token::StringLiteral(s));
    let eof = end().map(|_| Token::Eof);

    keyword.or(string_literal).repeated().then_ignore(eof)
}

fn main() {
    let code = r#"yazdır "Merhaba, Dünya!""#;
    let tokens = parser().parse(code).unwrap();

    for token in tokens {
        match token {
            Token::Keyword(ref keyword) => {
                if keyword == "yazdır" {
                    println!("Keyword: {}", keyword);
                } else {
                    panic!("Unknown keyword: {}", keyword);
                }
            }
            Token::StringLiteral(ref string) => {
                println!("StringLiteral: {}", string);
            }
            Token::Eof => break,
        }
    }
}