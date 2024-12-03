// NOTE: You use Option when something might or might not exist.
// When a value exists, it is Some(value), and when it doesn’t, it’s None.

// NOTE: Panic means that the program stops before the problem happens
// Rust sees that the function wants something impossible and stops.
//  It “unwinds the stack”  (takes the value off your stack) and tell you,
// “Sorry, I can’t do that.”
// NOTE: To fix this, we will change the return type from i32 to Option<i32>.
// This means “give me a Some(i32) if it’s there, and give me None if it’s not.”
//  We say that the i32 is “wrapped” in an Option, which means it’s inside an Option.
// If it’s Some, you have to do something to get the value out:
fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        // NOTE: In this case, the value  is still inside the Option.
        // How do we get the value out of there?
        // We can get the value inside an Option with a method called .unwrap()
        // but be careful with .unwrap(). It’s just like unwrapping a present:
        // maybe there’s something good inside, or maybe there’s an angry
        // snake inside. You only want to .unwrap() if you are sure.
        // If you unwrap a value that is None, the program will panic:
        Some(value[4])
    }
}

fn handle_options(options: &Vec<Option<i32>>) {
    for item in options {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a none!"),
        }
    }
}

fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    let mut options = Vec::new();

    // This will panic at case: small, because use unwrap() on None
    // println!(
    //     "{:?} {:?}",
    //     try_take_fifth(small),
    //     try_take_fifth(big).unwrap()
    // );

    // NOTE: But we don’t have to use .unwrap() We can use a match instead.
    // With match, we can print the value if we have Some and not touch it if
    // we have None
    options.push(try_take_fifth(small));
    options.push(try_take_fifth(big));
    handle_options(&options);
}
