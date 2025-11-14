use std::{env, process};

use crate::conifg::Config;

pub mod task;
pub mod conifg;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|e: &'static str| {
            eprintln!("{}", e);
            process::exit(1);
        });
}
