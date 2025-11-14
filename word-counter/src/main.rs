use std::{env, process::{self}};

use crate::{config::Config, word_count::run};

pub mod config;
pub mod word_count;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        });

    if let Err(e) = run(&config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
