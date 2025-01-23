# Shadowing

- You can declare a new variable with the same name as a previous variable.
- The first variable is *shadowed* by the second.
- The second variable is what the compiler will see when you use the name of the variable.
- The second variable overshadows the first, taking any use of the variable name to itself until either it itself is shadowed or the scope ends.
- We can shadow a variable by using the same variable's name and repeating the use of the `let` keyword.
- Shadowing is different from making a variable as `mut` because we'll get a compile-time error if we accidentally try to reassign to this variable without the `let` keyword.
- By using `let`, we can perform a few transformations on a value but have the variable be **immutable** after those transformations have been completed.
- We can change the type of the value but reuse the same name. (`mut` cannot)
