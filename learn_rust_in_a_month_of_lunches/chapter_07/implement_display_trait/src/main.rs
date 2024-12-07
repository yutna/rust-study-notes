use std::fmt;

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

// NOTE: If you implement `Display` for a type,
// you get the `ToString` trait for free, which
// gives you the `.to_string()` method.
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

fn print_excitedly(input: String) {
    println!("{input}!!!!!");
}

fn main() {
    let mr_mantle = Cat {
        name: "Raggie Mantle".to_string(),
        age: 4,
    };

    print_excitedly(mr_mantle.to_string());
    println!(
        "Mr. Mantle's String is {} letters long.",
        mr_mantle.to_string().chars().count()
    );
}
