use std::collections::HashMap;

fn main() {
    let numbers = vec![3, 1, 4, 2, 5];
    let median = get_median(&numbers);
    let mode = get_mode(&numbers);

    println!("Median = {median}, Mode = {mode}");
}

fn get_median(numbers: &Vec<i32>) -> f64 {
    let mut numbers = numbers.clone();

    numbers.sort();

    let n = numbers.len();
    let is_even = n % 2 == 0;

    if is_even {
        let middle_left = numbers[(n / 2) - 1];
        let middle_right = numbers[n / 2];

        return ((middle_left + middle_right) as f64) / 2.0;
    }

    numbers[n / 2] as f64
}

fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut hash_number = HashMap::new();

    for number in numbers {
        let count = hash_number.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut mode = (0, 1);

    for (key, value) in hash_number {
        if value > mode.1 {
            mode = (key, value);
        }
    }

    mode.0
}
