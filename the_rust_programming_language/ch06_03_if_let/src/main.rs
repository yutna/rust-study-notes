#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Same above, but concise
    // No tedious boilerplate code to satisfy (_ => ()) the match expression.

    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // if let - else

    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }

    println!("{count}");

    // to this

    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }

    println!("{count}");
}
