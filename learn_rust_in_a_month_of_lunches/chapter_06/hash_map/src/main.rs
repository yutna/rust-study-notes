// NOTE: A HashMap is a collection made out of keys and values.
// You use the key to look up the values that match the key.

// NOTE: Creating a new HashMap is easy; you can just use
// `HashMap::new().` After that, you can use the
// `.insert(key, value)` method to insert items.

// NOTE: The keys of a HashMap are **NOT ordered**, so if
// you print every key in a HashMap together; it will
// probably print differently.

use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    for (year, population) in tallinn.population {
        println!("In {year}, Tallinn had a population of {population}.");
    }
}
