macro_rules! print {
    ($($arg:expr),*) => {
        let acc = String::from("");
        $(
            acc.push
        )*
    };
}

fn main() {
    println!("Hello, world!");
    print!("sa", "as");
}
