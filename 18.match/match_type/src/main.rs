#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter,
}

#[allow(unused)]
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {

}