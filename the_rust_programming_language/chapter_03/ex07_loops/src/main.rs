// NOTE: Repetition with Loops
// - Rust has three kinds of loops
//   - loop
//   - while
//   - for

// NOTE: Repeating Code with loop
// - The `loop` keyword tells Rust to execute a block of code over and over
//   again forever or until you explicitly tell it to stop.
// - You can place the `break` keyword within the loop to tell the program when
//   to stop executing the loop.
// - We also used `continue`, which in a loop tells the program to skip over
//   any remaining code in this iteration of the loop and go to the next
//   iteration.
// - If you have loops within loops, `break` and `continue` apply to the
//   innermost loop at that point.

// NOTE: Returning Values from Loops
// - You can add the value you want returned after the `break` expression you
//   use to stop the loop.
// - You can also `return` from inside a loop.
// - While `break` only exits the current loop, `return` always exits the
//   current function.

// NOTE: Loop Labels
// - Loop labels must begin with a single quote.
// - You can optionally specify a loop label on a loop that you can then use
//   with `break` or `continue` to specify that those keywords apply to the
//   labeled loop instead of the innermost loop.

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // สังเกตว่าในกรณีนี้มี ;
        }
    };

    println!("The result is {result}");

    // ---
    println!("{}", ["-"; 20].join(""));
    // ---

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // ---
    println!("{}", ["-"; 20].join(""));
    // ---

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // ---
    println!("{}", ["-"; 20].join(""));
    // ---

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", arr[index]);
        index += 1;
    }

    // ---
    println!("{}", ["-"; 20].join(""));
    // ---

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {element}");
    }

    // ---
    println!("{}", ["-"; 20].join(""));
    // ---

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
