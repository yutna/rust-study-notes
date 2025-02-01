enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// This enum has four variants with different types:
enum Message {
    Quit,                       // has no data associated with it all.
    Move { x: i32, y: i32 },    // has named fields, like a struct does.
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values.
}

impl Message {
    // NOTE: expand this example with chatGPT
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit variant");
            }
            Message::Write(text) => {
                println!("The Write variant contains: {}", text);
            }
            Message::Move { x, y } => {
                println!("Moving to coordinates: ({}, {})", x, y);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to: ({}, {}, {})", r, g, b);
            }
        }
    }
}

fn main() {
    // Create instances of an enum
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // ---

    // let _home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let _loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // ---

    // let _home = IpAddr::V4(String::from("127.0.0.1"));
    // let _loopback = IpAddr::V6(String::from("::1"));

    // ---

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    // ---

    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::Move { x: 10, y: 20 };
    m.call()
}
