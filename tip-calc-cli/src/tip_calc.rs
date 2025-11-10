// cargo run 120.50 15 4
// # Output:
// # Tip: $18.08
// # Total: $138.58
// # Per person: $34.65

// - Take command-line inputs:
//   - `bill_amount` (float)
//   - `tip_percentage` (integer or float)
//   - `number_of_people` (integer)
// - Calculate:
//   - Total tip
//   - Total bill (including tip)
//   - Amount per person if splitting


use std::{error::Error};

use crate::config::Config;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.number_of_people == 0 {
        return Err("Number of people cannot be zero.".into());
    }
    let tip = config.bill_amount * config.tip_percentage as f32 * 0.01;
    let total = config.bill_amount + tip;
    let per_person = total / config.number_of_people as f32;

    println!("Tip: ${:.2}", tip);
    println!("Total: ${:.2}", total);
    println!("Per person: ${:.2}", per_person);
    Ok(())
}