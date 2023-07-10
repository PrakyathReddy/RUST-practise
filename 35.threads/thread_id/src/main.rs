use std::thread as my_name;
fn main() {
    println!("{:?}", my_name::current().id());
}