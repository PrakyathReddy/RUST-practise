fn main() {
    println!("{}",call_me());
}

fn call_me() -> &'static str {
    "hey there!!"
}
