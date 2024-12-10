// NOTE: Closures are quick functions that don’t need a name
// in other words, anonymous functions. Sometimes they are
// called lambdas in other languages.
// It’s easy to notice where closures are because they use ||
// instead of ().

fn main() {
    // NOTE: You can bind a closure to a variable, and then it
    // looks exactly like a function when you use it:
    let my_closure = || println!("This is closure");
    my_closure();

    // NOTE: In between the ||, we can add input variables and
    // types in the same way that we put them inside () for
    // regular functions.
    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5 + 5);

    // NOTE: For longer closures, you need to add a code block.
    let my_closure = || {
        let number = 7;
        let other_number = 10;

        println!("The two numbers are {number} and {other_number}");
    };
    my_closure();

    // NOTE: One thing that makes closures special is that they
    // can take variables from their environment that are outside
    // the closure, even if you only write ||. You can think of a
    // closure as a standalone type that can hold references in the
    // same way that a struct can.
    let number_one = 6;
    let number_two = 10;
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();

    // ---
    let number_one = 6;
    let number_two = 10;
    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);
}
