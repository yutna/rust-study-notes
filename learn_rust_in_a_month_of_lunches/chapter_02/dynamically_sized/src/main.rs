fn main() {
    let size_of_string = std::mem::size_of::<String>(); // Give you the size of a type in bytes
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("A String is sized an always {size_of_string} bytes");
    println!("An i8 is sized and always {size_of_i8} bytes");
    println!("An f64 is sized and always {size_of_f64} bytes");
    println!("But a &str is not sized: '자우림' is {size_of_jaurim} bytes.");
    println!("And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not Sized");
}
