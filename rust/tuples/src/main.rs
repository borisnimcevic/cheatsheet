fn main() {
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: 
First item &str: {:?}
Second item i32: {:?}
Third item Vec<char>: {:?}
Fourth item char: {:?}
Fifth item [{{i32}};3]: {:?}
Sixth item f64: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5,
    );
    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // pull elements into a tuple
    let (_, b, _) = (str_vec[0], str_vec[1], str_vec[2]); // if you don't want to create variables for 'a' and 'c'
    println!("{:?}", b);
}
