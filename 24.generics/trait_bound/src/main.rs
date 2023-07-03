#![allow(unused)]
fn main() {
    use std::ops::Add;

    struct Sequence3<T> {
        first: T,
        second: T,
        third: T,
    }

    impl<T> Sequence3<T>
    where
        T: Copy + Add<Output = T>,
    {
        pub fn sum(&self) -> T {
            self.first + self.second + self.third
        }
    }
}
