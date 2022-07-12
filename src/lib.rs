use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn build (args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 { return Err("Not enough arguments") }

        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;
    println!("Content is: {}", content);

    Ok(())
}