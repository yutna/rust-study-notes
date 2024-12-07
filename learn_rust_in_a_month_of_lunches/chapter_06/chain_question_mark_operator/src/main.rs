use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?;

    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn turn_into_string_and_parse(bytes: Vec<u8>) -> i32 {
    let as_string = String::from_utf8(bytes).unwrap();
    let as_num = as_string.parse::<i32>().unwrap();
    as_num
}

fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }

    println!("{}", ["-"; 50].join(""));

    let num = turn_into_string_and_parse(vec![49, 53, 53]);
    println!("{num}");
}
