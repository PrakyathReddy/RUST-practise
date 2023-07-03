// accept any type 'T' implements 'Display' meaning that they can be formatted as text
fn say_hello<T: std::fmt::Display>(value: &T) {
    println!("Hello {value}!");
}

fn main() {
    say_hello(&true);
    say_hello(&String::from("world"));
    say_hello(&1337);
}
