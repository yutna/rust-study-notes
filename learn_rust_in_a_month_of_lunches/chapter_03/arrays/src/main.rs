// NOTE: Arrays must only contain the same type
// Arrays cannot change their size.
// Array type: [type; number].
// [&str, 1] and [&str, 2] are different types.

fn main() {
    let _array1 = ["One", "Two"];
    let _array2 = ["One", "Two", "Five"];

    // NOTE: If you want an array with all the same value, you can declare it by
    // entering the value, then a semicolon, and then the number of times you
    // need it to repeat:
    let my_array = ["a"; 5];
    println!("{:?}", my_array);

    // Get entries in an array with [index]
    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);

    // You can also get a slice of an array.
    // First, you need a & because the compiler doesn't know the size (a slice
    // can be any length, so it is not Sized).
    // Then you can use .. to show the range, A range between index 2 and 5,
    // for example, is 2..5 (5 means up to index 5 but not including it)
    // It's called exclusive.
    // In case you need inclusive, use ..= instead ([0..=2])
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_of_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..]; // 1.. means from index 1 until the end.
    let end_of_five = &array_of_ten[..5]; // ..5 means from the beginning up to but not including index 5.
    let everything = &array_of_ten[..]; // Using .. means to slice the whole array: beginning to end.
    let inclusive_range = &array_of_ten[2..=5];

    println!("Two to five: {two_of_five:?}");
    println!("Start at one: {start_at_one:?}");
    println!("End at five: {end_of_five:?}");
    println!("Everything: {everything:?}");
    println!("Inclusive range: {inclusive_range:?}");
}
