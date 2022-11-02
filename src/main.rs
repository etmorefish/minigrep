use std::{env, process};

use minigrep::Config;
mod filt;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
