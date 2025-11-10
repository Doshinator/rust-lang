use std::{env, process};

use crate::config::Config;

pub mod config;
pub mod shop_list;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        });
}
