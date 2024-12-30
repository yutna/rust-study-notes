// NOTE: Unit-Like Structs
// - You can also define structs that don’t have any fields!
// - These are called unit-like structs because they behave similarly to ().
// - Can be useful when you need to implement a trait on some type but don’t
//   have any data that you want to store in the type itself.
// - To define unit-like struct:
//   - We use the `struct` keyword.
//   - The name we want.
//   - A semicolon.
//   - No need for curly brackets or parentheses!

struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;
}
