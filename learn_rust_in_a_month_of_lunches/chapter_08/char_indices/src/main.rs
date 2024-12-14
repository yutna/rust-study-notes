// NOTE: There is another method that is like `.enumerate()`
// for char: `.char_indices()`
// NOTE: You can also use | in match statement, meaning "or"

fn main() {
    let number_together = "140399923481800622623218009598281";

    for (index, num) in number_together.char_indices() {
        match (index % 3, num) {
            (0 | 1, num) => print!("{num}"),
            _ => print!("{num}\t"),
        }
    }  
}
