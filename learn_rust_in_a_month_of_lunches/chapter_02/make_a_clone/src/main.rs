fn get_length(input: String) {
    println!(
        "Clone: It's words {} long.",
        input.split_whitespace().count()
    );
}

// Better version with ref
fn get_length_ref(input: &String) {
    println!("Ref: It's words {} long.", input.split_whitespace().count());
}

fn main() {
    let mut my_string = String::new();

    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length(my_string.clone());
        get_length_ref(&my_string); // Instead of 50 clones, it’s zero.
    }
}

// NOTE: Here’s a good rule of thumb with references and functions:
// If you can use an immutable reference, go with that.
// You won’t have to worry about a function taking ownership of some data:
// The function will simply take a look at it and be done.
// For functions, if you don’t need to transfer ownership, a reference is always easiest!
