fn main() {
    let mut counter = 0;
    
    let result = 'label: loop {
        counter += 1;
        if counter == 3 {
            break 'label "Oh Yes!";
        }
    };

    println!("{result}");
}

