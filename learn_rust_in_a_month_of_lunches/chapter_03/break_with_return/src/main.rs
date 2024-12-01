fn main() {
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter; // NOTE: You write the return value after break and use a semicolon ;
        }
    };

    println!("{my_number}");
}

