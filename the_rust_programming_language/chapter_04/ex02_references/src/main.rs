// NOTE: References
// - A reference is *like* a pointer in that it’s an address we can follow to
//   access the data stored at that address; that data is owned by some other
//   variable.
// - Unlike a pointer, a reference is guaranteed to point to a valid value of a
//   particular type for the life of that reference.
// - Reference allow you to refer to some value without taking ownership of it.
// - `&` (ampersands) represent references.
// - The opposite of referencing by using `&` is dereferencing, which is
//   accomplished with the dereference operator, `*`.
// - We call the action of creating a reference *borrowing*`

// NOTE: Mutable References
// - Mutable references have one big restriction: if you have a mutable
//   reference to a value, you can have no other references to that value.
// - The restriction preventing multiple mutable references to the same data at
//   the same time allows for mutation.
// - The benefit of having this restriction is that Rust can prevent data races
//   at compile time.
// - We also cannot have a mutable reference while we have an immutable one to
//   the same value.
// - BIG NOTE: a reference’s scope starts from where it is introduced and
//   continues through the **last time** that reference is **used**.

// NOTE: Dangling References
// - A pointer that references a location in memory that may have been given to
//   someone else—by freeing some memory while preserving a pointer to that
//   memory. (In languages with pointers)
// - In Rust, by contrast, the compiler guarantees that references will never
//   be dangling references: if you have a reference to some data, the compiler
//   will ensure that the data will not go out of scope before the reference to
//   the data does.

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // ---

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    // ---

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{} {}", r1, r2);

    // ---

    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;

    // NOTE: Rust enforces a similar rule for combining mutable and immutable
    // references. This code results in an error:
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // ---

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");
    // The scopes of the immutable references r1 and r2 end after the println!
    // where they are last used, which is before the mutable reference r3 is
    // created. These scopes don’t overlap, so this code is allowed

    // ---
    let _reference_to_nothing = dangle();
}

// ใช้ reference แทนที่จะ take owership
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
