fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {} ", number_one, number_two, result);
}

fn multiply2(number_one: i32, number_two: i32) -> i32 {
    number_one * number_two
}

fn main() {
    multiply(8, 9);

    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);

    let multiply_result = multiply2(8, 9);
    println!("The two numbers multiplied are: {}", multiply_result);
}
