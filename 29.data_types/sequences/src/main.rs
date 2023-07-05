#![allow(unused)]
fn main() {
    let statically_sized_array: [u32; 3] = [1,2,3];
    let mut dynamically_sized_array: Vec<u32> = vec![1,2,3,4,45];

    println!("{:?}", dynamically_sized_array);
    
    dynamically_sized_array.pop();

    println!("{:?}", dynamically_sized_array);

    dynamically_sized_array.push(12);

    println!("{:?}", dynamically_sized_array);

    println!("{:?}", dynamically_sized_array[2]);

}