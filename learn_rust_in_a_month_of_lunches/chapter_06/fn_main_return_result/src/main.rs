use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let items = vec!["89", "8", "9.0", "eleven", "6060"];

    for item in items {
        let parsed = item.parse::<u32>()?;
        println!("{parsed}");
    }

    Ok(())
}
