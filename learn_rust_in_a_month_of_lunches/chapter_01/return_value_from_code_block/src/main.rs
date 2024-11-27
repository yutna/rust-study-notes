fn main() {
    let my_number = {
        let second_number = 8;
        second_number + 9 // No semicolon, so the code block returns 8 + 9. It works just like returning from a function.
    };

    println!("My number is {}", my_number);
}
