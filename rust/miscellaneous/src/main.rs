const NUMBER_OF_MONTHS: u32 = 12; 
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

fn main() {
    println!("const = {}", NUMBER_OF_MONTHS);
    println!("static = {}", SEASONS[2]);

}
