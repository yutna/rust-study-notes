# The Slice Type

- *Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection.
- A slice is a kind of reference. so it does **NOT** have ownership.

## String Slices

- A *string slice* is a reference to part of a `String`.
- The type that signifies *string slice* is written as `&str`.
- For example, given `let s = String::from("hello world");`, the slice `let hello = &s[0..5];` creates a reference to the substring "hello".

## Slice Syntax

- Slices are created using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice.

## Simplified Slice Notation

- Rust allows omission of the starting index to default to zero and the ending index to default to the length of the collection.
- For instance, `&s[..2]` is equivalent to `&s[0..2]`, and `&s[3..]` is equivalent to `&s[3..s.len()]`.

## String Literals as Slices

- String literals are immutable.
- `&str` is an immutable reference.

## String Slices as Parameters

- If we have a string slice, we can pass that directly.
- If we have a `String`, we can pass a slice of the `String` or a reference to the `String`.
- Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality.

## Slices in Other Collections

- While the chapter focuses on string slices, the concept applies to other collections like arrays. Slices can be used to reference a portion of an array without owning it.
