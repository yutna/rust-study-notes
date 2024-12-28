// NOTE: Functions
// - Rust code uses snake case as the conventional style for function and
//   variable names, in which all letters are lowercase and underscores
//   separate words.
// - We define a function in Rust by entering `fn` followed by a function name
//   and a set of parentheses.
// - The curly brackets tell the compiler where the function body begins and
//   ends.
// - Rust doesn’t care where you define your functions, only that they’re
//   defined somewhere in a scope that can be seen by the caller.

// NOTE: Parameters
// - We can define functions to have parameters, which are special variables
//   that are part of a function’s signature.
// - When a function has parameters, you can provide it with concrete values
//   for those parameters.
// - Technically, the concrete values are called *arguments*.
// - In function signatures, you **MUST** declare the type of each parameter.
// - When defining multiple parameters, separate the parameter declarations
//   with commas.

// NOTE: Statements and Expressions
// - Rust is an expression-based language.
// - **Statements** are instructions that perform some action and do NOT return
//   a value.
// - **Expressions** evaluate to a resultant value.

// NOTE: Functions with Return Values
// - Functions can return values to the code that calls them.
// - We must declare return type after an arrow (->).
// - In Rust, the return value of the function is synonymous with the value of
//   the final expression in the block of the body of a function.
// - You can return early from a function by using the `return` keyword and
//   specifying a value.

fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // ---

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // ---

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
