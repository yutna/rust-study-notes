# Struct

- A *struct*, or *structure*, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
- Struct are similar to tuples, in that both hold multiple related values.
- Like tuples, the pieces of a struct can be different types.
- Unlike with tuples, in a struct you'll name each piece of data so it's clear what the values mean.
- Adding these names means that struct are more flexible than tuples: you don't have to rely on the order of the data to specify or access the values of an instance.

## Defining Structs

- To define a struct, we enter the keyword `struct` and name the entire struct.
- A struct's name should describe the significance of the pieces of data being grouped together.
- Inside curly brackets, we define the names and types of the pieces of data, which we call *fields*.

## Instantiating Structs

- To use a struct after we've defined it, we create an *instance* of that struct by specifing concrete values for each of the fields.
- We create an instance by stating the name of the struct and then add curly brackets containing *key: value* pairs.
- Where the keys are the names of the fields and the values are the data we want to store in those fields.
- We don't have to specify the fields in the same order in which we declared them in the struct.
- To get a specific value from a struct, we use dot notation.
- If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
- We can use the *field shorthand syntax* if the parameter names or variable names and the struct field names are exactly the same.

## Creating Instances from Other Instances with Struct Update Syntax

- It's often useful to create a new instance of a struct that includes most of the values from another instance, but change some.
- You can do this using *struct update syntax*.
- The syntax `..` specifies that the remaining fields **NOT** explicitly set should have the same value as the fields in the given instance.

## Using Tuple Structs Without Named Fields to Create Different Types

- Rust also supports structs that similar to tuples, called *tuple struct*.
- Tuple structs have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the type of the fields.
- Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct whould be verbose or redundant.
- To define a tuple struct, start with the `struct` keyword, and the struct name followed by the types in the tuple.
- Each struct you define is its own type, even though the fields within the struct might have the same types. e.g. `struct Color(i32, i32, i32), struct Point(i32, i32, i32)`, the `Color` and `Point` are **NOT** the same type.

## Unit-Like Structs Without Any Fields

- You can also define structs that don't have any field!
- These are called *unit-like structs* because they behave similarly to `()`.
- Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
- To define the unit-like struct, we use the `struct` keyword, the name we want, and then a semicolon. (no need for curly brackets or parentheses)

