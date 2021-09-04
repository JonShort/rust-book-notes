// move_closures.rs

use std::thread;

// This cannot compile due to "may outlive borrowed value `v`"
// fn main() {
//   let v = vec![1, 2, 3];

//   let handle = thread::spawn(|| {
//     println!("Here's a vector: {:?}", v);
//   });

//  drop(v); // oh no!

//   handle.join().unwrap();
// }

fn main() {
  let v = vec![1, 2, 3];

  // move transfers ownership to the spawned thread
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  // ...which means trying to drop the vec would error on compile
  //  drop(v); // oh no!

  handle.join().unwrap();
}
