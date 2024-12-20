// NOTE: Sometimes you want a function that can take both a String
// and a &str. You can do this with the `AsRef` trait, which is
// used to give a reference from one type to another type. You can
// think of it as a sort of cheap version of From: instead of
// converting from one type to another. you do a cheap conversion
// from one reference to another. Here is how the standard library
// describes it:

// Both String and &str implement AsRef<str>.

fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref());
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
    // print_it(7);
}

