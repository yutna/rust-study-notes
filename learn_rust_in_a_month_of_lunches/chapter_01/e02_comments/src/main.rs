fn main() {
    // Rust programs start with fn main()
    // You put a code inside a block. It starts with { and ends with }
    let _some_number = 100; // We can write as much as we want here and the compiler won't look at it

    let _another_number/* : i16 */ = 100;

    // cargo doc --open
    sample();
}

/// Converts a string slice in a given base to an integer. Leading and trailing whitespace represent an error.
fn sample() {
    println!("Test doc comment")
}
