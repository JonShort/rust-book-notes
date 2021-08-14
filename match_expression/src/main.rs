enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    SilverDollar(u8),
}

fn main() {
    let my_coin = Coin::SilverDollar(13);
    let value = value_in_cents(my_coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("My coin value is: {}", value);
    println!("Six is: {:?}", six);
    println!("none is: {:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::SilverDollar(value) => value,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
