mod library;
mod parsers;

use std::{fs::File, io::Read};

use chumsky::{prelude::*, primitive::Choice, Parser};
use library::{Methods::Throw, Types::{Instruction, Object}};
use parsers::instructions::{yazdir, Kit::{self, InstructionEnum}};
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
    let InstructionVariant = AST.0;
    let ArgumentObjects = AST.1;
    let ArgumentInstructions = AST.2;
    
    match InstructionVariant {
        InstructionEnum::Yazdır => {
            PrintVec!(ArgumentObjects);
        }
        InstructionEnum::Forloop1 => {
            if let Object::Number(RepeatCount) = ArgumentObjects.clone().first().unwrap(){
                for index in 0..RepeatCount.value.floor() as i64 {
                    process(ArgumentInstructions.get(index as usize).unwrap().clone());
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

    match Kit::parser().parse(input.clone()) {
        Ok(results) => {
            println!("AST: {:#?}\n\n", results);
            process(results);
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
