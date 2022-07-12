use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config: Config = Config::new(&args);

    println!("Searching string is: {}", config.query);
    println!("Filename is: {}", config.filename);

    let content: String = fs::read_to_string(config.filename).unwrap();
    println!("Content is: {}", content)
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new (args: &[String]) -> Self {
        Config {
            query: args[1].clone(),
            filename: args[2].clone()
        }
    }
}

