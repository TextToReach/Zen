macro_rules! print {
    ($($arg:expr),*) => {
        {
            let mut result = String::new();
            let mut first = true;
            $(
                if !first {
                    result.push(' ');
                }
                result.push_str(&$arg.to_string());
                first = false;
            )*

            // Print the concatenated result
            println!("{}", result);
        }
    };
}

fn main() {
    print!("sa", "as");
}
