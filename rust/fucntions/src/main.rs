use std::mem;

fn main() {
    // '()' means no arguments passed
    println!("Hello, world!"); // '!' means the fuctions is a macro
    println!("print number = {}", 8); // '{}' means print number here
    println!(
        "number = {}, of size {}",
        number(),
        mem::size_of_val(&number())
    );
    multiply(2, 5);
    println!("multiply2 result = {}", multiply2(2, 4)); // '{}' means print number here
}

fn number() -> u16 {
    5 // it returns because there's no ';'
}

fn multiply(num1: i32, num2: i32) {
    // you have to give a name and a type
    let result = num1 * num2;
    println!("result  = {}", result);
}

fn multiply2(num1: i32, num2: i32) -> i32 {
    // you have to give a name and a type
    let result = num1 * num2;
    println!("result  = {}", result);
    return result; // return will work like in c
}
