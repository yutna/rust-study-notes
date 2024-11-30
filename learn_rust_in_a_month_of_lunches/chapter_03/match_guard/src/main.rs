// NOTE: You can even put if inside of match.

fn main() {
    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if !married => println!("Not married with {children} kids"),
        (children, married) if children == 0 && married => {
            println!("Married but no children")
        }
        _ => println!("Married? {married}. Number of children: {children}."),
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colors(first);
    match_colors(second);
    match_colors(third);

    // NOTE: Each arm of a match has to return the same type.
    // let my_number = 10;
    // let some_variable = match my_number {
    //     10 => 8, ❌
    //     _ => "Not ten", ❌
    // }

    // Not work for the same reason:
    // let my_number = 10;
    // let some_variable = if my_number == 10 { 8 } else { "something else" }; ❌

    // NOTE: but this work because some_variable lives and dies inside a separate scope.
    let my_number = 10;
    if my_number == 10 {
        let _some_varaible = 0;
    } else {
        let _some_variable = "Something else";
    }
}

// NOTE: You can use _ as many times as you want in a match.
// NOTE: A match statement always stops when it finds a match and doesn't check the rest.
fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least 10"),
    }
}
