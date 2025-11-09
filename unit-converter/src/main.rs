use std::{env, process};

use crate::{config::Config, unit_convert::run};
pub mod config;
pub mod unit_convert;


fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        });
    
    run(&config);
}

