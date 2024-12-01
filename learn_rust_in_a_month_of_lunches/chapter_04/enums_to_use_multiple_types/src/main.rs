// NOTE: Enum can carry data, and that mean...
// you can use an enum to hold different types inside a collection.
// ตรงนี้ถ้ากลับมาดูแล้วไม่เข้าใจหรือลืมว่ามันคืออะไรกันแน่ให้กลับไปอ่านในหนังสืออีกรอบ มันค่อนข้างจะอธิบายยาว
// ซึ่งหัวข้อชื่อเดียวกับ folder เลย
enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };

    number
}

fn main() {
    let my_vec = vec![get_number(-100), get_number(0)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("A u32 with the value {number}"),
            Number::I32(number) => println!("An i32 with the value {number}"),
        }
    }
}
