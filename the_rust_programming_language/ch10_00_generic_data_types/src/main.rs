fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // ---

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    // ---

    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // ---
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p_constraint = Point { x: 5.0, y: 10.0 };
    println!(
        "Distance from origin: {}",
        p_constraint.distance_from_origin()
    );

    // ---
    let p1 = MPoint { x: 5, y: 10.4 };
    let p2 = MPoint { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// The generic in function definitions

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The Generic in Struct definition.

// struct Point<T> {
//     x: T,
//     y: T,
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MPoint<X2, Y2>) -> MPoint<X1, Y2> {
        MPoint {
            x: self.x,
            y: other.y,
        }
    }
}
