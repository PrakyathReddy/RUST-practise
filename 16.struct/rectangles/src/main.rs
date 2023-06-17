#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};

    println!("Printing out the rect1 struct: {:#?}", rect1);
    dbg!("The area of the rectangle is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}