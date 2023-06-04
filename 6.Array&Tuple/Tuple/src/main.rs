fn main() {
    let t: (u8, bool) = (7, true);
    println!("2nd index is: {}", t.1);
    let bool_val = t.1;
    println!("{bool_val}");
    println!("1st index: {}", t.0);

    // struct types
    struct _User {
        name: String,
        age: i32,   
    }
}