fn main() {
    #[allow(unused)]
    {
        let v: Vec<i32> = Vec::new();
        let v2 = vec![1,2,3];
    }

    let mut v3: Vec<i32> = Vec::new();
    v3.push(3);
    v3.push(4);
    v3.push(5);
    v3.push(6);

    let v4 = vec![1,2,3,4,5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    let fourth: Option<&i32> = v4.get(3);
    match fourth {
        Some(fourth) => println!("The fourth value is {}", fourth),
        None => println!("The fourth value in None"),
    }

    for i in &v4 {
        println!("{i}");
    }

    println!("{:?}", v3);
    for i in &mut v3 {
        *i += 50;
    }
    println!("{:?}", v3);
}