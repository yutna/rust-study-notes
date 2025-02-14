fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // ---

    // let string1 = String::from("long string is long");

    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }

    // ---

    // let string1 = String::from("long string is long");
    // let result;

    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }

    // println!("The longest string is {result}");

    // ---

    // let string1 = String::from("hello");
    // let string2 = String::from("world");
    // println!("{}", longest(string1.as_str(), string2.as_str()));

    // ---

    // let string1 = String::from("hello");
    // let string2 = String::from("world");
    // println!("{}", longest(string1.as_str(), string2.as_str()));

    // ---

    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
//     x
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}
