use std::io;

fn main() {
    // Floating-Point example
    let _x = 2.0;
    let _y: f32 = 3.0;

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum: {sum}");
    println!("subtraction: {difference}");
    println!("multiplication: {product}");
    println!("division: {quotient}");
    println!("division (integer): {truncated}");
    println!("remainder: {remainder}");

    // The Boolean Type
    let _t = true;
    let _f: bool = false;

    // The Character Type
    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup; // Use pattern matching to destructure a tuple value
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred} {six_point_four} {one}");

    // The Array Type
    let _a = [1, 2, 3, 4, 5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("{:?}", a);

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
