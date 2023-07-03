#![allow(unused)]
fn main() {
    struct Sequence3<T> {
        first: T,
        second: T,
        third: T,
    }

    impl<T> Sequence3<T> {
        pub fn new(first: T, second: T, third: T) -> Self {
            Self {
                first,
                second,
                third,
            }
        }
    }
}
