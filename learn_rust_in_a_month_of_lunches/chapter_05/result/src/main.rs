// Result looks similar to Option, but here is the difference:
// - Option holds a Some or None (value or no value).
// - Result holds an Ok or Err (okay result or error result).

// Result<T, E> means you need to **think** of what you want to return for Ok
// and what you want to return for Err. In fact, you can return anything you
// like. Even returning a () in each case is okey.

fn see_if_number_is_even(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(());
    } else {
        return Err(());
    }
}

fn main() {
    if see_if_number_is_even(5).is_ok() {
        println!("It's okey, guys")
    } else {
        println!("It's an error, guys")
    }
}
