// NOTE: Associated Functions
// - All functions defined within an impl block are called associated functions
//   because they’re associated with the type named after the impl.
// - We can define associated functions that don’t have `self` as their first
//   parameter (and thus are not methods) because they don’t need an instance
//   of the type to work with.
// - Associated functions that aren’t methods are often used for constructors
//   that will return a new instance of the struct.
// - These are often called `new`, but `new` isn't a special name and isn't
//   built into the language.
// - To call this associated function, we use the `::` syntax with the struct
//   name. This function is namespaced by the struct: the `::` syntax is used
//   for both associated functions and namespaces created by modules.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    println!("{} {}", sq.width, sq.height);
}
