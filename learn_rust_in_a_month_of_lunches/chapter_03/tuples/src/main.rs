// NOTE: Items inside a tuple are also accessed with numbers 0, 1, 2 and so on.
// But to access them, you use a . (dot) instead of []. There is a good reason
// for this: tuples are more like objects than indexed collections.

fn main() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!("Inside the tuple is:");
    println!("First item: {:?}", random_tuple.0);
    println!("Second item: {:?}", random_tuple.1);
    println!("Third item: {:?}", random_tuple.2);
    println!("Fourth item: {:?}", random_tuple.3);
    println!("Fifth item: {:?}", random_tuple.4);
    println!("Six item: {:?}", random_tuple.5);

    // NOTE: You can use a tuple to create multiple variables at the same time.
    let strings = ("one".to_string(), "two".to_string(), "three".to_string());
    let (_, b, _) = strings;
    println!("{b}");

    // NOTE: Destructuring only works when the pattern matches.
    // let tuple_of_three = ("one", "two", "three");
    // let (a, b) = tuple_of_three ❌
}
