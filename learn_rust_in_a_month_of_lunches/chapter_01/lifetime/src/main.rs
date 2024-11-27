fn main() {
    // lifetime ของฟังก์ชั่น main
    let _my_float = 5.0;
    {
        // lifetime เฉพาะ code block นี้เท่านั้น
        let _my_other_float = 8.5;
    }
    // let third_float = my_float + my_other_float;
}
