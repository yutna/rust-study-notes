// NOTE: Sometimes a function with Result will use a String for the Err value.
// This is NOT a true error type yet, but it contains some information.

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err(format!("Sorry, bad number. Expected: 5 Got: {number}")),
    }
}

fn main() {
    for number in 4..=7 {
        println!("{:?}", check_if_five(number));
    }
}
