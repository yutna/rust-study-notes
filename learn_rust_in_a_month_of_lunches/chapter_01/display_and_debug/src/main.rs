/*
 * NOTE: {}—Display print. More types have Debug than Display, so if a type you
 * want to print can’t print with Display, you can try Debug
 *
 * {:?}—Debug print. If there is too much information on one line,
 * you can try {:#?}.
 *
 * {:#?}—Debug print, but pretty. Pretty means that each part of a type is
 * printed on its own line to make it easier to read.
 */

fn main() {
    let doesnt_print = ();
    println!("This will not print: {:?}", doesnt_print);
}
