// NOTE: Remember when we learned in the last chapter that shadowing doesn’t destroy a value but blocks it?
// We can prove this now that we know how to use references.

fn main() {
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;

    println!("{} {}", country_ref, country);
}
