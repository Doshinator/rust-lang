use std::{env, process};

use crate::{config::Config, shop_list::run};

pub mod config;
pub mod shop_list;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        });

    if let Err(e) = run(&config) {
        eprintln!("Error {}", e);
        process::exit(1);
    }
}
