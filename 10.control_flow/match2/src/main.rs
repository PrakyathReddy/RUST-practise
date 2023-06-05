fn main() {
    let number = 12;
    println!("tell me about number {}", number);

    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number"),
        13..=19 => println!("a teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
        // - => println!("sontha thelivi thetalu vadaku"),
    };

    println!("{:?} -> {}", boolean, binary);
}