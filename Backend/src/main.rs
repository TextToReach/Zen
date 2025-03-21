macro_rules! print {
    ($($arg:expr),*) => {
        {
            let mut result = String::new();

            // Iterate over all arguments and append each to the result string
            let mut first = true; // Flag to check if it's the first argument
            $(
                if !first {
                    result.push(' '); // Add a space before each argument, except the first
                }
                result.push_str(&$arg.to_string());
                first = false; // After the first argument, we set the flag to false
            )*

            // Print the concatenated result
            println!("{}", result);
        }
    };
}

fn main() {
    print!("sa", "as");
}
