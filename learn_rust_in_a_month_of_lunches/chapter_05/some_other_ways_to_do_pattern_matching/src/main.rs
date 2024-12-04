// NOTE: if let means "do something if it matches, and don't do anything if it
// doesn't" if let is for when you **don't care about matching for everything**

fn main() {
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0);
    let get_two = my_vec.get(10);

    println!("{:?}", get_one);
    println!("{:?}", get_two);

    println!("{}", ["-"; 50].join(""));

    for index in 0..10 {
        match my_vec.get(index) {
            Some(number) => println!("The number is: {number}"),
            None => {}
        }
    }

    println!("{}", ["-"; 50].join(""));

    // Using if-let
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {number}");
        }
    }

    println!("{}", ["-"; 50].join(""));

    // Using let-else
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {number}");
        }

        let Some(number) = my_vec.get(index) else {
            continue;
        };

        println!("The number is: {number}");
    }
}
