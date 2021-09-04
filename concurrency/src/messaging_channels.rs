// messaging_channels.rs

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// We create a new channel using the mpsc::channel function;
// mpsc stands for multiple producer, single consumer. In short,
// the way Rust’s standard library implements channels means a
// channel can have multiple sending ends that produce values but
// only one receiving end that consumes those values

fn main() {
  let (tx, rx) = mpsc::channel();

  // thread has to own the transmitter, so we move it
  thread::spawn(move || {
    let val = String::from("hi");
    // returns Result<T, E> (e.g. err is receiver droppped)
    tx.send(val).unwrap();
    // send takes ownership, so values can't be used after sending
    // e.g. println!("val is {}", val);
  });

  // recv - blocks thread until message received
  // returns Result<T, E>, e.g. err if transmitter closes
  let received = rx.recv().unwrap();
  println!("Got: {}", received);

  // try_recv - doesn't block, returns immediately
  // returns Result<T, E> - err if no messaage

  let (tx, rx) = mpsc::channel();

  // we can clone the transmitter for multiple senders
  let tx1 = tx.clone();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  // iterates until the channel is closed
  for received in rx {
    println!("Got: {}", received);
  }
}
