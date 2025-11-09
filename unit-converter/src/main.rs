use std::env;

use crate::config::Config;

pub mod config;


fn main() {
    let config = Config::build(env::args());
}

