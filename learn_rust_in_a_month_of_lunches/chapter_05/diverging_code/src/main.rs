// NOTE: Diverging code is basically any code that lets you escape before going
// to the next line.

// NOTE: You can write as much as you want inside the block after else, as long
// as you cand wit diverging code.

fn main() {
    let my_vec = vec![2, 3, 4];

    for index in 0..10 {
        let Some(number) = my_vec.get(index) else {
            println!("Looks like we got a None!");
            println!("We can still do whatever we want inside this block");
            println!("We just have to end with 'diverging code'");
            print!("Because after this block, ");
            println!("the variable 'number' has to exist");
            println!("Time to break the loop now, bye");
            break;
        };

        println!("The number is {number}");
    }
}
