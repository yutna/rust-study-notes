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

## Debugging Struct

- Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.
- The `Debug` trait enables us to print our struct in a way that is useful for developers so we can see its value while we're debugging our code.
- Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition.
- We can use `{:#?}` instead of `{:?}` for pretty output format.
- We can use the `dbg!` macro, which takes ownership of an expression, prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression.

## Method Syntax

- Methods are similar to functions:
  - We declare them with the `fn ` keyword and a name.
  - They can have parameters and a return value
  - They contain some code that's run when the method is called from somewhere else.
- Unlike functions:
  - Methods are defined within the context of a struct. (or an Enum, or a Trait object too)
  - Their first parameter is always `self`, which represents the instance of the struct the method is being called on.
- To define the function within the context of struct, we start an `impl` (Implementation) block for struct.
- Everything within this `impl` block will be associated with the struct type.
- The method syntax goes after an instace: we add a dot followed by the method name, parentheses, and any arguments. (`rect.area()`)
- In the method sigunature, the `&self` is actually short for `self: &Self`.
- Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for.
- We still need to use the `&` in front of the `self` shorthand to indicate that this method borrows the `Self` instance.
- Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably, just as they can any other parameter.
- If you choose `&self` in the first parameter that means: we don't want to take **OWNERSHIP**, and we just want to **READ** the data in the struct.
- If we wanted to change the instance, we'd use `&mut self` as the first parameter.
- Having a method that takes ownership of the instance by using just `self` as the first parameter is **RARE**; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.
- We can choose to give a method the same name as one the struct's fields.
  - With parentheses, Rust knowns we mean the method.
  - Without parentheses, Rust knows we mean the field.
  - Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called `getters`, and Rust does **NOT** implement them automatically for struct fields as some other languages do.
  - Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type's public API.
- Methods can take multiple parameters that we add to the signature after the `self` parameter, and those parameters work just like parameters in functions.

## Associated Functions

- All functions defined within an `impl` block are called *associated functions*.
- Because they are associated with the type named after the `impl`.
- We can define associated functions that **DO NOT HAVE** `self` as their first parameter (and thus are **NOT** methods) because they don't need an instance of the type to work with.
- Associated functions that are **NOT** methods are often used for constructors that will return a new instance of the struct.
- These are often called `new`, but `new` is **NOT** a special name and is **NOT** built into the language.
- The `Self` keywords in the return type and in the body of the function are aliases for the type that appears after the `impl` keyword.
- To call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);`.
- The `::` syntax is used for **BOTH** associated functions and namespaces created by modules.

## Multiple `impl` Blocks

- Each struct is allowed to have multiple `impl` blocks.
