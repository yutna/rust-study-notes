fn main() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity());

    num_vec.push('a');
    println!("{}", num_vec.capacity());

    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());

    num_vec.push('a');
    println!("{}", num_vec.capacity());

    println!("{}", ["-"; 20].join(""));

    // NOTE: If you think you know how many elements you need, you can use
    // Vec::with_capacity() to use less memory and make your program more
    // efficient.
    let mut num_vec = Vec::with_capacity(8);
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());
}
