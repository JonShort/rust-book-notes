// all_patterns.rs

fn main() {
    // ============ match
    let a = "a";

    // match must be exhaustive, _ is used as catchall
    let b = match a {
        "a" => "b",
        _ => "letter b",
    };

    println!("{}", b);

    // ============ if let

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if let is similar to match, but with just one condition
    // e.g:
    if let "a" = a {
        println!("a is {}", a);
    }

    // flexible, and can be matched with else, else if, else if let
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // can introduce shadowed variables, here age is resolved to a u8
    // rather than a Result as previous
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    // compiler doesn't check for exhaustivness when using if let
    // so if we didn't have this else we would miss some cases
    } else {
        println!("Using blue as the background color");
    }

    // ============ while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // continues running until the result doesn't match the condition
    // e.g. this loop pops everything from our stack
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // ============ for loops
    let v = vec!['a', 'b', 'c'];

    // pattern directly follows the keyword, here we match the values in the enumarated tuple
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // ============ let
    // let PATTERN = EXPRESSION
    let _x = 5;

    let (_x, _y, _z) = (1, 2, 3);

    // ============ function parameters
    fn _foo(_x: i32) {}

    // function params also match patterns, e.g. here we get values from a tuple reference
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
