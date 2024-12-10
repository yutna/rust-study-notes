// NOTE: iterators
// - .iter() --- For an iterator of references
// - .item_mut() --- For an iterator of mutable references
// - .into_iter() --- For an iterator of values (not references)
// A `for` loop is an iterator of values, so typing `for item in iterator`
// is the same as typing `for item in iterator.into_iter()`

fn main() {
    let vector1 = vec![1, 2, 3];
    let mut vector2 = vec![10, 20, 30];

    // First, we use `.iter()` so that vector1
    // is NOT destroyed.
    for num in vector1.iter() {
        println!("Printing a &i32: {num}");
    }

    // This is the same as writing
    // `for num in vector1.into_iter()`
    // It owns the values, and vector1 no
    // longer exists after this `for` loop
    // is done.
    for num in vector1 {
        println!("Printing an i32: {num}");
    }

    // This `for` loop takes mutable
    // references, so `vector2` still
    // exists after it is over.
    for num in vector2.iter_mut() {
        *num *= 10;
        println!("num is now {num}");
    }

    println!("{vector2:?}");
}
