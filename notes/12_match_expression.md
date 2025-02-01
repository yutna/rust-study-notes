# The `match` Control Flow Construct

- `match` allows you to compare a value against a series of patterns and execute code based on which pattern matches.
- Patterns can consist of literal values, variable names, wildcards, and other elements.
- Each value is evaluated against the patterns in a `match` expression. The first pattern that the value "fits" determines the corresponding code block that will be executed.
- A `match` consists of multiple *arms*, where each arm has two parts:
  1. A pattern.
  2. A block of code.
- The `=>` operator separates the pattern from the code to execute.
- Each arm is separated from the next by a comma.
- The code associated with each arm is an expression, and the result of the matching arm's expression is the value returned by the entire `match` expression.
- Curly brackets are typically omitted if the match arm contains a short expression.
- If you want to include multiple lines of code within a match arm, you **MUST** use curly brackets. In this case, the comma following the arm becomes *optional*.
- **NOTE**: The patterns in the arms must **COVER ALL POSSIBILITIES**.
- Rust enforces *exhaustiveness* in `match` expressions: every possible case **MUST** be covered for the code to be valid. This is especially important when handling `Option<T>`.
- `_` is a special catch-all pattern that matches any value but does not bind to it.
- The `_ => ()` catch-all pattern is used when no specific match is found, and no action needs to be taken.

## Patterns That Bind to Values

- Match arms can bind to parts of values that match the pattern, allowing extraction of values from enum variants.

## Concise Control Flow with `if let`

- The `if let` syntax combines `if` and `let`, providing a more concise way to handle values that match a single pattern while ignoring all others.
- The `if let` syntax consists of a pattern and an expression separated by an equal sign (`=`).
- You can think of `if let` as syntactic sugar for a `match` expression that executes code only when a value matches a specific pattern while ignoring other cases.
- `if let` can include an `else` block.
- The `else` block behaves the same way as the `_` catch-all case in a `match` expression.
