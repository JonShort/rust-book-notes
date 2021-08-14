use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    simple_closure();
    closure_with_move();
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simple_closure() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));
}

fn closure_with_move() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

struct Cacher<T, P, R>
where
    T: Fn(P) -> R
{
    calculation: T,
    value: HashMap<P, R>
}

impl<T, P, R> Cacher<T, P, R>
where
    T: Fn(P) -> R,
    P: std::cmp::Eq + std::hash::Hash + std::marker::Copy,
    R: std::marker::Copy
{
    fn new(calculation: T) -> Cacher<T, P, R> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: P) -> R {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let _v1 = c.value(1);
    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}
