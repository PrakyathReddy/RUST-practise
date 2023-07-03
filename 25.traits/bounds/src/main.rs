#![allow(unused)]
fn main() {

    struct MyStruct<A,B> {
        a: A,
        b: B,
    }

    use std::fmt::Display;

    impl<A: Display,B: Display> MyStruct<A,B> {
        fn print(&self) {
            println!("a: {}, b: {}", self.a, self.b);
        }
    }
}