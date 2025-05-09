#![allow(non_snake_case, dead_code)]

use std::process::exit;

use super::Types::{Severity, ZenError};
use colored::Colorize;

pub fn Str(val: &str) -> String {
	String::from(val)
}

#[macro_export]
macro_rules! Print {
    () => {
        println!();
    };

    ($($arg:expr),*) => {
        let mut acc: Vec<String> = Vec::new();

        $(
            acc.push(format!("{}", $arg));
        )*

        println!("{}", acc.join(" "));
    };
}

#[macro_export]
macro_rules! PrintVec {
    () => {
        println!();
    };

    ($($arg:expr),*) => {
        let mut acc: Vec<String> = Vec::new();

        $(
            for x in $arg {
                acc.push(format!("{}", x));
            }
        )*

        println!("{}", acc.join(" "));
    };
}

#[macro_export]
macro_rules! DebugVec {
    () => {
        println!();
    };

    ($($arg:expr),*) => {
        let mut acc: Vec<String> = Vec::new();

        $(
            for x in $arg {
                acc.push(format!("{:?}", x));
            }
        )*

        println!("{}", acc.join(" "));
    };
}

#[macro_export]
macro_rules! PrettyDebugVec {
    () => {
        println!();
    };

    ($($arg:expr),*) => {
        let mut acc: Vec<String> = Vec::new();

        $(
            for x in $arg {
                acc.push(format!("{:#?}", x));
            }
        )*

        println!("{}", acc.join(" "));
    };
}

#[macro_export]
macro_rules! Debug {
    () => {
        println!();
    };

    ($($arg:expr),*) => {
        let mut acc: Vec<String> = Vec::new();

        $(
            acc.push(format!("{:?}", $arg));
        )*

        println!("{}", acc.join(" "));
    };
}

#[macro_export]
macro_rules! PrettyDebug {
    () => {
        println!();
    };

    ($($arg:expr),*) => {
        let mut acc: Vec<String> = Vec::new();

        $(
            acc.push(format!("{:#?}", $arg));
        )*

        println!("{}", acc.join(" "));
    };
}

#[macro_export]
macro_rules! ReadFile {
	($path:expr) => {{
		use std::fs;
		match fs::read_to_string($path) {
			Ok(content) => content,
			Err(e) => panic!("Dosya okunurken bir hata oluştu! Hata: {}", e),
		}
	}};
}

#[macro_export]
macro_rules! Input {
	() => {{
		use std::io::{self, Write};
		let mut input = String::new();
		print!("");
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).unwrap();
		input.trim().to_string()
	}};

	($prompt:expr) => {{
		use std::io::{self, Write};
		let mut input = String::new();
		print!("{}", $prompt);
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).unwrap();
		input.trim().to_string()
	}};

	(<$t:ty>) => {{
		use std::io::{self, Write};
		let mut input = String::new();
		print!("Enter input: "); // Optional prompt
		io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
		io::stdin().read_line(&mut input).unwrap();
		input
			.trim()
			.parse::<$t>()
			.unwrap_or_else(|_| panic!("Failed to parse input into the specified type."))
	}};

	// Case when a prompt is passed, with a specified type
	($prompt:expr, <$t:ty>) => {{
		use std::io::{self, Write};
		let mut input = String::new();
		print!("{}", $prompt); // Print the custom prompt
		io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
		io::stdin().read_line(&mut input).unwrap();
		input
			.trim()
			.parse::<$t>()
			.unwrap_or_else(|_| panic!("Failed to parse input into the specified type."))
	}};
}

fn ColorStringBySeverity<T: ToString>(severity: &Severity, text: T) -> String {
	let txt = text.to_string();
	match severity {
		Severity::Low => txt.blue().to_string(),
		Severity::Medium => txt.yellow().to_string(),
		Severity::High => txt.red().to_string(),
	}
}

#[derive(Clone)]
pub struct FileAndLineInformation(pub u16, pub String);
pub fn Throw<T: AsRef<str>>(
	desc: T,
	errortype: ZenError,
	file_and_line: Option<FileAndLineInformation>,
	unreachable_error: Option<bool>,
	severity: Severity,
) {
	let description = desc.as_ref();
	println!(
		"\n{err1}\n[{line}] ve [{file}] noktasında bir hatayla karşılaşıldı.\n\n...\n\n{err2}: {desc}{extra}",
		extra = if unreachable_error.unwrap_or(false) {
			"\nBu hata normal kullanımda karşılaşılamaması gereken bir hatadır. Hata yüksek ihtimalle kullandığınız Zen sürümünden kaynaklıdır.\nLütfen Zen yüklemenizi güncelleyiniz. Eğer hata devam ederse lütfen giderilebilmesi için geliştiricilere domain@mail.com adresinden iletiniz."
		} else {
			""
		},
		err1 = ColorStringBySeverity(
			&severity,
			format!("Bir hata ile karşılaşıldı. ({})", format!("{:?}", errortype).underline().italic())
		),
		err2 = ColorStringBySeverity(&severity, format!("{:?}", errortype).underline().italic()),
		file = format!(
			"Satır: {}",
			if let Some(FileAndLine) = file_and_line.clone() {
				FileAndLine.0.to_string()
			} else {
				"Bilinmiyor".to_string()
			}
		)
		.green(),
		line = format!(
			"Dosya: {}",
			if let Some(FileAndLine) = file_and_line.clone() {
				FileAndLine.1.to_string()
			} else {
				"Bilinmiyor".to_string()
			}
		)
		.cyan(),
		desc = description
	);

	if severity == Severity::High {
		exit(1)
	};
}
