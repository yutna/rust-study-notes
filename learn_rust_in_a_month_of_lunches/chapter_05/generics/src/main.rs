// NOTE: concrete = not generic
// When talking about generics, people say that something is “generic over (name of the type).”
// So, for the return_item function, you would say, “The function return_item is generic over type T.”
use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn return_item<T: Debug>(item: T) -> T {
    println!("Here is your item");
    item
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

fn main() {
    let _item = return_item(5);
    print_item(5);

    let chalies = Animal {
        name: "Chalie".to_string(),
        age: 1,
    };

    let number = 55;

    print_item(chalies);
    print_item(number);
}
