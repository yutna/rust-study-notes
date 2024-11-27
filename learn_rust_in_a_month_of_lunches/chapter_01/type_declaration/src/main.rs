fn main() {
    // NOTE: To specify a type, add a colon after the variable name:
    let _small_number: u8 = 10;

    // NOTE: For numbers, you can add the type after the number. You don’t need
    // a space—just type it right after the number:
    let _small_number = 10u8;

    // NOTE: You can also add _ if you want to make the number easy to read:
    let _small_number = 10_u8;
    let _big_number = 100_000_000_i32;

    // NOTE: The _ is only to make numbers easy for humans to read and does not
    // affect the number. It is completely ignored by the compiler. In fact,
    // it doesn’t matter how many _ you use:
    let number = 10_____________u8;
    let number2 = 1__2_____3_____________4____________i32;

    println!("{} {}", number, number2);
}
