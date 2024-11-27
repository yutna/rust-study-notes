fn main() {
    // NOTE: When a character is part of a string, the string is encoded to use
    // the least amount of memory needed for each character.
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of : {}", " ".len());

    // NOTE: Be careful! The .len() method returns the number of bytes,
    // not the number of letters or characters.
    let str1 = "Hello!";
    println!("str1 is {} bytes", str1.len());

    let str2 = "안녕!";
    println!("str2 is {} bytes", str2.len());

    // NOTE: เราสามารถเช็ค bytes ของ character ใน string ได้ด้วย `as_bytes()`
    // อย่างเช่น "ß".len() ได้ 2 bytes แล้วแต่ละ bytes คือเลขอะไรบ้างเป็นต้น
    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "🦀".as_bytes());
}
