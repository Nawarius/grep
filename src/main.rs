use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Fail parsing command line arguments: {}", err);
        process::exit(1);
    });

    println!("Searching string is: {}", config.query);
    println!("Filename is: {}", config.filename);

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
    
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn build (args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 { return Err("Not enough arguments") }

        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}

fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    println!("Content is: {}", content);

    Ok(())
}

