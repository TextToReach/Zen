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
        print!("Enter input: ");  // Optional prompt
        io::stdout().flush().unwrap();  // Ensure the prompt is printed immediately
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap_or_else(|_| {
            panic!("Failed to parse input into the specified type.")
        })
    }};
    
    // Case when a prompt is passed, with a specified type
    ($prompt:expr, <$t:ty>) => {{
        use std::io::{self, Write};
        let mut input = String::new();
        print!("{}", $prompt);  // Print the custom prompt
        io::stdout().flush().unwrap();  // Ensure the prompt is printed immediately
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<$t>().unwrap_or_else(|_| {
            panic!("Failed to parse input into the specified type.")
        })
    }};
}
