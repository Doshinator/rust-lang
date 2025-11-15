use std::{env, process};

use crate::day1::{Config, run};

pub mod day1;
fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Application error. {}", err);
            process::exit(1);
        });

    if let Err(e) = run(&config) {
        eprintln!("Application error. {}", e);
        process::exit(1);
    }
}
