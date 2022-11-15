'!' after a function name means macro

Rust has two rules for mutable and immutable references. They are very important, but also easy to remember because they make sense.

Rule 1: If you have only immutable references, you can have as many as you want. 1 is fine, 3 is fine, 1000 is fine. No problem.
Rule 2: If you have a mutable reference, you can only have one. Also, you can't have an immutable reference and a mutable reference together.


    
Giving reference to functions:
fn function_name(variable: String) takes a String and owns it. If it doesn't return anything, then the variable dies inside the function.
fn function_name(variable: &String) borrows a String and can look at it
fn function_name(variable: &mut String) borrows a String and can change it

Remember this for match:
You write match and then make a {} code block.
Write the pattern on the left and use a => fat arrow to say what to do when it matches.
Each line is called an "arm".
Put a comma between the arms (not a semicolon).
A match statement always stops when it finds a match, and doesn't check the rest.
A match has to return the same type.
