use std::{env,  process,};

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|_err| {
        // println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(_e) = minigrep::run(config){
        // println!("Application error: {}", e);
        process::exit(1);
    }
}
