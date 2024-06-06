use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result <Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("{}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Read any command line arguments and collect the values into a vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config);

}
