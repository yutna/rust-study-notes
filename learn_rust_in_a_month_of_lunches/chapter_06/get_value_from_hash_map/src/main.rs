// NOTE: เราสามารถ get value จาก HashMap ได้โดยใช้ []
// เหมือน array แต่ถ้า key ไม่มีอยู่จริงโปรแกรมจะ error
// วิธีที่ safe คือใช้ get() method ซึ่งจะ return Option มาให้

use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];
    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "German");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldxxxx"));
}
