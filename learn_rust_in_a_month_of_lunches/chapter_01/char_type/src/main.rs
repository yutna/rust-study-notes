fn main() {
    // let first_letter = 'A';
    // let space = ' ';
    // let other_language_char = 'ก';
    // let cat_face = '😻';

    // NOTE: ถ้า character เป็น ASCII เราสามารถ cast เป็น u8 ได้โดยใช้ `as`
    // let my_number = 100;
    // println!("{}", my_number as u8 as char);

    // NOTE: Be a little careful when casting into a smaller type!
    // When casting, make sure the old number isn’t larger than the new type’s largest possible number.
    // let my_number = 600;
    // println!("{}", my_number as u8);

    let my_number: u8 = 100;
    println!("{}", my_number as char);
}
