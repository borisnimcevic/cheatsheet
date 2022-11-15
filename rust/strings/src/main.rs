fn main() {
    let name = "서태지"; // This is a Korean name. No problem, because a &str is UTF-8.
    let other_name = String::from("Adrian Fahrenheit Țepeș"); // Ț and ș are no problem in UTF-8
    println!("{}", name);
    println!("{}", other_name);
    let name = "😂";
    println!("My name is actually {}", name);

    println!(
        "A String is always {:?} bytes. It is Sized.",
        std::mem::size_of::<String>()
    ); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!(
        "But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("서태지")
    ); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!(
        "And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("Adrian Fahrenheit Țepeș")
    );

    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );

    println!("{}", together);
}
