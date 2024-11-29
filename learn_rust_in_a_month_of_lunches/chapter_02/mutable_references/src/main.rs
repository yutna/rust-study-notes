// NOTE: If you want to use a reference to change data, you can use a mutable reference (&mut)
// Using * lets you move from the reference to the value behind the reference.
// * is the opposite of &. Also, one * erases one &.
// Because using & is called referencing, using * is called dereferencing.
// You can only have one mutable reference.
// You can’t have an immutable reference and a mutable reference together.

fn main() {
    // let mut my_number = 8;
    // let num_ref = &mut my_number;

    // You can’t write num_ref += 10 because num_ref is not the i32 value;
    // it is an &i32. There’s nothing to add inside a reference.
    // The value to add is actually inside the i32.

    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("{}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Are they equal? {}", second_number == ***triple_reference);

    // The problem version
    // let mut number = 10;
    // let number_ref = &number; // สังเกตไหมว่าเราสามารถ ref mut variable เฉยๆสำหรับเอาไว้อ่านค่าอย่างเดียว
    // let number_change = &mut number;
    // *number_change += 10;
    // println!("{}", number_ref);

    // No problem version
    // Why? This code have mutable ref, and immu ref togethter 🧐
    // The code works because the compiler is smart enough to understand it.
    // It knows that we used number_change to change number but didn’t use it again,
    // so that is the end of the mutable borrow. No problem!
    // We are not using immutable and mutable references together. 🤯
    // คือง่ายๆว่า ถ้า compiler สามารถตรวจสอบได้ว่าไม่มีการใช้ mut ref อีก เราสามารถใช้
    // immu ref เพื่อใช้อ่่านค่าอย่างเดียวได้
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);
}
