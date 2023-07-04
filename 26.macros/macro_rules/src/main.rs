#![allow(unused)]
fn main() {
    macro_rules! create_vec {
        ( $( $item:expr ),* ) => {
            {
                let mut result = Vec::new();
            $(
                result.push($item);
            )*
            result
            }
        }
    }
    let items = create_vec!(1, 2, 3);
    println!("{:?}", items);
}
