// NOTE: The Slice Type
// - Slices let you reference a contiguous sequence of elements in a collection
//   rather than the whole collection.
// - A slice is a kind of reference, so it does not have ownership.

// NOTE: String Slices
// - A string slice is a reference to part of a String.
// - We create slices using a range within brackets by specifying
//   `[starting_index..ending_index]`,
// - The type that signifies **string slice** is written as `&str`.
// - `&str` is an immutable reference.

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{}", word);

    // ---

    let s = String::from("hello world");
    // Rather than a reference to the entire String, hello is a reference to a
    // portion of the String.
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // ---

    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let _slice = &s[3..len];
    let _slice = &s[3..];

    let _slice = &s[0..len];
    let _slice = &s[..];

    // ---

    let my_mystring = String::from("hello world");

    // Works on slices of `String`s, whether partial or whole
    let _word = first_word_slice(&my_mystring[0..6]);
    let _word = first_word_slice(&my_mystring[..]);
    // Also works on references to `String`s, which are
    // equivalent to whole slices of `String`s
    let _word = first_word_slice(&my_mystring);

    let my_string_literal = "hello world";

    // Works on slices of string literals, whether partial or whole
    // สังเกต ถึงแม้จะเป็น &str อยู่แล้ว ถ้าต้องการ partial ก็ยังต้องใช้ slice syntax อยู่คือ &[..]
    let _word = first_word_slice(&my_string_literal[0..6]);
    let _word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_slice(my_string_literal);

    // ---

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
