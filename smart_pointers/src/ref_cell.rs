// ref_cell.rs

// -- interior mutability means a "safe" wrapper around "unsafe" code
// -- code needs to be safe at runtime, but compiler is unable to determine that

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self
        .messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self
        .messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

// This doesn't compile because the self reference in send if borrowed
// `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// #[cfg(test)]
// mod tests {
//   use super::*;

//   struct MockMessenger {
//     sent_messages: Vec<String>,
//   }

//   impl MockMessenger {
//     fn new() -> MockMessenger {
//       MockMessenger {
//         sent_messages: vec![],
//       }
//     }
//   }

//   impl Messenger for MockMessenger {
//     fn send(&self, message: &str) {
//       self.sent_messages.push(String::from(message));
//     }
//   }

//   #[test]
//   fn it_sends_an_over_75_percent_warning_message() {
//     let mock_messenger = MockMessenger::new();
//     let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

//     limit_tracker.set_value(80);

//     assert_eq!(mock_messenger.sent_messages.len(), 1);
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      // borrow a mutable reference here
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  // -- This version panics at runtime, becuase there are two mutable references
  // -- a BorrowMutError
  // impl Messenger for MockMessenger {
  //   fn send(&self, message: &str) {
  //     let mut one_borrow = self.sent_messages.borrow_mut();
  //     let mut two_borrow = self.sent_messages.borrow_mut();

  //     one_borrow.push(String::from(message));
  //     two_borrow.push(String::from(message));
  //   }
  // }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    // --snip--
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    // borrow an immutable reference here
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}

fn main() {}
