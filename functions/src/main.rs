use std::io;

fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    let five = five();

    let mut user_choice = String::new();

    println!("Choose a number");

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");

    let user_choice: i32 = match user_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    another_function(user_choice, plus_one(five));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
