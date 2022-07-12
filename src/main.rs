use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let query: &str = &args[1];
    let filename: &str = &args[2];

    println!("Searching string is: {}", query);
    println!("Filename is: {}", filename);

    let content: String = fs::read_to_string(filename).unwrap();
    println!("Content is: {}", content)
}
