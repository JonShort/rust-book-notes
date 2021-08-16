// reference_counted.rs

// == This shows how using a Box (reference to heap) doesn't fit the use-case where
// == multiple values need to reference the same value

// enum List {
//   Cons(i32, Box<List>),
//   Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//   let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//   let b = Cons(3, Box::new(a));
//   let c = Cons(4, Box::new(a));
// }

// == Here we use a reference counter, which will keep track of our references
// == so we know when the shared value is dereferenced, and can drop it

// enum List {
//   Cons(i32, Rc<List>),
//   Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//   let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//   // Rc::clone doesn't make a deep copy of the data
//   // it just increments the reference count
//   let b = Cons(3, Rc::clone(&a));
//   let c = Cons(4, Rc::clone(&a));
// }

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Via immutable references, Rc<T> allows you to share data between multiple
// parts of your program for reading only. If Rc<T> allowed you to have multiple
// mutable references too, you might violate one of the borrowing rules discussed
// in Chapter 4
