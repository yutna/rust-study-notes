# Generic Types

- Generics allow us to replace specific types with a placeholder that represents multiple types to remove code
  duplication.
- You can use as many generic type parameters in a definition as you want, but using more than a few makes your code
  hard to read.
- If you're finding you need lots of generic types in your code, it could indicate that your code needs restructuring
  into smaller pieces.

## In Function Definitions

- When defining a function that using generics, we place the generic in the signature of the function where we would
  usually specify the data types of the parameters and return value.
- To define the generic type function, we place type name declarations inside angle brackets, `<>`, between the name of
  the function and the parameter list.
- The `T` is the default choice of most Rust programmers.

```rust
fn largest<T>(list: &[T]) -> &T {}
```

## In Struct Definitions

- We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

## In Enum Definitions

- We can define enums to hold generic data types in their variants (e.g. `Option<T>`, `Result<T, E>`).
- When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of
  the values they hold, you can avoid duplication by using generic types instead.

## In Method Definitions

- We can implement methods on structs and enums and use generic types in their definitions too.
- We **HAVE TO** declare `T` just after `impl` so we can use `T` to specify that we're implementing methods on the type
  `Point<T>`.
- By declaring `T` as a generic type after `impl` , Rust can identify that the type in the angle brackets in `Point` is
  a generic type rather than a concrete type.
- We can also specify constraints on generic types when defining methods on the type, for example, implement methods
  only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type. (meaning we don't declare
  any types after `impl`)
- From the example code below, this means that we can declare both generic types and constraint methods at the same
  time.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This code means the type Point<f32> will have a distance_from_origin method; other instances of Point<T> where T is
// not of type f32 will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

- Generic type parameters in a struct definition aren't always the same as those you use in that same struct's method
  signatures.

```rust
struct MPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with `impl`  
// and some are declared with the method definition.
impl<X1, Y1> MPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MPoint<X2, Y2>) -> MPoint<X1, Y2> {
        MPoint {
            x: self.x,
            y: other.y,
        }
    }
}
```

## Performance of Code Using Generics

- Using generic types won't make your program run any slower than it would with concrete types.
- Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using
  generics. When the code runs, it performs just as it would if we had duplicated each definition by hand.
