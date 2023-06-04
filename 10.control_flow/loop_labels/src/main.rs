fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count value is {count}");
        let mut remaining = 10;

        loop {
            println!("remaining is {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count is {count}");
}