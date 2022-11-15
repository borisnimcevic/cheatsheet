// Definition:
// An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature [T; length].

// Format:
// [T;N],
// T - type of elements
// N - the number of elements

// * Index numbers start at 0 (not 1)
// * Index ranges are exclusive (they do not include the last number)

use std::mem;

fn main() {
    let mut arr1 = [1, 2, 3, 4, 5]; // let the compile determine the type
    let arr2: [i8; 5] = [1, 2, 3, 4, 5]; // specify the type and length
    let arr3 = [1; 10]; // initialize all the data to 1
    let arr4: [i8; 10] = [0; 10]; // specify the type and length

    arr1.rotate_left(2);
    println!("arr1 = {arr1:?}");

    println!(
        "arr1; size = {} , length = {}",
        mem::size_of_val(&arr1),
        arr1.len()
    );
    println!(
        "arr2; size = {} , length = {}",
        mem::size_of_val(&arr2),
        arr2.len()
    );
    println!(
        "arr3; size = {} , length = {}",
        mem::size_of_val(&arr3),
        arr3.len()
    );
    println!(
        "arr4; size = {} , length = {}",
        mem::size_of_val(&arr4),
        arr4.len()
    );

    let some_range = &arr2[1..3]; // 3 is exclusive
    println!("some_range = {:?}", some_range);

    let some_other_range = &arr2[1..=3]; // 3 is inclusive
    println!("some_other_range= {:?}", some_other_range);
}
