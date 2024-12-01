// NOTE: With the `use` keyword, you can also `import` an enum,
// So you don't have to type so much

// NOTE: The `use` keyword isn't just for enums, by the way:
// it's used any time you use :: too much and want to type less.
// The `use` keyword can be used inside main or inside or outside
// another function. If you use it inside a smaller scope, like a
// separate function, then it will only apply inside that scope.
use std::mem::size_of_val;

enum Mood {
    Angry,
    Happy,
    NotBad,
    Sleepy,
}

fn match_mood(mood: &Mood) -> i32 {
    let happiness_level = match mood {
        Mood::Angry => 2,
        Mood::Happy => 10,
        Mood::NotBad => 7,
        Mood::Sleepy => 6,
    };

    happiness_level
}

// Let's try the `use` keyword to import this enum's variants
// so that we can type less.
// NOTE: to import everything, write *
fn match_mood2(mood: &Mood) -> i32 {
    use Mood::*;
    let happiness_level = match mood {
        Angry => 2,
        Happy => 10,
        NotBad => 7,
        Sleepy => 6,
    };

    happiness_level
}

fn main() {
    let my_mood = Mood::NotBad;
    let my_mood2 = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    let happiness_level2 = match_mood2(&my_mood2);

    println!("Out of 1 to 10, my happiness level is {happiness_level}");
    println!("Out of 1 to 10, my happiness level 2 is {happiness_level2}");

    println!("{}", ["-"; 50].join(""));

    let size_of_jaurim = size_of_val("자우림");
    let size_of_adrian = size_of_val("Adrian Fahrenheit Țepeș");
    println!("{size_of_jaurim}, {size_of_adrian}");
}
