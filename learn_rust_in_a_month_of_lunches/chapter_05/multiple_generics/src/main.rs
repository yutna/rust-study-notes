// NOTE: You can use + if you want to indicate more than one trait
// To sum up, <U: Display + PartialOrd> means there is a generic type that we
// are calling U, and it needs to have these two traits:

use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U) {
    println!(
        "{statement}! Is {input_1} greater than {input_2}? {}",
        input_1 > input_2
    );
}

// NOTE: To make generic functions easier to read, we can also use the keyword
// `where` right before the code block:

fn comapre_and_display2<T, U>(statement: T, input_1: U, input_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{statement}! Is {input_1} greater than {input_2}? {}",
        input_1 > input_2
    );
}
// Using where is a good idea when you have many generic types.
// Also note the following:
// NOTE: If you have one variable of type T and another variable of type T,
// they must be the same type.
// NOTE: If you have one variable of type T and another variable of type U,
// they can be different types. But they can also be the same.
fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    println!("I have two things to says: {statement_1} and {statement_2}");
}

fn main() {
    compare_and_display("Listen up!", 9, 8);
    comapre_and_display2("Listen up! 2", 9, 8);

    // Type T is a &str, but type U is a String. No problem: both of these implement Display.
    say_two("Hello there!", String::from("I hate sand."));
    // Here both types are String. No problem: T and U don’t have to be different types.
    say_two(
        String::from("Where is Padme?"),
        String::from("Is she all right?"),
    );
}
