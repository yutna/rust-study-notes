// NOTE: If you have a lot of things to group together, that's a struct,
// But if you have a lot of choices and need to select one, that's an enum.
// struct -> the name is short for structure.
// struct should be in UpperCamelCase
// There are 3 types of structs
// - Unit struct (does not have anything)
struct FileDirectory;
// - Tuple struct
struct ColorRgb(u8, u8, u8);
// - The named struct
struct SizeAndColor {
    color: ColorRgb,
    size: u32,
}

struct Country {
    capital: String,
    leader_name: String,
    population: u32,
}

fn main() {
    // let my_color = ColorRgb(50, 0, 50);
    // let size_and_color = SizeAndColor {
    //     color: my_color,
    //     size: 150,
    // };
    // println!("The second part of the color is: {}", my_color.1);

    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    let population = 500_000;
    // NOTE: If the field name and variable name are the same,
    // you don't have to write both.
    let kalmykia = Country {
        capital,
        leader_name,
        population,
    };
}
