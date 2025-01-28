fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    println!("{} {}", word, s.len());

    // ---

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // ---

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    println!("{slice}");

    // ---

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    println!("{slice}");

    // ---

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    println!("{slice}");

    // ---

    let mut s = String::from("hello world");
    let word = first_word_slice(&s);

    println!("{s} {word}");

    // ---

    // let mut s = String::from("hello world");
    // let word = first_word_slice(&s);

    // s.clear();

    // println!("the first word is: {word}");

    // ---

    let my_string = String::from("hello world");

    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    let word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    let word = first_word_slice(my_string_literal);

    // ---

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

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
            return &s[0..i];
        }
    }

    &s[..]
}
