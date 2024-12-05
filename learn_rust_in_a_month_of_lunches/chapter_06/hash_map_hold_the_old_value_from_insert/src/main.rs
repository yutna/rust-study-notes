use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();
    let mut old_hashmap_values = Vec::new();

    let hashmap_entries = [(1, "C"), (1, "Java"), (1, "PHP"), (1, "Ruby")];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting {old_value} with {value}!");
            old_hashmap_values.push(old_value);
        }
    }

    println!("All old values: {old_hashmap_values:?}");
}
