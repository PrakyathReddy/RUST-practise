fn main() {
    //creating new variable
    let _a: i32 = 5;

    //mutability
    let mut _b: i32 = 5;
    _b = 15;

    //shadowing
    let _c: i32 = 5;
    let _c: i32 = 10;
    println!("c is {_c}");

    //scope
    let _d: i32 = 10;
    {
        let _d: i32 = 50;
        println!("e is {_d}");
    }
    println!("d is {_d}");
}