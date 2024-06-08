use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // Read any command line arguments and collect the values into a vector

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) { // Use if let and not unwrap because there is no value to be returned in Ok instance
        println!("Application error: {}", e);

        process::exit(1);
    }

}
