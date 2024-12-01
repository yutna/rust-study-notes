fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {counter}");
        if counter == 5 {
            break;
        }   
    }

    println!("{}", ["-"; 50].join(""));

    // NOTE: Rust allow you to give a loop name,
    // which is helpful when you are within a loop
    // that is inside another loop. 
    // You can use a ' (called a tick) followed by 
    // a colon to give it a name:
    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 5 {
            println!("Now entering the second loop.");
            'second_loop: loop {
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}

