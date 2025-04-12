// pest = "2.5.6" crate'ini Cargo.toml dosyana eklemeyi unutma gardaş!

#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"] // Bu dosyayı aşağıda tanımlıyoruz
struct MyParser;

fn main() {
    let inputs = vec!["42", "3.14", "-15", "0.001"];
    
    for input in inputs {
        match MyParser::parse(Rule::number, input) {
            Ok(parsed) => println!("Girdi: {} -> Geçerli! Parse Edildi!", input),
            Err(e) => println!("Hata: {:?} -> {}", e, input),
        }
    }
}
