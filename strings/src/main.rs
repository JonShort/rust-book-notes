fn main() {
    let data = "bar";

    let mut empty_string = String::new();
    empty_string.push_str("foo");
    empty_string.push_str(data);
    println!("{}", empty_string);

    let mut s = data.to_string();
    let ss = "initial contents".to_string();
    s.push('s');
    println!("{}, {}", s, ss);

    let s1 = String::from("lemon");
    let s2 = String::from("lime");
    let s3 = s1 + &s2;
    // s3 takes ownership of s1 but not s2
    println!("{}", s3);

    let st1 = String::from("tic");
    let st2 = String::from("tac");
    let st3 = String::from("toe");
    let tictactoe = format!("{}-{}-{}", st1, st2, st3);
    println!("{}", tictactoe);

    for c in tictactoe.chars() {
        println!("{}", c);
    }

    for b in tictactoe.bytes() {
        println!("{}", b);
    }
}
