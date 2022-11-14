fn main() {
    {
        let _my_number = 8; // my_number starts here
                            // my_number ends here!
                            // prefix with '_' to ignore the warning
    }

    let num = {
        let second_number = 8;
        second_number + 9 // No semicolon, so the code block returns 8 + 9.
                          // It works just like a function
    };

    let num_none = {
        let second_number = 8; // declare second_number,
        second_number + 9; // add 9 to second_number
                           // but we didn't return it!
                           // second_number dies now
    };

    println!("num = {}", num);
    println!("num_none = {:?}", num_none);
    // {:#?} is called "pretty printing". It is like {:?} but prints with different formatting over more lines.

    // shadowing
    let my_var: u8 = 5;
    let my_var = "shadowing variable";
    println!("new var = {}", my_var);

    // references (pointer)

    let a = 5;
    let ref_a = &a;
    println!("a = {}", a);
    println!("ref_a = {}", ref_a);
}
