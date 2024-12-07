// fn get_fourth(input: &Vec<i32>) -> i32 {
//     // let fourth = input.get(3).unwrap();
//     let fourth = input.get(3).expect("Input vector needs at least 4 items");
//     *fourth
// }

fn try_two_unwraps(input: Vec<Option<i32>>) {
    // println!("Index 0 is: {}", input[0].unwrap());
    // println!("Index 1 is: {}", input[1].unwrap());

    println!(
        "Index 0 is: {}",
        input[0].expect("The first unwrap had a None!")
    );

    println!(
        "Index 1 is: {}",
        input[1].expect("The second unwrap has a None!")
    );
}

fn main() {
    //let my_vec = vec![9, 0, 10];
    // let _fourth = get_fourth(&my_vec);

    let vector = vec![None, Some(1000)];
    try_two_unwraps(vector);
}
