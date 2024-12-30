// NOTE: Ownership of Struct Data
// - It’s also possible for structs to store references to data owned by
//   something else, but to do so requires the use of lifetimes.
// - Lifetimes ensure that the data referenced by a struct is valid for as long
//   as the struct is.

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let _user = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
