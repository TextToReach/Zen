mod library;

fn main() {
    Print!("Selam", 1.12, 1123);
    let mut selam = Input!("Bir sayı giriniz: ");
    selam = "selamlar".to_owned();

    println!("{}", selam.parse::<i32>().unwrap() + 123);
}
