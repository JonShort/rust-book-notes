fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.pop();

    let vv = vec![1, 2, 3, 4, 5];
    let third: &i32 = &vv[2];
    println!("The third element is {}", third);

    match vv.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let mut vvv = vec![100, 32, 57];
    for i in &mut vvv {
        *i += 50;
    }

    let vvv = vvv;

    for i in &vvv {
        println!("{}", i);
    }

    let vvvv = spreadsheet_example();

    for i in &vvvv {
        println!("{:?}", i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn spreadsheet_example() -> Vec<SpreadsheetCell> {
    vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]
}
