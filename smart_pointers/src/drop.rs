// drop.rs

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let d = CustomSmartPointer {
    data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created.");

  // cannot call drop manually
  // c.drop();
  // println!("CustomSmartPointer dropped before the end of main.");

  // to drop early, must call std::mem::drop
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
}
