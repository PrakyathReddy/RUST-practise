fn main() {
    fn first_element<T>(array: &[T]) -> &T {
        if array.len()>0 {
            &array[0]
        } else {
            unimplemented!("What is there to be returned here ..?")
        }
    }
}