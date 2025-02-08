use std::fs::File;
// use std::io::ErrorKind;

fn main() {
    // Manual call
    // panic!("crash and burn");

    // Backtrace example
    // let v = vec![1, 2, 3];
    // v[99];

    // Call a function that returns a `Result`
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");
    //         }
    //     }
    // };
    
    // Alternatives to Using `match` with `Result<T, E>`
    // let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });
    
    // The `unwrap` method
    // let _greeting_file = File::open("hello.txt").unwrap();
    
    // The `expect` method
    let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}
