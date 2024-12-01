// NOTE: A struct can hold an enum,
// an Enum can hold a struct,
// and an enum can hold other types data

enum Climate {
    Continental,
    Dry,
    Polar,
    Temperate,
    Tropical,
}

enum ThingsInTheSky {
    Sun,
    Stars,
}

struct Country {
    capital: String,
    climate: Climate,
    leader_name: String,
    population: u32,
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!"),
    }
}

fn main() {
    let _kalmykia = Country {
        capital: String::from("Elista"),
        climate: Climate::Continental,
        leader_name: String::from("Batu Khasikov"),
        population: 500_000,
    };

    let time = 0;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}
