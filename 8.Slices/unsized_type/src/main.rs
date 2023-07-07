fn create_data(small: bool) -> Box<[u8]> {
    if small {
        Box::new([1, 2, 3])
    } else {
        Box::new([1, 2, 3, 4, 5, 6])
    }
}

fn main() {
    let data = create_data(true);
    println!("data: {data:?}");
}
