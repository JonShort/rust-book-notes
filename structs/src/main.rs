struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let black = Color(0, 0, 0);

    println!("Username1 is {}", user1.username);
    println!("Email1 is {}", user1.email);
    println!("Sign in count is {}", user1.sign_in_count);
    println!("Username2 is {}", user2.username);
    println!("Email2 is {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
