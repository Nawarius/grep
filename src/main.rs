use std::env;
use std::process;

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Fail parsing command line arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = grep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
    
}