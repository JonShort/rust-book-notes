#![allow(unused_variables)]
use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    result_example()
}

// Call panic! to exit with error message
fn instant_panic() {
    panic!("crashed!")
}

// This code will panic, use the RUST_BACKTRACE env var to see
// the backtrace point back to this invlid index of vec
fn panic_in_external() {
    let v = vec![1, 2, 3];

    v[99];
}

// This example handles the error using Result
fn result_example() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("problem creating the file: {:?}", e),
            },
            other_error => panic!("problem opening the file: {:?}", other_error),
        },
    };
}

// This example will propogate the error to the caller using Result
fn long_read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// This is the same as above but uses the ? Operator
// note - ? can only be used in func that returns Result
fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Here we chain the call to make it even shorter
fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// The shortest way of writing the above!
fn shortest_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
