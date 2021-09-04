// mutexes.rs
// abbreviation for mutual exclusion

// mutexes work using a "lock" system - threads must request access
// (unlock), use the data, then signal done (lock)

// - You must attempt to acquire the lock before using the data.
// - When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let m = Mutex::new(5);

  {
    // blocks thread until it's our turn to have the lock
    let mut num = m.lock().unwrap();

    // MutexGuard<T> is a smart pointer, with deref yielding the inner value
    // and Drop implementation which releases the lock
    *num = 6;
  }

  println!("m = {:?}", m);

  // Arc is a reference counter that is safe to use in concurrent situations
  // It is independant of Rc because of the perf hit for the extra guarantees
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      // Mutex<T> provides interior mutability, as the Cell family does. In the
      // same way we used RefCell<T> in Chapter 15 to allow us to mutate contents
      // inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}

// Another detail to note is that Rust can’t protect you from all kinds of logic errors
// when you use Mutex<T>. Recall in Chapter 15 that using Rc<T> came with the risk of
// creating reference cycles, where two Rc<T> values refer to each other, causing memory
// leaks. Similarly, Mutex<T> comes with the risk of creating deadlocks. These occur when
// an operation needs to lock two resources and two threads have each acquired one of the
// locks, causing them to wait for each other forever. If you’re interested in deadlocks,
// try creating a Rust program that has a deadlock; then research deadlock mitigation
// strategies for mutexes in any language and have a go at implementing them in Rust. The
// standard library API documentation for Mutex<T> and MutexGuard offers useful information.
