// NOTE: If an enum doesn't contain any data,
// then its variants can be cast into an integer.
// That's because Rust gives each variant of these
// simple enums a number that starts with 0 for its
// own use.
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

// NOTE: You can also choose a different number if you like.
// To do this, add an = and your number to the variant.
// You don't have to give all of them a number. But if you
// don't, Rust will add 1 from the variant before to give it
// a number:
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main() {
    use Season::*;
    use Star::*;

    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }

    println!("{}", ["-"; 50].join(""));

    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 && size <= 200 => println!("This is a good-sized star."),
            other_size => println!("That star is pretty big! It's {other_size}"),
        }
    }
}
