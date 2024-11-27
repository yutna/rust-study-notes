fn main() {
    // NOTE: these types are not officially called floats;
    // they are called f32 and f64.
    // let _my_float = 5.;

    // NOTE: only floats of the same type can be used together.
    // You can’t add an f32 to an f64.
    // let my_float: f64 = 5.0;
    // let my_other_float: f32 = 8.5;
    // let _third_float = my_float + my_other_float as f64;

    // NOTE: The Rust compiler is pretty smart and will not make an f64 if we
    // declare an f32 and try to add it to another float
    // สังเกตไหม เมื่อเทียบกับตัวอย่างข้างบน ข้างบน f64 + f32 จะ error คือเมื่อเราประกาศ type
    // แบบ explicit แต่ถ้าเราไม่ได้ประกาศ type แบบตัวอย่างนี้ (my_other_float) ถึง
    // default มันจะเป็น f64 ตอนแรก แล้วเอามาบวกกับตัว explicit type ตัว Rust compiler
    // จะเปลี่ยน type ของตัว my_other_float ให้เองเลยจาก f64 เป็น f32
    let my_float: f32 = 5.0;
    let my_other_float = 8.5;
    let _third_float = my_float + my_other_float;
}
