// NOTE: That is useful if you want to always have a value that you want to choose.
// If you do this, it will never panic

fn main() {
    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or(&0);
    println!("{fourth}");
}
