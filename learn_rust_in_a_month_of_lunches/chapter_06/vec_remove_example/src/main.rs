fn main() {
    let mut my_vec = vec![9, 8, 7, 6, 5];
    my_vec.remove(0);
    println!("{:?}", my_vec);

    let mut my_vec = vec![0; 600_000];
    for _ in 0..600_000 {
        my_vec.remove(0);
    }
}
