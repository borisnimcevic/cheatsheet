fn main() {
    let mut counter = 0; // set a counter to 0
    loop {
        counter += 1; // increase the counter by 1
        println!("The counter is now: {}", counter);
        if counter == 5 {
            // stop when counter == 5
            break;
        }
    }

    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop {
        // Give the first loop a name
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 5 {
            // Starts a second loop inside this loop
            println!("Now entering the second loop.");

            'second_loop: loop {
                // now we are inside 'second_loop
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop; // Break out of 'first_loop so we can exit the program
                }
            }
        }
    }

    let mut counter = 0;

    while counter < 5 {
        counter += 1;
        println!("The counter is now: {}", counter);
    }

    for number in 0..3 {
        println!("The number is: {}", number);
    }

    for number in 0..=3 {
        println!("The next number is: {}", number);
    }

    for _ in 0..3 {
        // if you don't need a variable name
        println!("Printing the same thing three times");
    }

    // use a break to return a value
    let mut counter = 5;
    let my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{}", my_number);

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}

fn match_colours(rbg: (i32, i32, i32)) {
    println!(
        "Comparing a colour with {} red, {} blue, and {} green:",
        rbg.0, rbg.1, rbg.2
    );
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")]; // Put the colours in a vec. Inside are tuples with the colour names
    let mut all_have_at_least_10 = true; // Start with true. We will set it to false if one colour is less than 10
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false; // Now it's false
            println!("Not much {}.", item.1) // And we print the colour name.
        }
    }
    if all_have_at_least_10 {
        // Check if it's still true, and print if true
        println!("Each colour has at least 10.")
    }
    println!(); // Add one more line
}
