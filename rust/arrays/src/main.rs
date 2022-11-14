// Definition:
// An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature [T; length].

// Format:
// [T;N],
// T - type of elements
// N - the number of elements

use std::mem;

fn main() {
    // Two ways to initialize an array:
    // 1. Manualy set the vaues
    let mut arr1 = [1, 2, 3, 4, 5]; // let the compile determine the type
    let arr2: [i8; 5] = [1, 2, 3, 4, 5]; // specify the type and length
    let arr3 = [1; 10]; // initialize all the data to 1
    let arr4: [i8; 10] = [0; 10]; // specify the type and length
                                  //
    arr1.rotate_left(2);
    println!("size of arr1 = {arr1:?}");
    println!("*** Sizes:");

    println!("size of arr1 = {}", mem::size_of_val(&arr1));
    println!("size of arr2 = {}", mem::size_of_val(&arr2));
    println!("size of arr3 = {}", mem::size_of_val(&arr3));
    println!("size of arr4 = {}", mem::size_of_val(&arr4));

    println!("*** Lengths:");

    println!("lenght of arr1 = {}", arr1.len());
    println!("lenght of arr2 = {}", arr2.len());
    println!("lenght of arr3 = {}", arr3.len());
    println!("lenght of arr4 = {}", arr4.len());
}
