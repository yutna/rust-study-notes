// NOTE: Tuple Struct
// - Tuple structs are useful when you want to give the whole tuple a name
//   and make the tuple a different type from other tuples.
// - To define a tuple struct:
//   - Start with the `struct` keyword
//   - The struct name.
//   - The types in the tuple

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: rgb({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: [{}, {}, {}]", origin.0, origin.1, origin.2);

    // ลองแล้วไม่ได้
    // let (r, g, b) = black;
    // let (x, y, z) = origin;

    // println!("Black: rgb({r}, {g}, {b})");
    // println!("Origin: [{x}, {y}, {z}]");
}
