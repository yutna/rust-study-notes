// NOTE: Sometimes you see |_| in a closure. 
// It is not a special syntax, though.
// It only means that the closure to take
// an argument that you give a name to 
// (like x or num), but you don't want to 
// use it.

fn main() {
    let my_vec = vec![8, 9, 10];
    my_vec
        .iter()
        .for_each(|_| println!("We didn't use the variables at all"))    
}
