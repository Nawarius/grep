use std::env;
use std::process;

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Fail parsing command line arguments: {}", err);
        process::exit(1);
    });

    println!("Searching string is: {}", config.query);
    println!("Filename is: {}", config.filename);

    if let Err(err) = grep::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
    
}