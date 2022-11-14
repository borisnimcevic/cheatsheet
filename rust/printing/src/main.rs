fn main() {
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    ); // ignore all the '\' and '"' in the print
    let r#let = 6; // The variable's name is let
    let mut r#mut = 10; // This variable's name is mut

    let my_string = "'Ice to see you,' he said."; // single quotes
    let quote_string = r#""Ice to see you," he said."#; // double quotes
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##; // Has one # so we need at least ##
    let many_hashtags =
        r####""You don't have to type ### to use a hashtag. You can just use #.""####; // Has three ### so we need at least ####

    println!(
        "{}\n{}\n{}\n{}\n",
        my_string, quote_string, hashtag_string, many_hashtags
    );
    println!("{:?}", b"This will look like numbers"); // print as bytes
    println!("{:?}", br##"I like to write "#"."##); // combinging r and b

    println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u
                                                      // print pointer
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let number = 555;
    println!(
        "Binary: {:b}, hexadecimal: {:x}, octal: {:o}, decimal: {}",
        number, number, number, number
    );

    // change the order
    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!(
        "This is {1} {2}, son of {0} {2}.",
        father_name, son_name, family_name
    );

    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );
}
