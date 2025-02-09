// use std::error::Error;
// use std::fs::File;
// use std::io::{self, Read};

// use std::fs;
// use std::io;

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Manual call
    // panic!("crash and burn");

    // Backtrace example
    // let v = vec![1, 2, 3];
    // v[99];

    // Call a function that returns a `Result`
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     }
    // };

    // Alternatives to Using `match` with `Result<T, E>`
    // let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // The `unwrap` method
    // let _greeting_file = File::open("hello.txt").unwrap();

    // The `expect` method
    // let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    // ---
    let _greeting_file = File::open("hello.txt")?;
    Ok(())
}

// Propagating Error
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// Propagating Error - with the ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// Propagating Error - with the ? operator and chaining method
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

// Propagating Error - shorter version using `fs::read_to_string`
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

// The ? operator with return an `Option`
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
