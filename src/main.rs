#![allow(non_snake_case)]

mod library;
mod parsers;
mod features;

use std::{cell::RefCell, fs::File, io::Read, ops::Deref, rc::Rc};

use chumsky::{Parser, prelude::*, primitive::Choice};
use clap::{Parser as ClapParser, Subcommand};
use colored::Colorize;
use features::preprocessor;
use library::{
    Environment::Environment,
    Methods::Throw,
    Types::{Comparison, Expression, Instruction, InstructionEnum, Object, ZenError},
};
use parsers::instructions::{InstrKit, Print};

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

        /// AST çıktısını göster
        #[arg(long, default_value_t = false)]
        printast: bool,

        #[arg(long, default_value_t = false)]
        printpreprocessoutput: bool,
    },
}

/// Resolve the object to its final value (gets the value of variables)
fn resolve(obj: Object, currentScope: Rc<RefCell<Environment>>) -> Object {
    match obj {
        Object::Expression(expr) => {
            expr.evaluate(currentScope.clone())
        }
        _ => {obj},
    }
}

fn process(AST: Instruction, currentScope: Rc<RefCell<Environment>>, verbose: bool) {
    let InstructionVariant = AST.0;

    match InstructionVariant {
        InstructionEnum::Print(Objects) => {
            let resolved_objects: Vec<_> = Objects.clone()
                .into_iter()
                .map(|obj| resolve(obj.evaluate(currentScope.clone()), currentScope.clone()))
                .collect();
            

            PrintVec!(&resolved_objects);
            if verbose { PrettyDebugVec!(resolved_objects); }
        }
        InstructionEnum::Forloop1(RepeatCount, Instructions) => {
            let innerScope = Rc::new(RefCell::new(Environment::with_parent(currentScope.clone())));

            for index in 0..RepeatCount {
                for instr in Instructions.iter() {
                    process(instr.clone(), innerScope.clone(), verbose);
                }
            }
        }
        InstructionEnum::VariableDeclaration(Name, Value) => {
            let value = Value.evaluate(currentScope.clone());
            
            currentScope.borrow_mut().set(&Name, value);
        }
        InstructionEnum::If { ifBlock, elifBlocks, elseBlock } => {
            let ifCondition = ifBlock.condition.evaluate(currentScope.clone()).isTruthy();

            if ifCondition {
                let innerScope = Rc::new(RefCell::new(Environment::with_parent(currentScope.clone())));
                for instr in ifBlock.onSuccess.iter() {
                    process(instr.clone(), innerScope.clone(), verbose);
                }
            } else if let Some(elifBlocks) = elifBlocks {

                for elifBlock in elifBlocks {
                    let elifCondition = elifBlock.condition.evaluate(currentScope.clone()).isTruthy();
                    if elifCondition {
                        let innerScope = Rc::new(RefCell::new(Environment::with_parent(currentScope.clone())));
                        for instr in elifBlock.onSuccess.iter() {
                            process(instr.clone(), innerScope.clone(), verbose);
                        }
                        break;
                    }
                }
                
            } else if let Some(elseBlock) = elseBlock {
                let innerScope = Rc::new(RefCell::new(Environment::with_parent(currentScope.clone())));
                for instr in elseBlock.iter() {
                    process(instr.clone(), innerScope.clone(), verbose);
                }

            }
        }
        _ => {}
    }
}

fn run(file: String, verbose: bool, printAst: bool, printPreprocessOutput: bool) {
    let mut input = match File::open(file) {
        Ok(res) => {
            let mut buffr = String::new();
            let mut res = res;
            match res.read_to_string(&mut buffr) {
                Ok(_) => {}
                Err(_) => Throw(
                    "Dosya okunmaya çalışırken bir hatayla karşılaşıldı.".to_owned(),
                    library::Types::ZenError::GeneralError,
                    None,
                    None,
                ),
            }
            buffr
        }
        Err(_) => {
            Throw(
                "Dosya okunmaya çalışırken bir hatayla karşılaşıldı.".to_owned(),
                library::Types::ZenError::GeneralError,
                None,
                None,
            );
            String::from("")
        }
    };
    let ROOT_SCOPE = Rc::new(RefCell::new(Environment::new()));
    preprocessor::index(&mut input);
    if printPreprocessOutput { println!("Buffer:\n{}", input.red()); }
    
    match InstrKit::parser(ROOT_SCOPE.clone()).parse(input.clone()) {
        Ok(results) => {
            if printAst { println!("AST: {:#?}\n\n", results); }
            

            for result in results {
                process(result, ROOT_SCOPE.clone(), verbose);
            }
        }
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
        Commands::Run { file, verbose, printast, printpreprocessoutput } => {
            run(file, verbose, printast, printpreprocessoutput);
        }
    }
}
