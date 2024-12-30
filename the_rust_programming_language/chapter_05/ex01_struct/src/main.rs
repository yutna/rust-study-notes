// NOTE: Struct
// - A struct, or structure, is a custom data type that lets you package
//   together and name multiple related values that make up a meaningful group.
// - Like tuples, the pieces of a struct can be different types.
// - Unlike with tuples, in a struct you’ll name each piece of data so it’s
//   clear what the values mean.
// - To define a struct, we enter the keyword `struct` and name the entire
//   struct.
// - Inside curly brackets, we define the names and types of the pieces of data,
//   which we call *fields*.
// - To use a struct after we’ve defined it, we create an instance of that
//   struct by specifying concrete values for each of the fields.
// - To get a specific value from a struct, we use dot notation.
// - If the instance is mutable, we can change a value by using the dot
//   notation and assigning into a particular field.
// - Note that the entire instance must be mutable; Rust doesn’t allow us to
//   mark only certain fields as mutable.
// - If the variable names or parameter names and the struct field names are
//   exactly the same, we can use the *field init shorthand* syntax
// - It’s often useful to create a new instance of a struct that includes most
//   of the values from another instance, but changes some. You can do this
//   using *struct update syntax*. The syntax `..` specifies that the remaining
//   fields not explicitly set should have the same value as the fields in the
//   given instance.
// - **IMPORTANT** Don't forget about ownership rules when using struct update
//   syntax. see the example below, the `user1.username` has been transferred
//   (moved) to `user2`.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand syntax
        email,    // field init shorthand syntax
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // Using struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user2.username);
}
