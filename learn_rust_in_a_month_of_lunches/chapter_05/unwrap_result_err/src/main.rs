// NOTE: Just like unwrapping a None for Option, using `.unwrap()` on Err will
// panic:

// NOTE: A Result is just a regular enum, so we can create one whenever we like.
// Both Option and Result and their variants are already in scope, so we can
// just write Err instead of Result::Err

fn main() {
    let error_value: Result<i32, &str> = Err("There was an error");
    error_value.unwrap();
}
