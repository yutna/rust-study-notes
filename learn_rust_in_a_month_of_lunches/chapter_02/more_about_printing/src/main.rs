fn main() {
    // print!("\t start with a tab\nand move to a new line");
    // NOTE: Inside a single "" you can write over multiple lines, but be careful with the spacing:
    println!(
        "Inside quotes
you can write over
many lines
and it will print just fine.
    "
    );

    println!(
        "If you forgot to write
    on the left side, the spaces
    will be added when you print."
    );

    println!("Here there are two escape characters \\n and \\t");

    // NOTE: Sometimes you end up using too many escape characters and just want
    // Rust to print a string as you see it. To do this, you can add r# to the
    // beginning and # to the end. The r here stands for raw:
    println!("He said, \"You can find the file at c:\\files\\my_documents.txt\\file.txt\" Then I found the file.");
    println!(
        r#"He said, "You can find the file at c:\files\my_documents.txt\file.txt" Then I found the file."#
    );

    // NOTE: But what if # marks the end of the string and you need to print
    // text with a #" inside? In that case, you can start with r## and end
    // with ##. You can keep adding # to the beginning and end if you have
    // longer instances of the # symbol in your text.
    // พูดง่ายๆ ถ้ามี # เข้ามาใน string กี่ตัวก็เพิ่มเข้าไปจาก r#"..."# ปกติ เช่น มี # ใน
    // string 2 ตัว คือ r###"...#...#..."###
    let my_string = "'Ice to see you,' he said.";
    let quote_string = r#""Ice to see you," he said."#;
    let hashtag_string = r##"The hashtag "#IceToSeeYou" had become very popular."##;
    let many_hastags =
        r####"You don't have to type '###' to use a hashtag. You can just use #."####;

    println!(
        "{}\n{}\n{}\n{}",
        my_string, quote_string, hashtag_string, many_hastags
    );

    // NOTE: If you want to print the bytes of a &str or a char, you can write b
    // before the string. This works for all ASCII characters
    println!("{:?}", b"This will look like numbers");
    println!("{:?}", br##"I like to write "#""##);

    // NOTE: There is also a Unicode escape that lets you print any Unicode
    // character inside a string: \u{}.
    // A hexadecimal number goes inside the {} to print it.
    // คือเอาเลข hex ใส่ใน {} จะได้ unicode character ออกมา
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
    // วิธีการหาเลข hex จาก unicode character
    println!("{:x}", '행' as u32);
    println!("{:x}", 'H' as u32);
    println!("{:x}", '居' as u32);
    println!("{:x}", 'い' as u32);

    // NOTE: If you have a reference, you can use {:p} to print the pointer address
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    // NOTE: You can print binary, hexadecimal, and octal:
    let number = 555;
    println!(
        "Binary {:b}, Hexadecimal: {:x}, Octal: {:o}",
        number, number, number
    );

    // NOTE: You can also add numbers inside {} to change the order of what gets printed.
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";

    println!(
        "This is {1} {2}, son of {0} {2}",
        father_name, son_name, family_name
    );

    // NOTE: You can also use a name instead of an index value to do the same thing.
    // In this case, you have to use the = sign to indicate which name applies to which value:
    println!(
        "{city1} is in {country} and {city2} is also in {country}, but {city3} is not in {country}.",
        city1 = "Soul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea",
    );

    // NOTE: Complex printing in Rust is based on this format:
    // {variable:padding alignment minimum.maximum}
    // the ^ means alignment in the middle
    // < means alignment on the left
    // > means alignment on the right.
    let letter = "a";
    println!("{:ㅎ^11}", letter);

    let title = "TODAY's NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
