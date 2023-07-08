#![allow(unused)]
fn main() {
    pub trait From<T> {
        fn from(value: T) -> self;
    }

    pub trait Into<T> {
        fn into(self) -> T; 
    }
}