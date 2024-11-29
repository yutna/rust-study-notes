const NUMBER_OF_MONTH: u32 = 12;
static SEASON: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

fn print_months() {
    println!("Number of months in the year: {NUMBER_OF_MONTH}");
}

fn main() {
    print_months();
}
