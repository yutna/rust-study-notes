#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("{rect:#?}");
    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
