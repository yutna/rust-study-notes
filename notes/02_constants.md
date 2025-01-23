# Constants

- Like immutable variables, *constants* are values that are bound to a name and are NOT allowed to change.
- There are a few differences between constants and variables.
  1. You are NOT allowed to use `mut` with constants. (They're always immutable).
  2. You can declare constants using the `const` keyword.
  3. The type of the value *must* be annotated.
  4. Constants can be declared in any scope.
  5. Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
- Rust's naming convention for constants is to use all UPPERCASE with underscore (_) between words.
- Constants are valid for the entire time a program runs, within the scope in which they were declared.
