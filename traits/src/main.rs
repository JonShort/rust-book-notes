use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from(&self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

// This is syntax sugar for below
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify<T: Summary>(item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// Here items can be different types (as long as Summary implemented)
pub fn notify_two(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Here items must be the same type (which implements Summary)
pub fn notify_two_same_type<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Two traits can be specified with +
pub fn two_traits(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

// === where clause for trait bounds
// this is a less-cluttered version of the below
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{}, {:?}", t, u);
    6
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
//     println!("{}, {:?}", t, u);
//     6
// }

// impl in return syntax to return type which implements trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("altshort"),
        content: String::from("Carole Baskin killed her husband and fed him to the tigers"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };

    notify(tweet);
    notify(article);
    some_function("Hello", 2);
}
