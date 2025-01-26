# Ownership

- *Ownership* is a set of rules that govern how a Rust program manages memory.
- It enables Rust to make memory safety guarantees without needing a garbage collector.
- Memory is managed through a system of ownership with a set of rules that the compiler checks.
- If any of the rules are violated, the program won't compile.
- None of the features of ownership will slow down your program while it's running.

## The Stack and the Heap

- Both the stack and the heap are parts of memory available to your code to use at runtime.
- The stack stores values in the order it gets them and removes the values in the opposite order. (last in, first out)
- All data stored on the stack **MUST** have a known, fixed size.
- Data with an *unknown* size at compile time or a size that might change must be stored on the heap instead.
- When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
- The pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
- Pushing to the stack is faster than allocating on the heap.
- Accessing data in the heap is slower than accessing data on the stack.

## Ownership Rules

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope

- A scope is the range within a program for which an item is **VALID**.
  - When a variable comes in to scope, it is valid.
  - It remains valid until it goes out of scope.

## The `String` type

- String literals are immutable.
- The `String` type manages data allocated on the heap and as such is able to store an amount of text that is unkown to us at compile time.
- You can create a `String` from string literal using the `from` function, like so: `let s = String::from("hello")`. (This String type can be mutated)
- A `String` is made up of three parts:
  1. A pointer
  2. A length
  3. A capacity
- The length is how much memory, in bytes, the contents of the `String` are currently using.
- The capacity is the total amount of memory, in bytes, that the `String` has received from the allocator.

## Memory and Allocation

- In Rust, the memory is automatically returned once the variable that owns it goes out of scope.
- When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`.
- Rust calls `drop` automatically at the closing curly bracket.

## Variables and Data Interacting with Move

- When we assign a variable from `x` to `y`, we would say that `x` was *moved* into `y`.
- Rust will never automatically create *deep* copies of your data.
- Any *automatic* copying can be assumed to be inexpensive in terms of runtime performance.

## Scope ans Assignment

- When you assign a completely new value to an existing variable, Rust will call `drop` and free the original value's memory immediatly.


## Variables and Data Interacting with Clone

- If we *do* want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

## Stack-Only Data: Copy

- The types that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. (no move)
- Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack.
- If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
- Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.
- Here are some of the types that implement `Copy`:
  1. All the integer types
  2. The Boolean type.
  3. All the floating-point types.
  4. The character type. (char)
  5. Tuples, if they only contain types that also implement `Copy`. (i32, i32) -> Yes, (i32, String) -> No

## Ownership and Functions

- Passing a variable to a function will **MOVE** or **COPY**, just as assignment does.

## Summary

The ownership of a variable follows the same patterns every time:

|> assigning a value to another value moves it.

When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.
