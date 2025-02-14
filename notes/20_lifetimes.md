# Lifetimes

- Lifetimes are another kind of generic.
- Lifetimes ensure that references are valid as long as we need them to be.
- Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
- Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
- Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at rmtime will definitely be valid.
- The main aim of lifetimes is to prevent *dangling references*, which cause a program to reference data other than the data it's intended to reference.
- The Rust compiler has a *borrow checker* that compares scopes to determine whether all borrows are valid.

## Lifetime Annotation Syntax

- Just as functions can accept any type when the signature specifies a generric type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
- Lifetime annotation: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types.
- Most people use the name `'a` for the first lifetime annotation.'
- We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference's type.

```rust
&i32;       // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

- One lifetime annotation by itself doesn’t have much meaning because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.

## Lifetime Annotations in Function Signatures

- To use lifetime annotations in function signature, we need to declare the generic *lifetime* parameters inside angle brackets between the function name and the parameter list, just as we did with generic type annotation.
- When we specify the lifetime parameters in the function signature, we're not changing the lifetimes of any values passed in or returned. Rather, we're specifying that the borrow checker should reject any values that don't adhere to these constraints.
- When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.

## Thinking in Terms of Lifetimes

- When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
- If the reference returned does **NOT** refer to one of the parameters, it must refer to a value created within this function.
- Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.

## Lifetime Annotations in Struct Definitions

- We can define structs to hold references. (ก่อนหน้าบทนี้เรา define structs ด้วย concrete types เพียงอย่างเดียว)
- We would need to add a lifetime annotation on every reference in the struct's definition.
- As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition.
