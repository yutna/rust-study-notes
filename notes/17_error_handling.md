# Error Handling

> Errors are inevitable in software.

- Rust categorizes errors into two main types:
    1. Recoverable errors
    2. Unrecoverable errors
- **Recoverable errors**, like a *file not found* error, should be reported to the user, and the operation can be
  retried.
- **Unrecoverable errors** indicate bugs, such as accessing an array out of bounds, and should stop the program
  immediately.
- Rust does **not** use exceptions. Instead, it uses `Result<T, E>` for recoverable errors and the `panic!` macro for
  unrecoverable errors.

## Unrecoverable Errors with `panic!`

- Panics can occur in two ways:
    1. By performing an action that causes a panic, like accessing an array out of bounds.
    2. By explicitly calling the `panic!` macro.
- By default, panics print a failure message, unwind the stack, clean up, and quit the program.

### Unwinding the Stack or Aborting on Panic

- When a panic occurs, Rust unwinds the stack by default, cleaning up data from each function.
- Alternatively, Rust can abort immediately without cleaning up. The operating system will then clean up the memory.
- To make the binary smaller, you can switch to aborting on panic by adding `panic = 'abort'` to the `[profile]`
  sections in your `Cargo.toml` file.
- For example, to abort on panic in release mode:

    ```toml
    [profile.release]
    panic = 'abort'
    ```

- Setting the `RUST_BACKTRACE` environment variable provides a backtrace, showing the sequence of function calls leading
  to the error.

## Recoverable Errors with `Result`

- The `Result` enum has two variants: `Ok` and `Err`.

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

- `T` is the type of the value returned on success (`Ok`).
- `E` is the type of the error returned on failure (`Err`).

### Shortcuts for Panic on Error: `unwrap` and `expect`

- The `unwrap` method returns the value inside `Ok` or calls `panic!` if it's an `Err`.
- The `expect` method does the same but allows you to set a custom panic message.
- Using `expect` with a good error message helps in debugging.
- **Note**: In production code, prefer `expect` over `unwrap` to provide more context.

### Propagating Errors

- Instead of handling errors within a function, you can return them to the calling code using the `?` operator.
- The `?` operator works like a `match` expression:
    - Returns the value inside `Ok` and continues.
    - Returns the `Err` from the function if it encounters an error.
- The `?` operator can only be used in functions that return `Result`, `Option`, or a compatible type.
- If the function's return type doesn't match, you'll get a compile error.
- The `?` operator can also be used with `Option`:
    - Returns `None` if the value is `None`.
    - Continues with the value inside `Some`.
- The `?` operator doesn't convert between `Result` and `Option`.
- The `fs::read_to_string` function conveniently reads a file into a `String`.
- `Box<dyn Error>` represents any kind of error.
- If `main` returns `Result<(), E>`, the program exits with `0` on `Ok(())` and a nonzero value on `Err`.

## To `panic!` or Not to `panic!`

- Returning `Result` is generally the best choice for functions that might fail.
- In examples, prototypes, and tests, it's okay to use `panic!` instead of returning `Result`.
- The `unwrap` and `expect` methods are useful for prototyping before deciding on error handling.
