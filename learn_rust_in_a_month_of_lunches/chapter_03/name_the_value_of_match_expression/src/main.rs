// NOTE: You can also use @ to give a name to the value of a match expression,
// and then you can use it.

fn match_number(input: i32) {
    match input {
        number @ 4 => println!("{number} is unlucky in China"),
        number @ 13 => println!("{number} is lucky in Italy!"),
        number @ 14..=19 => println!("Some other number that ends with -teen: {number}"), // wow ใช้ท่านี้ได้ด้วยแหะ 😮
        _ => println!("Some other number, I guess"),
    }
}

fn main() {
    match_number(50);
    match_number(13);
    match_number(16);
    match_number(4);
}
