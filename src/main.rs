mod library;
mod parsers;

use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use chumsky::{prelude::*, primitive::Choice, Parser};
use library::{Environment::Environment, Methods::Throw, Types::{Instruction, InstructionEnum, Object, ZenError}};
use parsers::instructions::{Print, Kit};
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

fn process(AST: Instruction, currentScope: Rc<RefCell<Environment>>){
    let InstructionVariant = AST.0;

    match InstructionVariant {
        InstructionEnum::Yazdır(Objects) => {
            let resolved_objects: Vec<_> = Objects.into_iter()
                .map(|obj| {
                    match obj {
                    Object::Variable(var_name) => currentScope.borrow().get(&var_name).unwrap_or_else(|| {
                        Throw(
                            format!("{} adında bir değişken tanımlı değil.", var_name),
                            ZenError::GeneralError,
                            None,
                            None
                        );
                        Object::Nil
                    }),
                        _ => obj,
                    }
                })
                .collect();
            
            PrintVec!(resolved_objects);
        }
        InstructionEnum::Forloop1(RepeatCount, Instructions) => {
            let innerScope = Rc::new(RefCell::new(Environment::with_parent(currentScope.clone())));

            for index in 0..RepeatCount {
                for instr in Instructions.iter() {
                    process(instr.clone(), innerScope.clone());
                }
            }
        }
        InstructionEnum::VariableDeclaration(Name, Value) => {
            if let Object::Variable(var_name) = Value { // Continue from here
                Throw(
                    format!("{} adında bir değişken tanımlı değil.", var_name),
                    ZenError::GeneralError,
                    None,
                    None
                );
            }
            currentScope.borrow_mut().set(&Name, Value);
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
            let mut ROOT_SCOPE = Rc::new(RefCell::new(Environment::new()));

            for result in results {
                process(result, ROOT_SCOPE.clone());
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
