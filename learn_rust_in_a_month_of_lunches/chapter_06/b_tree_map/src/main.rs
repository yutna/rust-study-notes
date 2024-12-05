// NOTE: If you want a HashMap that gives you its keys in order,
// you can use a BTreeMap. Underneath, they are differently types,
// but fortunately, their method names and signatures are very similar.

use std::collections::BTreeMap;

struct City {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    for (year, population) in tallinn.population {
        println!("In {year}, Tallinn had a population of {population}.");
    }
}
