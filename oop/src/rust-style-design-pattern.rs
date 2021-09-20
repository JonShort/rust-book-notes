// rust-style-design-pattern.rs

// This accomplishes similar things to the oop design pattern, but benefits from
// compile-time checking on misuse. Rather than catering for the invalid scenarios
// the compiler will error when the code tries to execute a method which doesn't
// exist on the type.

pub struct DraftPost {
  content: String,
}

pub struct PendingReviewPost {
  content: String,
}

pub struct Post {
  content: String,
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

impl PendingReviewPost {
  pub fn reject(self) -> DraftPost {
    DraftPost {
      content: self.content,
    }
  }

  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");
  // compile-time error as content doesn't exist on post
  // assert_eq!("", post.content());

  let post = post.request_review();

  let post = post.approve();

  println!("{}", post.content());
}
