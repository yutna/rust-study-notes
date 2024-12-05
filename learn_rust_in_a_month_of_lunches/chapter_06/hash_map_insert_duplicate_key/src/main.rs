// NOTE: If as HashMap already has a key when you try to put it,
// using `.insert()` will overwrite its value:

use std::collections::HashMap;

fn prevent_by_checking_entry_exists() {
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "Rust Programming Language");

    let key = 1;
    match book_hashmap.get(&key) {
        Some(value) => println!("Key {key} has a value already: {value}"),
        None => {
            // NOTE: again each arm of match statement should return the same type
            book_hashmap.insert(key, "Ruby Programming Language");
        }
    }
}

fn main() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "C Programing Language");
    book_hashmap.insert(1, "Java Programming Language");
    book_hashmap.insert(1, "JavaScript Programming Language");
    book_hashmap.insert(1, "Rust Prgramming Language");

    // The get method take a reference, which is why
    // we have &1 here.
    println!("{:?}", book_hashmap.get(&1));

    // Another example for checking key exists
    prevent_by_checking_entry_exists();
}
