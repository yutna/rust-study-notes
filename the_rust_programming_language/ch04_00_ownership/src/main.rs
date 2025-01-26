fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{s}");

    // Variables and data interactive with Move
    let s1 = String::from("hello");
    let _s2 = s1;

    // println!("{s1}, world!");

    // Scope and Assignment
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // Variables and data interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}
