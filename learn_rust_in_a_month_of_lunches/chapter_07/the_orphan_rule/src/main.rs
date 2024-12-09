// NOTE: The Orphan rule
// - You can implement your trait on someone else's type.
// - You can implement someone else's trait on your type.
// - However, you can NOT implement someone else's trait on someone else's type.

// So what's the best way to get around the orphan rule?
// The easiest way is to wrap someone else's type in a
// **tuple struct**, thereby creating an entirely new type.
// This is called the newtype idiom, and we will learn that now.

#[derive(Clone, Debug)]
struct File(String);

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_bytes = format!("{:?}", self.0.as_bytes());
        write!(f, "{as_bytes}")
    }
}

fn main() {
    let file = File(String::from("I am file contents"));
    println!("{file:?}");
    println!("{file}");
}
