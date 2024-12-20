// NOTE: The thing to remember about trait is that they 
// are about the shared behavior of something. Any type
// that implements them will all have these abilities.

struct Dog {
    name: String,
} 

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof woof!");
    }

    fn run(&self) {
        println!("The dog is running!");
    }
}

impl DogLike for Dog {}

impl DogLike for Parrot {
    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

fn main() {
    let rover = Dog {
        name: "Rover".to_string(),
    };

    let brian = Parrot {
        name: "Brain".to_string(),
    };

    rover.bark();
    rover.run();

    brian.bark();
    brian.run(); 
}

