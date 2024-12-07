// NOTE: Rust has a `panic!()` macro that you can use to make it panic.

fn print_all_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three items");
    }

    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn main() {
    // panic!("Time to panic!");

    let my_vec = vec![8, 9, 10, 10, 55, 99];
    print_all_three_things(my_vec);
}
