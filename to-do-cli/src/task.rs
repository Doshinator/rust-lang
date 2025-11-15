use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader}};
use std::io::Write;

use crate::conifg::{Command, Config};

pub struct Task {
    description: String,
    status: bool,
}

pub fn run(config: &Config) -> Result<(), std::io::Error> {
    let mut lines = collect_file_lines()?;
    match &config.command {
        Command::Add(item) => {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("task.txt")?;

            writeln!(file, "{}", item)?;
            println!("Added \"{}\" to to-do", item);
        },
        Command::Remove(index) => {
            if *index == 0 || *index > lines.len() {
                eprintln!("Invalid index: {}", index);
                return Ok(());
            }

            let removed_item = lines.remove(index - 1);
            println!("Removed \"{}\" from to-do", removed_item);
            write_to_file(&lines)?;
        },
        Command::List => {
            if lines.is_empty() {
                eprintln!("Empty file");
                return Ok(());
            }

            for (i, task) in lines.iter().enumerate() {
                println!("{}. {}", i + 1, task);
            }
            
        },
        Command::Complete(index) => {
            if *index == 0 || *index > lines.len() {
                eprintln!("Invalid index: {}", index);
                return Ok(());
            }
            
            let task = &mut lines[index - 1];
            if task.starts_with("[x] ") {
                println!("Task already completed.");
            } else {
                *task = format!("[x] {}", task);
                println!("Marked task {} as complete.", index);
            }
            
            write_to_file(&lines)?;
        },
    };

    Ok(())
}

fn collect_file_lines() -> Result<Vec<String>, std::io::Error> {
    let file = File::open("task.txt")?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn write_to_file(items: &[String]) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("task.txt")?;

    for item in items {
        writeln!(file, "{}", item)?;
    }

    Ok(())
}