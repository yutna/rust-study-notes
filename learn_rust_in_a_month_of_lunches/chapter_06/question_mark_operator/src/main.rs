// NOTE: There is an even shorter way to deal with `Result`,
// shorter than `match` and even shorter than `if let`.
// It is called the `question mark operator` and you simply
// type `?` to use it. This will
// - Give what is inside the `Result` if it is `Ok`.
// - Pass the error back if it is `Err` (this is called an early return).
// In other words, it does almost everything for you.

// NOTE: The `?` operator work with `Option` too.

use std::num::ParseIntError;

// fn parse_and_log_str_with_match(input: &str) -> Result<i32, ParseIntError> {
//     let parsed_number = match input.parse::<i32>() {
//         Ok(number) => number,
//         Err(e) => return Err(e),
//     };

//     println!("Number parsed successfully into {parsed_number}");
//     Ok(parsed_number)
// }

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    // IF it does NOT parse successfully, the function
    // ends here and returns an error.
    let parsed_number = input.parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }
}
