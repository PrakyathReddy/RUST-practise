#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method
    fn area(&self) -> u32 { 
        self.width * self.height
    }
    // method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

// each struct is allowed to have multiple impl blocks
impl Rectangle {
    //associated function (no self as first parameter)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 60, height: 45};

    println!("Printing out the rect1 struct: {:#?}", rect1.area());
    dbg!("The area of the rectangle is {}", rect1.area());
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect2.can_hold(&rect3));

    // we use :: to call associated functions
    let sq = Rectangle::square(3);
    println!("new square strucct through associated function is: {:?}", sq);
}

