// NOTE: One of the rules of values in Rust is that value can only have one owner.
// This makes references very useful for functions because
// you can give a function a quick view of some data without having to pass ownership.

/*
 * NOTE: Conclusion
 * fn function_name(variable: String) takes a String and owns it. If it doesn’t
 * return anything, then the variable dies inside the function.
 *
 * fn function_name(variable: &String) borrows a String and can look at it.
 * The variable doesn’t die inside the function.
 *
 * fn function_name(variable: &mut String) borrows a String and can change it.
 * The variable doesn’t die inside the function.
 */

fn print_country(country_name: &String) {
    println!("{}", country_name);
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {country_name}");
}

fn main() {
    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);

    let mut country = String::from("Austria");
    add_hungary(&mut country);
}
