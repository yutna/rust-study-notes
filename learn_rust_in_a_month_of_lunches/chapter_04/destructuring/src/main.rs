struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    // Destructuring papa_doc
    // let Person {
    //     name,
    //     real_name,
    //     height,
    //     happiness,
    // } = papa_doc;
    //
    // You can rename variables as you destructure.
    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness,
    } = papa_doc;

    println!("They call him {fake_name} but his real name is {real_name}. He is {cm}cm tall and is he happy? {happiness}");
}
