# Vector

- The `Vec<T>` known as a vector.
- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
- Vectors can only store values of the same type.
- They are useful when you have a list of items.

## Creating a New Vector

- To create a new empty vector, we call the `Vec::new` function.
- We need to add a type annotation if you are using the `Vec::new` to the variable. (Rust does't know that kind of elements we intend to store)
- If you create a `Vec<T>` with initial values, then Rust will infer the type of value you want to store.
- When we create a vector to hold a specific tye, we can specify the type within angle brackets.
- Rust conveniently provides the `vec!` macro, which will create a new vector that holds the values you give it.

## Updating a Vector

- We can use the `push` method to add an element to it.

## Reading Elements of Vectors

- There are two ways to reference a value stored in a vector:
  1. Indexing
  2. By using the `get` method
- Vectors are indexed by number, starting at zero.
- Using `&` and `[]` gives us a reference to the element at the index value.
- When we use the `get` method with the index passed as an argument, we get an `Option<&T>` that we can use with `match`.

## Iterating Over the Values in a Vector

- We can use a `for` loop to get immutable references to each element in a vector.
- We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
- To change the value that the mutable reference refers to, we have to use the `*` dereference operator to get to the value in `i` before we can use the `+=` operator.

## Using an Enum to Store Multiple Types

- We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same types: that of the enum. Then we can create a vector to hold that enum and so, ultimately, hold different types.
- If you don't know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won't work.
