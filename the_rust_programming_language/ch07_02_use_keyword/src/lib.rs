// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// ---

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // use crate::front_of_house::hosting;

// mod customer {
//     use crate::front_of_house::hosting;

//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//         // super::hosting::add_to_waitlist();
//     }
// }

// ---

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

// ---

// use std::collections::HashMap;

// pub fn idiomatic_use_struct_path() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// ---

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {}

// fn function2() -> io::Result<()> {}

// ---

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

// ---

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// ---

// use std::cmp::Ordering;
// use std::io;

// use std::{cmp::Ordering, io};

// ---

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};

// ---
