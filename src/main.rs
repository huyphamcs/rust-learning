#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let coin = Coin::Penny;
    println!("{}", get_value(&coin));
}

fn get_value(coin: &Coin) -> usize {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}