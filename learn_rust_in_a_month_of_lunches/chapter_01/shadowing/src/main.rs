// NOTE: Shadowing means using let to declare a new variable with the same name as another variable
// NOTE: Shadowing is completely different from mutability: it’s just a variable with the same name that blocks the other one

fn main() {
    let my_number = 8;
    println!("{}", my_number);

    // Here we say that we “shadowed” my_number with a new “let binding.”
    // The variable my_number is now pointing to a completely different value
    // So, is the first my_number destroyed? No, but when we call my_number,
    // we now get my_number the f64.
    // let my_number = 9.2;
    // println!("{}", my_number);

    // So, when you shadow a variable with a new variable with the same name,
    // you don’t destroy the first one. You block it
    {
        let my_number = 9.2;
        println!("{}", my_number);
    }
    println!("{}", my_number);

    /*
     * NOTE: What is the advantage of shadowing? Shadowing is good when you need to
     * work on a variable a lot and you don’t care about it in between
     */
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x
    };
    println!("{}", final_number);

    // NOTE: Shadowing can be useful when working with mutability, too
    let x = 9;
    let mut x = x as f32;
    x += 0.5;
    println!("{}", x);
}

fn times_two(number: i32) -> i32 {
    number * 2
}
