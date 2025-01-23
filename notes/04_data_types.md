# Data Types

- Every value is Rust is of a certain *data type*
- Rust is *statically typed language*, which means that it must know the types of all variables at compile time.
- In cases when many types are possible, such as when we converted a `String` to a numberic type (i8, u8, i16, u16, ...) with `parse`, we **MUST** add a type annotation.

## Scalar Types

- A *scalar* type represents a single value.
- Rust has four primary scalar types:
  1. integers
  2. floating-point numbers
  3. Booleans
  4. characters.

### Integer Types

- An *integer* is a number without a fractional component.
- The `isize` and `usize` types depend on the architecture of the computer your program is running on.
- The primary situation in which you'd use `isize` or `usize` is when indexing some sort of collection.
- Number literals that can be multiple numeric types allow a type suffix, such as `57u8`.
- Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`.
- Hex (0xff)
- Octal (0o77)
- Binary (0b1111_0000)
- Byte (u8 only) (b'A')
- Integer types default to `i32`.

### Floating-Point Types

- The *floating-point* is a number with decimal points.
- Rust's floating-point types are `f32` and `f64`.
- The default type is `f64`.
- All floating-point types are signed.

### The Boolean Type

- A Boolean type in Rust has two possible values; `true` and `false`.
- Boolean are one byte in size.
- The Boolean type in Rust is specified using `bool`.

### The Character Type

- Rust's `char` type is the language's most primitive alphabetic type.
- We specify `char` literals with single quotes.

## Compound Types

- *Compound types* can group multiple values into one type.
- Rust has two primitive compound types:
  1. tuples
  2. arrays

### The Tuple Type

- A *tuple* is a general way of grouping together a number of values with a variety of types into one compound type.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- We create a tuple by writing a comma-separated list of values inside parentheses.
- Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same.
- To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.
- We also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access.
- The tuple without any values has special name, *unit*. `()`
- The unit, its represent an empty value or an empty return type.
- Expressions implicitly return the unit value if they don't return any other value.

### The Array Type

- Another way to have a collection of multiple values is with an *array*.
- Every element of an array must have the same type.
- Arrays in Rust have a fixed length.
- We writes the values in an array as a comma-separated list inside square brackets.
- Arrays data allocated on the stack.
- Arrays are more useful when you know the number of elements will not change.
- You write an array's type using square brackets with the type of each element, a semicolon, and then the number of elements in the array. (`let a: [i32; 5] = [...]`)
- You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in squre brackets. (`let a = [3; 5]`)
- You can access elements of an array using indexing.
