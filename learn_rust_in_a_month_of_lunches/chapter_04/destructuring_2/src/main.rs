struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: &str, name_before: &str, population: u32, date_founded: u32) -> Self {
        Self {
            name: String::from(name),
            name_before: String::from(name_before),
            population,
            date_founded,
        }
    }

    fn print_name(&self) {
        let City {
            name,
            name_before,
            .. // NOTE: These two dots tell Rust not to care about the other parameter inside City.
        } = self;

        println!("The city {name} used to be called {name_before}.");
    }
}

fn main() {
    let tallinn = City::new("Tallinn", "Reveal", 426_538, 1219);
    tallinn.print_name();
}
