use std::collections::HashMap;

fn main() {
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];

    let number_word_hashmap: HashMap<_, _> = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect();

    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );

    // ------------------------------------------

    let keys = vec![0, 1, 2, 3, 4, 5].into_iter();
    let values = vec!["zero", "one", "two", "three", "four", "five"].into_iter();

    let number_word_hashmap: HashMap<i32, &str> = keys.zip(values).collect();

    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );
}

