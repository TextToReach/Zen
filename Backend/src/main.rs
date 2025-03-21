macro_rules! print {
    ($($arg:expr),*) => {
        {
            let mut result = String::new();
            let mut _first = true;
            $(
                if !_first {
                    result.push(' ');
                }
                result.push_str(&$arg.to_string());
                _first = false;
            )*

            // Print the concatenated result
            println!("{}", result);
        }
    };
}

fn main() {
    print!("sa", "as", "123");
    
}
