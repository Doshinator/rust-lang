use std::{env, process};
use crate::config::Config;

pub mod config;
fn main() {
    let config = Config::build(env::args())
        .map_err(|e| {
            eprint!("{}", e);
            process::exit(1);
        });
    
    run(&config);
}
