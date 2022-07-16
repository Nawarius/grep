use std::env;
use std::process;

use grep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Fail parsing command line arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = grep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
    
}