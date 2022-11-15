struct Colour(u8, u8, u8);

struct SizeAndColour {
    size: u32,
    colour: Colour, // And we put it in our new named struct
}

struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let my_colour = Colour(50, 0, 50); // Make a colour out of RGB (red, green, blue)
    println!("The second part of the colour is: {}", my_colour.1);

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour,
    };

    println!(
        "size is {}, third color is {}",
        size_and_colour.size, size_and_colour.colour.2
    );

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };

    println!("{}", kalmykia.population);
    println!("{}", kalmykia.capital);
    println!("{}", kalmykia.leader_name);
}
