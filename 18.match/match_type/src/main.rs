#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska, 
    Washington,
    California, 
    NewYork,
}

#[allow(unused)]
fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {

}