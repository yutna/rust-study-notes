# Control Flow

- The most common constructs that let you control the flow of execution of Rust code are `if` expressions and loops.

## `if` Expression

- An `if` expression allows you to branch your code depending on conditions.
- You provide a condition and then state, *If this condition is met, run this block of code. If the condition is NOT met, do NOT run this block of code.*
- Blocks of code associated with the conditions in `if` expressions are sometimes called *arms*.
- Optionally, we can also include an `else` expression, to give the program an alternative block of code to execute should the condition evaluate to `false`.
- If you don't provide an `else` expression and the condition is `false`, the program will just skip the `if` block and move on to the next bit of code.
- The condition **MUST** be a `bool` type.
- Rust will **NOT** automatically try to convert non-Boolean types to a Boolean.
- You **MUST** be explicit and always provide `if` with a Boolean as its condition.
- You can use multiple conditions by combining `if` and `else` in an `else if` expression.
- Rust only executes the block for the first `true` condition, and once it finds one, it doesn't even check the rest.

## Using `if` in a `let` statement

- Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable.
- The values that have the potential to be results from each arm of the `if` **MUST** be the same type.

## Repetition with Loops

- Rust has three kinds of loops:
  1. `loop`
  2. `while`
  3. `for`

### Repeating Code with `loop`

- The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
- You can place the `break` keyword within the loop to tell the program when to stop executing the loop.
- You also can used `continue`, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.
- One of the uses of a `loop` is to retry an operation you know might fail.
- You can returning a value from the loop, To do this, you can add the value you want returned after the `break` expression you use to stop the loop; that value will be returned out of the loop so you can use it.
- **BIG NOTE**: You can also `return` from inside a loop. While `break` only exits the current loop, `return` always exits the current function.
- If you have loops within loops, `break` and `continue` apply to the innermost at that point.
- You can optionally specify a *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.
- Loop labels **MUST** begin with a single quote.

### Conditional Loops with `while`

- While a condition evaluates to `true`, the code runs; otherwise, it exits the loop.

### Looping Through a Collection with `for`

- You can use a `for` loop and execute some code for each item in a collection.
- Use `for` loop in situations in which you want to run some code a certain number of times.
