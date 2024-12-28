// NOTE: Constants
// - Constants aren’t just immutable by default, they’re always immutable.
// - You aren’t allowed to use `mut` with constants.
// - You declare constants using the `const` keyword instead of the `let`
//   keyword.
// - The type of the value must be annotated.
// - Constants can be declared in any scope, including the global scope.
// - Constants may be set only to a constant expression, not the result of a
//   value that could only be computed at runtime.
// - Rust’s naming convention for constants is to use all uppercase with
//   underscores between words.
// - Constants are valid for the entire time a program runs, within the scope
//   in which they were declared.

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("{THREE_HOURS_IN_SECONDS}");
}
