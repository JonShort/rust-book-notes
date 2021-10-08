// pattern_syntax.rs

fn main() {
  // ================== matching literals
  let x = 1;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // ================== matching named literals
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    // inner scope means this y is declared here
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
  }

  // but doesn't affect the usage here
  println!("at the end: x = {:?}, y = {:?}", x, y);

  // ================== multiple patterns
  let x = 1;

  match x {
    // pipe acts as OR
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // ================== Matching Ranges of Values with ..=
  // Can only be used on numeric and char
  let x = 5;

  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  let x = 'c';

  match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
  }

  // ================== Destructuring to Break Apart Values

  // ====== Destructuring Structs

  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point { x: 0, y: 7 };

  // here we destructure the values into a & b with renaming
  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);

  // if we're not renaming we can just name the fields
  let Point { x, y } = p;
  assert_eq!(0, x);
  assert_eq!(7, y);

  let p = Point { x: 0, y: 7 };

  // match statment with pattern matching of field value/s
  match p {
    // allows us to access even unrelated field to the comparison
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    // still needs to be exhaustive, which this is by using two
    // irrefutable values
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }

  // ====== Destructuring Enums
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    // no data, no destructure
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    // struct
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    // tuple with single value
    Message::Write(text) => println!("Text message: {}", text),
    // tuple with multiple values
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
  }

  // ====== Destructuring Nested Structs and Enums
  enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }

  enum MessageTwo {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
  }

  let msg = MessageTwo::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    // follow the chain and destructure both tuples in the same way
    MessageTwo::ChangeColor(Color::Rgb(r, g, b)) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
    // here we handle the case of Hsv enum
    MessageTwo::ChangeColor(Color::Hsv(h, s, v)) => println!(
      "Change the color to hue {}, saturation {}, and value {}",
      h, s, v
    ),
    _ => (),
  }

  // ====== Destructuring Structs and Tuples

  // here we destructure a tuple of tuple, and struct
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
  println!("{}, {}", feet, inches);
  println!("{}, {}", x, y);

  // ==================== Ignoring Values in a Pattern

  // ====== Ignoring an Entire Value with _

  // _ can be used in a number of places to ignore the value
  fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
  }

  foo(3, 4);

  // Ignoring Parts of a Value with a Nested _

  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    // here we ignore the inner value, but match the Option
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }

  println!("setting is {:?}", setting_value);

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    // ignoring items within a tuple
    (first, _, third, _, fifth) => {
      println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
  }

  // ====== Ignoring an Unused Variable by Starting Its Name with _

  // we can "ignore" vars using underscore, but the variable will still be assigned
  let _x = 5;

  let s = Some(String::from("Hello!"));

  // can cause errors when this results in a value being moved, e.g.
  // if let Some(_s) = s {
  // would error as s moved
  if let Some(_) = s {
    println!("found a string");
  }

  println!("{:?}", s);

  // ====== Ignoring Remaining Parts of a Value with ..

  struct PointTwo {
    x: i32,
    y: i32,
    z: i32,
  }

  let origin = PointTwo { x: 0, y: 0, z: 0 };

  match origin {
    // we only need to destructure x, so ignore everything else with ..
    PointTwo { x, .. } => println!("x is {}", x),
  }

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    // .. syntax works with a tuple, and here matches anything between first and last
    // must be unambiguous, e.g. (first, second, .., penultimate, last) would work,
    // but (first, .., third, .., last) wouldn't
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }

  // ====== Extra Conditionals with Match Guards
  let num = Some(4);

  // allows for complex logic to be represented in a single match
  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }

  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Got 50"),
    // can help avoid shadowing, where you'd need to declare a new variable
    // e.g. if this was Some(y), we'd have to operate on the shadowed y
    Some(n) if n == y => println!("Matched, n = {}", n),
    _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {}", x, y);

  let x = 4;
  let y = false;

  match x {
    // can include OR operator, guard always need to match too (it's not just 6 if y)
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }

  // ====== @ Bindings

  // Create a variable at the same time as testing it against a pattern
  enum MessageThree {
    Hello { id: i32 },
  }

  let msg = MessageThree::Hello { id: 5 };

  match msg {
    MessageThree::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    MessageThree::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    MessageThree::Hello { id } => println!("Found some other id: {}", id),
  }
}
