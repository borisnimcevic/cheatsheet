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

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        // Self means Animal.
        // You can also write Animal instead of Self

        Self {
            // When we write Animal::new(), we always get a cat that is 10 years old
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        // because we are inside Animal, &mut self means &mut Animal
        // use .change_to_dog() to change the cat to a dog
        // with &mut self we can change it
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        // use .change_to_cat() to change the dog to a cat
        // with &mut self we can change it
        println!("Changing animal to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        // we want to read self
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }

    fn update_age_to(&mut self, age: u8) {
        self.age = age;
        println!("I am {} old now", self.age);
    }
}

enum Mood {
    Good,
    Bad,
    Sleepy,
}

impl Mood {
    fn check(&self) {
        match self {
            Mood::Good => println!("Feeling good!"),
            Mood::Bad => println!("Eh, not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        Self {
            name,
            name_before,
            population,
            date_founded,
        }
    }
}

fn process_city_values(city: &City) {
    let City {
        name,
        name_before,
        population,
        date_founded,
    } = city;
    // now we have the values to use separately
    let two_names = vec![name, name_before];
    println!("The city's two names are {:?}", two_names);
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

    let mut new_animal = Animal::new(); // Associated function to create a new animal
                                        // It is a cat, 10 years old
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();
    new_animal.update_age_to(5);
    println!("I am {}", new_animal.age);

    let mood = Mood::Good;
    mood.check();

    let person = Person {
        name: "Boris".to_string(),
        real_name: "Boris".to_string(),
        height: 23,
        happiness: true,
    };

    let Person {
        // destructure
        name: a,
        real_name: b,
        height: c,
        happiness: d,
    } = person;

    println!(
        "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        a, b, c, d
    );

    let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
    process_city_values(&tallinn);
}
