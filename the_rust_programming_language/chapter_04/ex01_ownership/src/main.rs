// NOTE: Ownership
// - Ownership is a set of rules that govern how a Rust program manages memory.
// - Memory is managed through a system of ownership with a set of rules that
//   the compiler checks.
// - If any of the rules are violated, the program won’t compile.
// - Ownership Rules
//   - Each value in Rust has an owner.
//   - There can only be one owner at a time.
//   - When the owner goes out of scope, the value will be dropped.
// - Rust has a feature for using a value without transferring ownership,
//   called references. 😁

// NOTE: The Stack
// - The stack stores values in the order it gets them and removes the values
//   in the opposite order.
// - This is referred to as last in, first out.
// - Adding data is called pushing onto the stack.
// - Removing data is called popping off the stack.
// - All data stored on the stack must have a known, fixed size.
// - Data with an unknown size at compile time or a size that might change must
//   be stored on the heap instead.

// NOTE: The Heap
// - The heap is less organized: when you put data on the heap, you request a
//   certain amount of space.
// - The memory allocator finds an empty spot in the heap that is big enough,
//   marks it as being in use, and returns a pointer, which is the address of
//   that location.
// - Because the pointer to the heap is a known, fixed size, you can store the
//   pointer on the stack, but when you want the actual data, you must follow
//   the pointer.

// NOTE: Variable Scope
// - A scope is the range within a program for which an item is valid.
// - When a variable goes out of scope, Rust calls a special function for us.
//   This function is called drop.

// NOTE: The String Type
// - This type manages data allocated on the heap and as such is able to store
//   an amount of text that is unknown to us at compile time.
// - You can create a String from a string literal using the `from` function.
// - A String is made up of three parts:
//   - A pointer to the memory that holds the contents of the string.
//   - A length, how much memory, in bytes, the contents of the `String` are
//     currently using.
//   - A capacity, the total amount of memory, in bytes, that the `String` has
//     received from the allocator.

// NOTE: Variables and Data Interacting with Clone
// - If we do want to deeply copy the heap data of the `String`, not just the
//   stack data, we can use a common method called `clone`.

// NOTE: Stack-Only Data: Copy
// - If a type implements the `Copy` trait, variables that use it do not move.
// - Rust won’t let us annotate a type with `Copy` if the type, or any of its
//   parts, has implemented the `Drop` trait.
// - Any group of simple scalar values can implement `Copy`
//   - All the integer types
//   - All the floating-point types
//   - The Boolean type
//   - The character type
//   - Tuples, if they only contain types that also implement `Copy`.

// NOTE: Ownership and Functions
// - The mechanics of passing a value to a function are similar to those when
//   assigning a value to a variable.
// - Passing a variable to a function will move or copy, just as assignment
//   does.

// NOTE: Return Values and Scope
// - Returning values can also transfer ownership.
// - Rust does let us return multiple values using a tuple.

fn main() {
    let _s = "hello";

    // ---

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // ---

    let x = 5;
    let y = x; // Copy x value to y
    println!("{x} {y}");

    // The variable `s2` that has a copy of the pointer, length, and capacity
    // of `s1` and Rust considers `s1` as no longer valid.
    // Rust invalidates the first variable (`s1`), we would say that `s1` was
    // moved into `s2`.
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");

    // ---

    // When you assign a completely new value to an existing variable,
    // Rust will call drop and free the original value’s memory immediately.
    let mut s = String::from("hello");
    s = String::from("ahoy");
    println!("{s}, world!");

    // ---

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // ---

    let s = String::from("hello");
    takes_ownership(s); // value moves into the function and longer valid here.

    let x = 5;
    makes_copy(x); // i32 is Copy, so it's okay to still use `x` afterward.
    println!("{x}");

    // ---

    let s1 = gives_ownership();
    let s2 = String::from("hello"); // สังเกตว่า s2 จะใช้ไม่ได้แล้วเพราะถูก move ไปแล้ว
    let s3 = takes_and_gives_back(s2);
    println!("{s1} {s3}");

    // ---

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    // it will move its return value into the function that calls it.
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string is returned and moves out to the calling function.
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
