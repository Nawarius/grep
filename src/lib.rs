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
    
    for line in search(&config.query, &content) {
        println!("{}", line)
    }

    Ok(())
}

pub fn search <'a, 'b> (query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&'b str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn one_result () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}