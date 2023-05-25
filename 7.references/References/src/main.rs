fn main() {
    let mut y: i32 = 10;
    let ref_x: &mut i32 = &mut y;
    *ref_x = 15;
    println!("x: {y}");
}