fn main() {
    let number = 13;

    match number {
        1 => println!("This is one"),
        4...20 => println!("this is between 2 and 20"),
        // 3 | 2 => println!("It's either 3 or 4"),
        _ => println!("thellarlu ante evadu dekadu ikkada")
    }
}