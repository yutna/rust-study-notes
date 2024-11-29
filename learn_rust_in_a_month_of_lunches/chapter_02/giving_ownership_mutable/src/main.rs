fn adds_hungary(mut string_to_add_hungary_to: String) {
    string_to_add_hungary_to.push_str("-Hungary");
    println!("{}", string_to_add_hungary_to);
}

fn main() {
    let country = String::from("Austria");
    adds_hungary(country);
}
