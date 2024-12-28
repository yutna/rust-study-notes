fn main() {
    for n in 1..=10 {
        println!("{}", fibo_number(n));
    }
}

fn fibo_number(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibo_number(n - 1) + fibo_number(n - 2)
}
