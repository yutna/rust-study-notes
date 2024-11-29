// NOTE: There are many ways to make a string (String type). Here are some:
// - String::from("This is the string text");
// - "This is the string text".to_string()
// - The `format!` macro—This works just like println!, except it creates a string instead of printing.

fn main() {
    let name = "Billybrobby";
    let country = "USA";
    let home = "Korea";
    let together = format!("I am {name} from {country} but I live in {home}.");

    println!("{together}");

    // NOTE: Another way to make a String is called .into()
    // but it is a bit different because .into() isn’t for making a string;
    // it’s for converting from one type into another type.
    // Some types can easily convert to and from another type using from:: and .into();
    // if you have from::, you also have .into().
    // from:: is clearer because you already know the type.
    // You know that String::from("Some str") is a String from a &str.
    // But with .into(), sometimes the compiler doesn’t know:
    // You need to explicit type when use .into()
    let my_string: String = "Try to make this a String".into();
    println!("{my_string}");
}
