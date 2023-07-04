#![allow(unused)]
fn main() {
    let value = "hello";
    println!("regular: {}", value);
    println!("pagged: {:_>8}", value);
}
