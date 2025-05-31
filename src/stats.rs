// cloc --include-lang=Rust --csv --quiet . | awk -F, '/Rust/ {print $5}'

use std::process::Command;

pub fn stats(input: String) -> String {
	input.replace(
		"{{ RUST_LINES }}",
		String::from_utf8_lossy(&Command::new("bash").arg("./scripts/rust_lines.sh").output().unwrap().stdout)
			.to_string()
			.trim(),
	)
}
