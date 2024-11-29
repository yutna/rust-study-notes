fn main() {
    let my_number = 15;
    let single_reference = &my_number;
    let double_reference = &&my_number;
    let five_references = &&&&&my_number;

    println!("{:#?}", my_number);
    println!("{:#?}", single_reference);
    println!("{:#?}", double_reference);
    println!("{:#?}", five_references);

    // These are all different types
    // just in the same way that “a friend of a friend” is different from “a friend.”
    // In practice, you probably won’t see references that are five deep, but you will sometimes see a reference to a reference.
}
