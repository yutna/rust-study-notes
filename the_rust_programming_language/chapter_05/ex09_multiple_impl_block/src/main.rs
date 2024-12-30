// NOTE: Multiple `impl` block
// - Each struct is allowed to have multiple `impl` block.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 32,
        height: 64,
    };

    let other_rect = Rectangle {
        width: 12,
        height: 24,
    };

    println!("{}", rect.area());
    println!("{}", rect.can_hold(&other_rect));
}
