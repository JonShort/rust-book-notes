use std::cmp::PartialOrd;
use std::fmt;

// Two generics allow values to be independent
struct Point<T, U> {
    x: T,
    y: U,
}

// The x method will be available on all Point structs
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This method will only be available on Point structs with 2 floats
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Here the method has generics also, with the result mixing generics from the method and the struct
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// <T> allows generic type to be set via. vec param
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// === If we wanted to use clone trait instead, code would require rework
// fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
//     let mut largest = list[0].clone();

//     for item in list.iter() {
//         if item > &largest {
//             largest = item.clone();
//         }
//     }

//     largest
// }

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

// --- Here how to manually implment debug for the Pair struct
// impl<T: fmt::Debug> fmt::Debug for Pair<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Point")
//          .field("x", &self.x)
//          .field("y", &self.y)
//          .finish()
//     }
// }

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let int_and_float = Point { x: 5, y: 1.0};
    let float_and_float = Point { x: 5.5, y: 1.0};

    // here the struct has the x method, but not the distance_from_origin
    println!("{}", int_and_float.x());
    // println!("{}", int_and_float.distance_from_origin());

    // Here the struct has both methods, due to types matching
    println!("{}", float_and_float.x());
    println!("{}", float_and_float.distance_from_origin());

    // Here a struct is passed with different generics
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = int_and_float.mixup(p2);
    println!("{}, {}", p3.x, p3.y);

    // ---------------------------------------------------

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // ---------------------------------------------------

    let pair_one = Pair::new(5, 10);

    pair_one.cmp_display();

    let pair_two = Pair::new(Pair::new(5, 5), Pair::new(1, 2));

    println!("{:?}", pair_two);

    // -- this doesn't work because Pair doesn't implement PartialOrd
    //pair_two.cmp_display();
}
