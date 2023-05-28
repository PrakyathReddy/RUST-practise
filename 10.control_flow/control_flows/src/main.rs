fn main() {
    let a: i32 = 5;
    if a>5 {
        println!("a is greater than 5");
    } else if a>3 {
        println!("a is greater than 3");
    } else {
        println!("a is less than or equal to 3");
    }

    let b: i32 = if a>5 {1} else {-1};
    println!("{b}");

    //loop - loop name should start with '
    'outer: loop {
        println!("loop forever");
        loop {
            break 'outer;
        }
    }

    let _x: i32 = loop {
        break 5;
    };
    println!("looping x");

    //while loop
    let mut a: i32 = 0;
    while a<5 {
        println!("a is {a}");
        a+=1;
    }

    //for loop
    let a: [i32;5] = [1,2,3,4,5];
    for i in a {
        println!("{}",i);
    }

}