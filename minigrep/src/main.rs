use std::{env, error::Error, fs::{self}, process::{self}};
use minigrep::{search, search_case_insensitive};
fn main() {
    // read command line arg
    let config = Config::build(env::args())
        .unwrap_or_else(|error| {
            eprintln!("Problem parsing arguments: {error}");
            process::exit(1);
        });

    // result returning () ? just use if let b/c we only want to catch error
    if let Err(e) = run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Didn't get a file path"),
        };

        // get ignore_case from env var
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

