fn main() {
    let x: Option<i32> = Some(10);
    match x {
        Some(val) => println!("Value is: {}", val),
        None => println!("No value"),
    }

}