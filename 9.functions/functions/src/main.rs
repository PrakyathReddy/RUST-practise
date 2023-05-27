fn main() {
    let z: i32 = my_function(22);
    println!("my_function returned {z}");
}

fn my_function(x: u32) -> i32 {
    println!("my_function called with: {x}");
    let y = 10;
    y
}

