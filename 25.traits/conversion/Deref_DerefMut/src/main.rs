fn main() {
    struct DerefExample<T> {
        value: T,
    }

    impl<T> std::ops::Deref for DerefExample<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);
}
