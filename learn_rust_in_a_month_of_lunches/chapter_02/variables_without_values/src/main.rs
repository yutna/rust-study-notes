fn main() {
    let my_number;

    {
        let calcuation_result = { 57 };
        my_number = calcuation_result;
        println!("{}", my_number);
    }

    println!("{}", my_number);
}
