fn main() {
    let a = 20;
    let b = &a;
    assert!(*b == 20);
    println!("{}", *b);
}
