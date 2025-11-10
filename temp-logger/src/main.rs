use std::{env, process};

use crate::{config::Config, temp_logger::run};

pub mod config;
pub mod temp_logger;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1)
        });
    
    run(&config);
}
