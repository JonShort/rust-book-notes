enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("something");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));

    let _nothing: Option<i32> = None;

    if let Message::Write(value) = &m {
        println!("Write is: {}", value);
    }

    m.call();
}
