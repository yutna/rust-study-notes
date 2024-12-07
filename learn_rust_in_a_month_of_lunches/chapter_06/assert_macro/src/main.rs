// NOTE: There are three other macros that are similar to `panic!` that
// you use a lot in testing.
// - assert! -> If the part inside () is NOT true, the program will panic.
// - assert_eq! -> The two items inside () must be equal.
// - assert_ne! -> The two items inside () must NOT be equal (ne means "not equal")

fn main() {
    let my_name = "Loki Laufeyson";

    assert!(my_name == "Loki Laufeyson");
    assert_eq!(my_name, "Loki Laufeyson");
    assert_ne!(my_name, "Mithridates");

    // NOTE: You can also add a message to these methods if you want:
    assert!(
        my_name == "Loki Laufeyson",
        "Name {my_name} is wrong: should be Loki Laufeyson"
    );

    assert_eq!(
        my_name, "Loki Laufeyson",
        "{my_name} and Loki Laufeyson should be equal"
    );

    let my_name = "Mithridates";
    assert_ne!(
        my_name, "Mithridates",
        "You entered {my_name}. Input must NOT equal Mithridates"
    );
}
