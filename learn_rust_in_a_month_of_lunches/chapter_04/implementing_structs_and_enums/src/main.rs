// NOTE: Remember that `Self` means the type `Self`,
// and `self` means the variable called `self` that
// refers to the object itself.

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
    mood: Mood,
}

impl Animal {
    fn new_cat(age: u8, mood: Mood) -> Self {
        Self {
            age,
            animal_type: AnimalType::Cat,
            mood,
        }
    }

    fn new_dog(age: u8, mood: Mood) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog,
            mood,
        }
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Cat => println!("The animal is a cat"),
            AnimalType::Dog => println!("The animal is a dog"),
        }
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}");
    }
}

#[derive(Debug)]
enum Mood {
    Good,
    Bad,
    Sleepy,
}

impl Mood {
    fn check(&self) {
        match self {
            Mood::Good => println!("Feeling good!"),
            Mood::Bad => println!("Eh, not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}

fn main() {
    let mut new_animal = Animal::new_cat(10, Mood::Good);
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();

    println!("{}", ["-"; 50].join(""));

    let my_mood = Mood::Sleepy;
    my_mood.check();
}
