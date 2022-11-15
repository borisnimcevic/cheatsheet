use std::mem;

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();
    // If we run the program now, the compiler will give an error.
    // It doesn't know the type of vec.

    my_vec.push(name1); // Now it knows: it's Vec<String>
    my_vec.push(name2);
    println!("{:?}", my_vec);

    let mut my_vec: Vec<String> = Vec::new(); // tell in advance what type it is
    my_vec.push("Boris".to_string());
    println!("{:?}", my_vec);

    // like an array but with macro
    let mut my_vec = vec![1, 2, 3];
    my_vec.push(4);
    my_vec.push(5);
    println!("{:?}", my_vec);
    println!(
        "size = {}, len = {}, capacity = {}",
        mem::size_of_val(&my_vec),
        my_vec.len(),
        my_vec.capacity()
    );
    my_vec.push(6);
    println!(
        "size = {}, len = {}, capacity = {}",
        mem::size_of_val(&my_vec),
        my_vec.len(),
        my_vec.capacity()
    );
    my_vec.push(7); // it doubles the capacity every time it reaches the limit, like in c++
    println!(
        "size = {}, len = {}, capacity = {}",
        mem::size_of_val(&my_vec),
        my_vec.len(),
        my_vec.capacity()
    );


    // or if you know in advance the capacity, you can tell it
    let mut num_vec = Vec::with_capacity(7); // Give it capacity 8
    num_vec.push(1);
    num_vec.push(2);
    num_vec.push(3);
    num_vec.push(4);
    num_vec.push(5);
    num_vec.push(6);
    num_vec.push(7);
    println!(
        "size = {}, len = {}, capacity = {}",
        mem::size_of_val(&num_vec),
        num_vec.len(),
        num_vec.capacity()
    ); 

    let my_vec: Vec<u8> = [1, 2, 3].into();  // give it a tupy
    let my_vec2: Vec<_> = [9, 0, 10].into(); // Vec<_> means "choose the Vec type for me"
                                             // Rust will choose Vec<i32>
}
