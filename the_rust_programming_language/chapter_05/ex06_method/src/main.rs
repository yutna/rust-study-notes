// NOTE: Methods
// - Methods are similar to functions.
// - Unlike functions, methods are defined within the context of struct or enum
//   or a trait object.
// - Their first parameter is always `self`, which represents the instance of
//   the struct the method is being called on.
// - Everything within the `impl` block will be associated with the struct type.
// - We use the method syntax to call the method on the struct instance.
// - The method syntax goes after an instance: we add a dot followed by the
//   method name, parentheses, and any arguments.
// - The `&self` is actually short for `self: &Self`.
// - Within `impl` block, the type `Self` is an alias for the type that the
//   `impl` block is for.
// - We still need to use the `&` in front of the `self` shorthand to indicate
//   that this method borrows the `Self` instance.
// - Methods can take ownership of `self`, borrow `self` immutably, or borrow
//   `self` mutably.
// - If we wanted to change the instance that we've called the method on as part
//   of what the method does, we'd use `&mut self` as the first parameter.
// - Having a method that takes ownership of the instance by using just `self`
//   as the first parameter is rare; this technique is usually used when the
//   method transforms `self` into something else and you want to prevent the
//   caller from using the original instance after the transformation.
// - We can choose to give a method the same name as one of the struct's fields.
//   check the `width` method below.
// - When we call a method with the same name as a field, using parentheses,
//   Rust knows we are calling the method. Otherwise, it treats it as accessing
//   the field.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("The rectangle has a nonzero width; it is {}", rect.width);
    println!(
        "Is the rectangle's width greater than zero?: {}",
        rect.width()
    );
}
