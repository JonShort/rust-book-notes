// refutability.rs

// irrefutable pattern can never fail, e.g. let x = 5

// refutable pattern can fail e.g: if let Some(x) = a_value
// if a_value is None this pattern is not matched

// Function parameters, let statements, and for loops can only accept irrefutable patterns

fn main() {
  let some_option_value = Some(5);

  // we can't use Some(x) as irrefutable because we have to handle the None case
  // e.g. let Some(x) = some_option_value;

  if let Some(x) = some_option_value {
    println!("{}", x);
  }
}
