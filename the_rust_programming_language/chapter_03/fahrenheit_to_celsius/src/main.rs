fn main() {
    let fahrenheit = 68.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);

    println!("{fahrenheit}F == {celsius}c");
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
