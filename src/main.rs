mod library;
mod parsers;

use std::{fs::File, io::Read};

use chumsky::{prelude::*, primitive::Choice, Parser};
use library::{Methods::Throw, Types::{Instruction, Object}};
use parsers::instructions::{yazdir, Kit};
use clap::{Parser as ClapParser, Subcommand};

/// Ana CLI aracı
#[derive(ClapParser, Debug)]
#[command(author = "...", version = "0.0.1+ALPHA", about = "Zen CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Alt komutlar
#[derive(Subcommand, Debug)]
enum Commands {
    /// Dosya çalıştırma komutu
    Run {
        /// İşlenecek dosya adı
        file: String,

        /// Ayrıntılı çıktı göster
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

/* fn Lexer() -> Choice<[Box<dyn Parser<char, Object, Error = Simple<char>>>; 2], Simple<char>>{
    choice([
        Object::parser()
    ])
} */ 

fn process(AST: Instruction){
    let instruction = AST.0.as_str();
    let argobj = AST.1;
    let argins = AST.2;
    match instruction {
        "yazdır" => {
            PrintVec!(argobj);
        }
        "repeat" => {
            println!("shit hell");
            if let Object::Number(c) = argobj[0]{
                for i in 0..c.value.floor() as i64 { // left here - got stack overflow error
                    process(ins);
                }
            }
        }
        _ => {}
    }
}

fn run(file: String, verbose: bool) {
    let input = match File::open(file) {
        Ok(res) => {
            let mut buffr = String::new();
            let mut res = res;
            match res.read_to_string(&mut buffr) {
                Ok(_) => {}
                Err(_) => Throw("Dosya okunmaya çalışırken bir hatayla karşılaşıldı.".to_owned(), library::Types::ZenError::GeneralError, None, None),
            }
            buffr
        }
        Err(_) => {Throw("Dosya okunmaya çalışırken bir hatayla karşılaşıldı.".to_owned(), library::Types::ZenError::GeneralError, None, None); String::from("")}
    };

    match Kit::parser().repeated().parse(input.clone()) {
        Ok(results) => {
            println!("AST: {:#?}\n\n", results);
            for result in results {
                process(result);
            }
        },
        Err(errors) => {
            for error in errors {
                println!("Hata: {:?}", error);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, verbose } => {
            run(file, verbose);
        }
    }
}
