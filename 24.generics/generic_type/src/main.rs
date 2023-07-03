struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let a = Point {x: 2, y: 3};
    println!("x={}, y={}", a.x, a.y);

    let b = Point {x: String::from("one"), y: String::from("two")};
    println!("x: {}, y: {}", b.x, b.y);
}