// NOTE: Data Types
// - When many types are possible, such as when we converted a String to a
//   numeric type using parse, we must add a type annotation.

// NOTE: Scalar Types
// - A scalar type represents a single value.
// - Rust has four primary scalar types:
//   - integers
//   - floating-point numbers
//   - Booleans
//   - characters

// NOTE: Integer Types
// - An integer is a number without a fractional component.
// - The `isize` and `usize` types depend on the architecture of the computer
//   your program is running on
// - Number literals that can be multiple numeric types allow a type suffix,
//   such as `57u8` (`let number: u8 = 57;`).
// - Number literals can also use `_` as a visual separator to make the number
//   easier to read, such as 1_000_000.
// - Integer types default to i32.
// - The primary situation in which you’d use `isize` or `usize` is when
//   indexing some sort of collection.
//
// | Length    | Signed | Unsigned |
// |-----------|--------|----------|
// | 8-bit     | i8     | u8       |
// | 16-bit    | i16    | u16      |
// | 32-bit    | i32    | u32      |
// | 64-bit    | i64    | u64      |
// | 128-bit   | i128   | u128     |
// | arch      | isize  | usize    |
//
// | Number literals  | Example     |
// |------------------|-------------|
// | Decimal          | 98_222      |
// | Hex              | 0xff        |
// | Octal            | 0o77        |
// | Binary           | 0b1111_0000 |
// | Byte (u8 only)   | b'A'        |

// NOTE: Floating-Point Types
// - Rust also has two primitive types for floating-point numbers: f32 and f64.
// - The default type is f64.
// - All floating-point types are signed.

// NOTE: The Boolean Type
// - A Boolean type in Rust has two possible values: `true` and `false`.
// - Booleans are one byte in size.
// - The Boolean type in Rust is specified using `bool`.

// NOTE: The Character Type
// - Rust’s `char` type is the language’s most primitive alphabetic type.
// - We specify `char` literals with single quotes.
// - Rust’s `char` type is four bytes in size and represents a Unicode Scalar
//   Value.

// NOTE: Compound Types
// - Compound types can group multiple values into one type.
// - Rust has two primitive compound types: tuples and arrays.

// NOTE: The Tuple Type
// - A tuple is a general way of grouping together a number of values with a
//   variety of types into one compound type.
// - Tuples have a fixed length: once declared, they cannot grow or shrink in
//   size.
// - We create a tuple by writing a comma-separated list of values inside
//   parentheses
// - Each position in the tuple has a type, and the types of the different
//   values in the tuple don’t have to be the same.
// - We can use pattern matching to destructure a tuple value.
// - We can also access a tuple element directly by using a period (.) followed
//   by the index of the value we want to access.
// - The first index in a tuple is 0.
// - The tuple without any values has a special name, *unit*. `()`
// - The unit represent an empty value or an empty return type.
// - Expressions implicitly return the unit value if they don’t return any
//   other value.

// NOTE: The Array Type
// - Every element of an array must have the same type.
// - Arrays in Rust have a fixed length.
// - We write the values in an array as a comma-separated list inside square
//   brackets
// - You write an array’s type using square brackets with the type of each
//   element, a semicolon, and then the number of elements in the array.
// - You can also initialize an array to contain the same value for each
//   element by specifying the initial value, followed by a semicolon, and then
//   the length of the array in square brackets. (`let arr = [3; 5];`)
// - You can access elements of an array using indexing.

use std::io;

fn main() {
    let byte = b'A';
    println!("{}", byte); // print 65 NOT 'A'

    // ---

    let _x = 2.0; // f64
    let _y: f32 = 3.0;

    // ---

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;

    // ---

    let _t = true;
    let _f: bool = false;

    // ---

    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    // ---

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // Pattern matching to destructure a tuple value
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");

    // ---

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("{first} {second}");

    let arr = [3; 5];
    println!("{:?}", arr);

    // ---

    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was NOT a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
