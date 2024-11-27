fn main() {
    let str1 = "Hello!";
    println!(
        "str1 is {} bytes and also {} characters",
        str1.len(),
        str1.chars().count()
    );

    let str2 = "안녕!";
    println!(
        "str2 is {} bytes and also {} characters",
        str2.len(),
        str2.chars().count()
    );
}
