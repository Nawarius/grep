use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let query: &str = &args[1];
    let filename: &str = &args[2];

    println!("Searching string is: {}", query);
    println!("Filename is: {}", filename);

}
