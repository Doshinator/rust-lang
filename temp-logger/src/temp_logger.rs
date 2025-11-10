use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, ErrorKind, Result}};
use chrono::Local;
use std::io::Write;


use crate::config::{Config, Mode};

pub fn run(config: &Config) {
    match config.mode {
        Mode::Log { degree, unit } => {
            if let Err(e) = log_temperature(degree, unit) {
                eprintln!("Error logging temperature: {}", e);
            }
        },
        Mode::Show => {
            if let Err(e) = show_file() {
                eprintln!("Error reading log file: {}", e);
            }
        },
    };
}

fn log_temperature(degree: f32, unit: char) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "{} - {} {}", timestamp, degree, unit.to_uppercase())?;
    Ok(())
}

fn show_file() -> Result<()>{
    let file_handler = match File::open("log.txt") {
        Ok(f) => f,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("No long file found");
            return Ok(());

        }
        Err(err) => return Err(err),
    };

    let reader = BufReader::new(file_handler);
    println!("Logged temperatures:");
    for line in reader.lines(){
        println!("{}", line?);
    }

    Ok(())
}