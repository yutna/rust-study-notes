# Error Handling

> Errors are a fact of life in software.

- Rust group errors into two major categories:
  1. Recoverable error
  2. Unrecoverable error
- Recoverable error, such as a *file not found* error, we most likely just want to report the problem to the user and retry the operation.
- Unrecoverable errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program.
- Rust do **NOT** have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

## Unrecoverable Errors with `panic!`

- There are two ways to cause a panic in practice:
  1. By taking an action that causes our code to panic (such as accessing an array past the end).
  2. By explicitly calling the `panic!` macro.
- By default, these panics will print a failure message, unwind, clean up the stack, and quit.

### Unwinding the Stack or Aborting in Response to a Panic

- By default, when a panic occurs the program starts *unwinding*, which means Rust walks back up the stack and cleans up the data from each function it encounters.
- Rust allows you to choose the alternative of *unwinding* to immediately *aborting*, which ends the program without cleaning up.
- Memory that the program was using will then need to be cleaned up by the operating system.
- If in your project you need to make the resultant binary as small as possible, you can switch from *unwinding* to *aborting* upon a panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in your `Cargo.toml` file.
- If you want to abort on panic in release mode, add this:

    ```toml
    [profile.release]
    panic = 'abort'
    ```

- We can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error.
- A backtrace is a list of all the functions that have been called to get to this point.
- The key to reading the backtrace is to start from the top and read until you see files you wrote. That's the spot where the problem originated.

## Recoverable Errors with `Result`

- The `Result` enum is defined as having two variants, `Ok` and `Err`.

  ```rust
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ```

- The `T` and `E` are generic type parameters:
  - `T` represents the type of the value that will be returned in a success case within the `Ok` variant.
  - `E` represents the type of the error that will be returned in a failure case within the `Err`variant.

### Shortcuts for Panic on Error: `unwrap` and `expect`

- The `unwrap` method is a shortcut method implemented just like the `match` expression.
  - If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`
  - If the `Result` value is the `Err` variant, `unwrap` will call the `panic!` macro for us.
- The `expect` method lets us also choose the `panic!` error message.
- Using `expect` instead of `unwrap` and providing good error messages can coney your intent and make tracking down the source of a panic easier.
- We use `expect` in the same way as `unwarp`: to return file handle or call the `panic!` macro.
- **NOTE**: In production-quality code, most Rustaceans choose `expect` rather than `unwrap` and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

### Propagating Errors

WIP
