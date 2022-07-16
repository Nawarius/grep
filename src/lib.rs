use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn`t get a file path")
        };

        Ok(Config { 
            query, 
            filename,
            ignore_case: env::var("IGNORE_CASE").is_ok() 
        })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_insensitive(&config.query, &content)
    } else {
        search_sensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search_sensitive <'a, 'b> (query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&'b str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive <'a, 'b> (query: &'a str, contents: &'b str) -> Vec<&'b str> {
    let mut results: Vec<&'b str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn case_sensitive () {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive () {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}