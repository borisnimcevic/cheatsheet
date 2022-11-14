use std::mem;

fn main() {
    let num0: u8;
    num0 = 0;
    let num1: u8 = 1;
    let num2 = 10u8;
    let num3 = 10; // defaults to i32
    let num4: u8 = 1______0; // underscores don't matter
    let num5: f32 = 2.; 
    let num6: f32 = 2.0; 
    let num7 = 2.5; // default is f64
    let num8: bool = false;
    let num9: char = 'a';

    println!("num0: value {}, size {}", num0, mem::size_of_val(&num0));
    println!("num1: value {}, size {}", num1, mem::size_of_val(&num1));
    println!("num2: value {}, size {}", num2, mem::size_of_val(&num2));
    println!("num3: value {}, size {}", num3, mem::size_of_val(&num3));
    println!("num4: value {}, size {}", num4, mem::size_of_val(&num4));
    println!("num5: value {}, size {}", num5, mem::size_of_val(&num5));
    println!("num6: value {}, size {}", num6, mem::size_of_val(&num6));
    println!("num7: value {}, size {}", num7, mem::size_of_val(&num7));
    println!("num8: value {}, size {}", num8, mem::size_of_val(&num8));
    println!("num9: value {}, size {}", num9, mem::size_of_val(&num9));

    // unsigned
    println!("unsigned:");
    println!(
        "u8: min = {}, max = {}, size = {}",
        u8::MIN,
        u8::MAX,
        mem::size_of::<u8>()
    );
    println!(
        "u16: min = {}, max = {}, size = {}",
        u16::MIN,
        u16::MAX,
        mem::size_of::<u16>()
    );
    println!(
        "u32: min = {}, max = {}, size = {}",
        u32::MIN,
        u32::MAX,
        mem::size_of::<u32>()
    );
    println!(
        "u64: min = {}, max = {}, size = {}",
        u64::MIN,
        u64::MAX,
        mem::size_of::<u64>()
    );
    println!(
        "u128: min = {}, max = {}, size = {}",
        u128::MIN,
        u128::MAX,
        mem::size_of::<u128>()
    );
    println!(
        "usize: min = {}, max = {}, size = {}",  // The size of this primitive is how many bytes it takes to reference any location in memory.
        usize::MIN,
        usize::MAX,
        mem::size_of::<usize>()
    );

    // signed
    println!("signed:");
    println!(
        "i8: min = {}, max = {}, size = {}",
        i8::MIN,
        i8::MAX,
        mem::size_of::<i8>()
    );
    println!(
        "i16: min = {}, max = {}, size = {}",
        i16::MIN,
        i16::MAX,
        mem::size_of::<i16>()
    );
    println!(
        "i32: min = {}, max = {}, size = {}",
        i32::MIN,
        i32::MAX,
        mem::size_of::<i32>()
    );
    println!(
        "i64: min = {}, max = {}, size = {}",
        i64::MIN,
        i64::MAX,
        mem::size_of::<i64>()
    );
    println!(
        "i128: min = {}, max = {}, size = {}",
        i128::MIN,
        i128::MAX,
        mem::size_of::<i128>()
    );
    println!(
        "isize: min = {}, max = {}, size = {}",
        isize::MIN,
        isize::MAX,
        mem::size_of::<isize>()
    );

    println!(
        "f32: min = {}, max = {}, size = {}",
        f32::MIN,
        f32::MAX,
        mem::size_of::<f32>()
    );

    println!(
        "f64: min = {}, max = {}, size = {}",
        f64::MIN,
        f64::MAX,
        mem::size_of::<f64>()
    );

    println!(
        "bool: size = {}",
        mem::size_of::<bool>()
    );

    println!(
        "char: max = {}, size = {}",
        char::MAX,
        mem::size_of::<char>()
    );
}
