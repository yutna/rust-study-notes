fn return_string() -> &String {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref
}

fn main() {
    let country = return_string();
}
