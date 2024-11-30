// NOTE: There are two main ways to declare a vector.
// - Using new
// - Using vec! macro
// All items in a Vec must all the same type.

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    println!("{:#?}", my_vec);

    let my_vec = vec![8, 10, 10];
    println!("{:?}", my_vec);
}

