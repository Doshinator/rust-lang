use std::error::Error;
use std::{env, process};
use crate::config::Config;
use crate::tip_calc::run;

pub mod config;
pub mod tip_calc;
fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args())
        .unwrap_or_else(|e| {
            eprint!("{}", e);
            process::exit(1);
        });
    
    run(&config)?;
    Ok(())
}