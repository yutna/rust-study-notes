// NOTE: when you use a method, Rust will dereference for you until
// it reaches the original type.

fn main() {
    let my_name = "Billy".to_string();
    let double_ref = &&my_name;
    println!("{}", double_ref.is_empty());
    // without . you would have to write this
    // (&**double_ref).is_empty()
    //
    println!("{}", &&&&&&double_ref.is_empty());
}
