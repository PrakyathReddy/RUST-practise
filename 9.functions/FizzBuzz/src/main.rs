fn main() {
    print_fizzbuzz_to(20);
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

///Determine if the first argument is divisible by the second argument
///
///If the second argument is zero, the result is false
fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n,3) {
        "fizz"
    } else {""};
    let buzz = if is_divisible(n,5) {
        "buzz"
    } else {""};
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}
