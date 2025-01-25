# Functions

- Rust code uses `snake_case` as the conventional style for function names.
- We define a function in Rust by entering `fn` followed by a function name and a set of parentheses.
- The curly brackets tell the compiler where the function body begins and ends.
- We can call any function we've defined by entering its name followed by a set of parentheses.
- Rust does **NOT** care where you define your functions, only that they're defined somewhere in a scope that can be seen by the caller.

## Parameters

- We can define functions to have *parameters*, which are special variables that are part of a function's signature.
- When a function has parameters, you can provide it with concrete values for those parameters.
- The concrete values are called *arguments*.
- In function signatures, you **MUST** declare the type of each parameter.
- When defining multiple parameters, separate the parameter declarations with commas.

## Statements and Expressions

- **Statement** are instructions that perform some action and do **NOT** return a value.
- **Expressions** evaluate to a resultant value.
- **Expressions** can be part of statements.
- **Expressions** do **NOT** include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then **NOT** return a value.
- Rust is an expression-based language.
- Function bodies are made up of a series of statements optionally ending in an expression.
- Function definitions are also statements.
- Calling a function is **NOT** a statement, it is an expression.
- Calling a macro is an expression too.
- A new scope block created with curly brackets is an expression.

## Functions with Return Values

- Functions can return values to the code that call them.
- We don't name return values, but we must declare their type after an arrow (->).
- In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
- You can return early from a function by using the `return` keyword and specifying a value.
- Most functions return the last expression implicitly.
