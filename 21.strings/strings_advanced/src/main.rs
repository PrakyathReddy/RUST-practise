#[allow(unused)]
fn main() {
    let mut s = String::new(); 

    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");

    // above is the same as ...
    let mut s = String::from("initial contents");
    s.push_str(" and more");
    s.push('1');
    println!("{s}");

    let a = String::from("Hello ");
    let b = String::from("World!!");
    // let together = b + &a;
    // println!("{together}");

    let formatted_together = format!("{a} {b}");
    println!("{formatted_together}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }
}

// A String is a wrapper over a Vec<u8>. So it cannot be accessed through indexing