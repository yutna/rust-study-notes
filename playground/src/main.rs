use std::num::ParseIntError;

fn main() {
    if let Ok(number) = parse_and_log_str("1") {
        println!("{}", number);
    }
}

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    println!("Number parsed successfully into {}", parsed_number);
    Ok(parsed_number)
}
