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

## Lifetime Elision

- Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.
- The compiler uses three rules to figure out the lifetimes of the reference when there aren't explicit annotations.
  1. The compiler assign a lifetime parameter to each parameter that's a reference.
  2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
  3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.

จากหัวข้อนี้ถ้าเราติดปัญหาในการเขียน lifetime หรืออยากทบทวนอีกครั้งให้กลับไปอ่าน paragraph ที่ขึ้นต้นด้วยว่า *Let’s pretend we’re the compiler.* อีกครั้งจะเข้าใจหลัการในการใช้งาน lifetime มากขึ้น

## Lifetime Annotations in Method Definitions

- Lifetime names for struct fields always need to be declared after the `impl` keyword and the used after then struct’s name because those lifetimes are part of the struct’s type.
- In method signatures inside the `impl` block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.

## The Static Lifetime

- `'static` lifetime denotes that the affected reference can live for the entire duration of the program.
- All string literals have the `'static` lifetime, which we can annotate as follows: `let s: &'static str = "I have a static lifetime."`
- You might see suggestions to use the `'static` lifetime in error messages. But before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to.
- *Most of the time*, an error message suggesting the `'static` lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is to fix those problems, **NOT** to specify the `'static` lifetime.

## Generic Type Parameters, Trait Bounds, and Lifetimes Together

```rust

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str {

}
```
