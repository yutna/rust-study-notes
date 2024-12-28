// NOTE: Shadowing
// - You can declare a new variable with the same name as a previous variable.
// - Rustaceans say that the first variable is shadowed by the second, which
//   means that the second variable is what the compiler will see when you use
//   the name of the variable.
// - In effect, the second variable overshadows the first, taking any uses of
//   the variable name to itself until either it itself is shadowed or the
//   scope ends.
// - when we use the `let` keyword for shadowing, we can change the type of the
//   value but reuse the same name.

fn main() {
    let x = 5;
    let x = x + 1; // x = 6

    {
        // The third let statement also shadows x and creates a new variable.
        let x = x * 2; // x = 12
        println!("The value of x in the inner scope is: {x}");
        // When that scope is over, the inner shadowing ends and x returns to
        // being 6.
    }

    println!("The value of x is: {x}");

    // ---

    // Shadowing can store difference types
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type

    // ถ้าไม่ใช่ shadowing แต่ใช้ `mut` แทนสำหรับ reuse variable name และ store new
    // value จะไม่สามารถทำได้เพราะ type ต่างกัน
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
