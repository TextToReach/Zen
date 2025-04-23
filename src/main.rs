#![allow(non_snake_case)]

mod library;
mod parsers;

use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use chumsky::{Parser, prelude::*, primitive::Choice};
use clap::{Parser as ClapParser, Subcommand};
use library::{
    Environment::Environment,
    Methods::Throw,
    Types::{Arithmetic, Instruction, InstructionEnum, Object, ZenError},
};
use parsers::instructions::{Kit, Print};

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

pub fn ProcessArithmeticTree(obj: Box<Arithmetic>, currentScope: Rc<RefCell<Environment>>) -> Object {
    match *obj {
        Arithmetic::Add(lhs, rhs) => {
            let left = ProcessArithmeticTree(lhs, currentScope.clone());
            let right = ProcessArithmeticTree(rhs, currentScope.clone());
            left + right
        }
        Arithmetic::Sub(lhs, rhs) => {
            let left = ProcessArithmeticTree(lhs, currentScope.clone());
            let right = ProcessArithmeticTree(rhs, currentScope.clone());
            left - right
        }
        Arithmetic::Mul(lhs, rhs) => {
            let left = ProcessArithmeticTree(lhs, currentScope.clone());
            let right = ProcessArithmeticTree(rhs, currentScope.clone());
            left * right
        }
        Arithmetic::Div(lhs, rhs) => {
            let left = ProcessArithmeticTree(lhs, currentScope.clone());
            let right = ProcessArithmeticTree(rhs, currentScope.clone());
            left / right
        }
        Arithmetic::Mod(lhs, rhs) => {
            let left = ProcessArithmeticTree(lhs, currentScope.clone());
            let right = ProcessArithmeticTree(rhs, currentScope.clone());
            left % right
        }
        Arithmetic::Value(val) => {
            match *val {
                Object::Variable(ref var_name) => {
                    currentScope.borrow().get(var_name).unwrap_or_else(|| {
                        Throw( format!("{} adında bir değişken tanımlı değil.", var_name), ZenError::GeneralError, None, None, );
                        Object::Nil
                    })
                }
                ref other => other.clone(),
            }
        }
    }
}

fn resolve(obj: Object, currentScope: Rc<RefCell<Environment>>) -> Object {
    match obj {        
        Object::Variable(var_name) => {
            currentScope.borrow().get(&var_name).unwrap_or_else(|| {
                Throw( format!("{} adında bir değişken tanımlı değil.", var_name), ZenError::GeneralError, None, None, );
                Object::Nil
            })
        }
        Object::ArithmeticExpression(expr) => {
            ProcessArithmeticTree(Box::new(expr), currentScope.clone())
        }
        _ => obj,
    }
}

fn process(AST: Instruction, currentScope: Rc<RefCell<Environment>>, verbose: bool) {
    let InstructionVariant = AST.0;

    match InstructionVariant {
        InstructionEnum::Yazdır(Objects) => {
            let resolved_objects: Vec<_> = Objects.clone()
                .into_iter()
                .map(|obj| resolve(obj, currentScope.clone()))
                .collect();

            PrintVec!(resolved_objects);
            if verbose { PrettyDebugVec!(Objects); }
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
            if let Object::Variable(var_name) = &Value {
                // Continue from here
                Throw( format!("{} adında bir değişken tanımlı değil.", var_name), ZenError::GeneralError, None, None, );
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
    
    match Kit::parser(ROOT_SCOPE.clone()).parse(input.clone()) {
        Ok(results) => {
            if verbose { println!("AST: {:#?}\n\n", results); }
            

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
        Commands::Run { file, verbose } => {
            run(file, verbose);
        }
    }
}
