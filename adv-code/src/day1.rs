use std::{fs::File, io::{BufRead, BufReader}};


// day1_part1
pub struct Config {
    pub input: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, std::io::Error> {
        args.next();

        let input = args
            .next()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing input"))?;

        Ok(Config {
            input
        })
    }
}


pub fn run(config: &Config) -> Result<(), std::io::Error> {
    let file_path = &config.input;
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    // let lines: Vec<String> = reader
    //     .lines()
    //     .collect::<Result<Vec<_>, _>>()?;

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for lines in reader.lines() {
        let line = lines?;
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    let total_distance: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total distance: {}", total_distance);

    Ok(())
}
