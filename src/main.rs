mod library;
mod parsers;

use chumsky::{prelude::*, primitive::Choice, Parser};
use library::Types::Object;
use parsers::instructions::Kit;
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

fn Lexer() -> Choice<[Box<dyn Parser<char, Object, Error = Simple<char>>>; 2], Simple<char>>{
    choice([
        Object::parser(),
        Kit::parser()
    ])
}

fn run(file: String, verbose: bool) {
    let input = ReadFile!(file);

    match Lexer().parse(input) {
        Ok(result) => println!("Parse başarılı: {:#?}", result),
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
