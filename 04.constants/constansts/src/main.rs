
const NUMBER: u32 = 3;

fn main() {
    println!("Number {}", NUMBER);

    // convert types
    let a = 12;
    let b = a as usize;
    println!("{b}, {a}");

    let name: String = String::from("{Prakyath Reddy}");
    println!("{}", name.capacity());
    println!("{}", name.len());

    let mut title: String = String::from("Solana ");
    title.push_str("Developer");
    println!("{title}");
}