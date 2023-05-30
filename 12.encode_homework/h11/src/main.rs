// #[test]

fn main() {
    slice_out_of_array();
}

fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..];

    println!("sliced array {:?}", nice_slice);
}