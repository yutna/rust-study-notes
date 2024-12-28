// NOTE: if Expression
// - An `if` expression allows you to branch your code depending on conditions.
// - All `if` expressions start with the keyword `if`, followed by a condition.
// - Blocks of code associated with the conditions in `if` expressions are
//   sometimes called arms.
// - Optionally, we can also include an `else` expression, to give the program
//   an alternative block of code to execute should the condition evaluate to
//   `false`.
// - The condition **MUST** be a `bool`.
// - You can use multiple conditions by combining `if` and `else` in an
//   `else if` expression.
// - Rust only executes the block for the first `true` condition, and once it
//   finds one, it doesn’t even check the rest.

// NOTE: Using if in a let Statement
// - Because if is an expression, we can use it on the right side of a let
//   statement to assign the outcome to a variable.
// - The results from each arm **MUST** be the same type.

fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // ---

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    // ---

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // ---

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if in a let statement.
    println!("The value of number is: {number}");

    // Type mismatch error, each arm must be the same type!
    // let number = if condition { 5 } else { "six" };
}
