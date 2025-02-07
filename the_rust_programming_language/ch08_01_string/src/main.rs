fn main() {
    // Creating a new string.
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    let _s = "the method also works on a literal directly".to_string();

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // ---

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);
    println!("s2 is {s2}");

    // ---

    let mut s = String::from("lo");
    s.push('l');

    println!("{s}");

    //  Concatenation with the `+` Operator or the `format!` Macro

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // NOTE: s1 has been moved here and can no longer be used.

    println!("{s3}");

    // ---

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    // Indexing into Strings

    // let s1 = String::from("hello");
    // let h = s1[0];

    // Iterating over strings
    for c in "สวัสดีครับ".chars() {
        println!("{c}");
    }

    for b in "สวัสดีครับ".bytes() {
        println!("{b}");
    }
}
