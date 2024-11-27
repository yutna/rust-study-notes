fn main() {
    // NOTE: Since 2021, you can capture variables inside the {} of println!,
    // so you can also do this:
    let my_number = 8;
    println!("Hello, number {my_number}");

    let color1 = "red";
    let color2 = "green";
    let color3 = "blue";
    println!("I like {color1} and {color2} and {color3}");

    // But sometimes using a comma after {} looks better
    let naver_base_url = "naver";
    let google_base_url = "google";
    let microsoft_base_url = "microsoft";

    println!("The url is www.{naver_base_url}.com");
    println!("The url is www.{google_base_url}.com");
    println!("The url is www.{microsoft_base_url}.com");

    println!("The url is www.{}.com", naver_base_url);
    println!("The url is www.{}.com", google_base_url);
    println!("The url is www.{}.com", microsoft_base_url);
}
