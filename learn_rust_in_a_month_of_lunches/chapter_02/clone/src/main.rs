fn prints_country(country_name: String) {
    println!("{country_name}");
}

fn main() {
    let country = String::from("Kiribati");
    prints_country(country.clone());
    prints_country(country);
}
