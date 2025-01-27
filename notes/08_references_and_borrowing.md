# References and Borrowing

- A *reference* is like a pointer in that it's an address we can follow to access the data stored at that address.
- Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- The ampersands (`&`) represent *references*, and they allow you to refer to some value without taking ownership.
- The opposite of referencing by using `&` is *dereferencing*, which is accomplished with the dereference operator, `*`.
- We call the action of creating a reference *borrowing*.
- Just as variables are immutable by default, so are references. We're **NOT** allowed to modify something we have a reference to.
- A reference's scope starts from where it is introduced and continues through the last time that reference is used.

## Mutable References

- Mutable references have one big restriction: if you have a mutable reference to a value, you can have **NO** other references to that value.
- We also **CANNOT** have a mutable reference while we have an immutable one to the same value.

## Danling References

- A pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory.
- In Rust, by contrast, the compiler guarantees that references will never be dangling references.

## The Rules of References

- At any given time, you can have either one mutable reference or any number of immutable references.
- References **MUST** always be valid.
