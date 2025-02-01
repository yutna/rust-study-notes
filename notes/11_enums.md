# Enums

- Enums allow you to define a type by enumerating its possible *variants*.
- Enums provide a way to specify that a value can be one of a predefined set of values.
- The variants of an enum are namespaced under its identifier, and we use a double colon (`::`) to separate the two.
- We can include data directly within each enum variant.
- The name of an enum variant that includes data also acts as a function that constructs an instance of the enum (e.g., `IpAddr::V4("127.0.0.1")`).
- Each variant can store different types and amounts of associated data.
- You can store any kind of data inside an enum variant: strings, numeric types, structs, or even another enum.
- Just like structs, enums can have methods defined on them.

## The `Option` Enum and Its Advantages Over Null Values

- The `Option` type represents a common scenario in which a value may either be present or absent.
- Rust does **NOT** have a `null` feature like many other programming languages.
- `null` is a special value in some languages that indicates the absence of a valid value.
- In languages that support `null`, variables can either be `null` or non-null.
- The problem with `null` values is that using a `null` where a valid value is expected often results in runtime errors. Since `null` is widely used, such errors are common.
- While Rust does **NOT** have `null`, it provides an enum that encapsulates the concept of an optional value.
- This enum is `Option<T>`.
- The variants of `Option<T>`—`Some` and `None`—can be used directly without the `Option::` prefix.
- The `<T>` syntax indicates that `Some` can store a value of any type, and each concrete type used in place of `T` makes `Option<T>` a distinct type.
- Rust can infer the type if we provide a value inside the `Some` variant.
- If a variable is assigned only `None`, Rust requires explicit type annotation, such as:

  ```rust
  let absent_number: Option<i32> = None;
  ```

- When we have a `Some` value, it means that a valid value is present inside `Some`.
- When we have a `None` value, it signifies the absence of a value, similar to `null` in other languages.
- To allow a value to be `null`-like, you must explicitly opt in by using `Option<T>`.
- Anywhere a type is **not** an `Option<T>`, you can safely assume that the value is guaranteed to be non-null.
- To extract the inner value from a `Some` variant, use a `match` expression. (The Rust standard library also provides many useful methods for working with `Option<T>`—check them out!)
