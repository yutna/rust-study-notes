// NOTE: With `.entry()`, you can try to make an entry and then another method like
// `.or_insert()` to insert a default value if there is no key. The interesting part
// is that the second method also returns a mutable reference. so you can change it
// if you want.

use std::collections::HashMap;

fn main() {
    let book_collection = vec![
        "C in Action",
        "Java in Action",
        "SQL in Action",
        "Ruby in Action",
        "Ruby in Action", // NOTICE: Ruby in Action appear twice
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }

    for (book, true_of_false) in book_hashmap {
        println!("Do we have {book}? {true_of_false}");
    }
}
