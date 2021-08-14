use std::collections::HashMap;

fn main() {
    // inserting items into newed-up hash map
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert(String::from("Yellow"), 50);

    iteration();

    ownership();

    access();

    existing_values();
}

fn iteration() {
    // iterating over two vectors and populating a hash map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // _, _ here allows hash map to infer type based on above vectors
    let scores_two: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:#?}", scores_two);
}

fn ownership() {
    // hash maps take ownership of values that don't implement the Copy trait
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // this would panic due to field_name now being owned by map
    //println!("{}", field_name);

    let field_name_two = String::from("Favourite color");
    let field_value_two = String::from("Blue");

    let mut map_two = HashMap::new();
    map_two.insert(&field_name_two, &field_value_two);

    // this is fine because we have stored a reference in the hash map
    println!("{}", field_name_two);
    println!("{}", field_value_two);

    let field_nickname = "Five".to_string();
    let field_score: i32 = 5;
    let mut map_three = HashMap::new();
    map_three.insert(field_nickname, field_score);

    // but i32 is fine because it has Copy trait
    println!("{}", field_score);
}

fn access() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let some_score = scores.get(&team_name);

    // Some(value) returned, because the key might not exist in the hash map
    match some_score {
        None => println!("No score found for {} team", team_name),
        Some(score) => println!("{} team score: {:#?}", team_name, score),
    }

    // This prints the key:values in arbitrary order
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn existing_values() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // value has been overwritten
    println!("{:#?}", scores);

    // entry will only execute method if the key doesn't already exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // iterate over each word and pass word as key to entry
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // increase the referenced integer by 1
        *count += 1;
    }

    println!("{:#?}", map);
}

// Exercises

/*
    - Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    - Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

    - Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/
