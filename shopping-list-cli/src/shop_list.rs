use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, ErrorKind, Write}};

use crate::config::{Command, Config};

pub fn run(config: &Config) -> std::io::Result<()> {
    match &config.command {
        Command::Add(item) => add_item(item)?,
        Command::Remove(index) => remove_item(*index)?,
        Command::List => show_list()?,
    };

    Ok(())
}

fn add_item(item: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("shopping_list.txt")?;
    
    writeln!(file, "{}", item)?;
    println!("Added \"{}\" to your shopping list.", item);
    Ok(())
}

fn remove_item(index: usize) -> std::io::Result<()> {
    let mut lines: Vec<String> = {
        let file = match File::open("shopping_list.txt") {
            Ok(f) => f,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                eprintln!("No shopping list found.");
                return Ok(());
            }
            Err(err) => return Err(err),
        };

        let reader = BufReader::new(file);
        reader.lines().collect::<std::io::Result<Vec<_>>>()?
    };

    if index == 0 || index > lines.len() {
        eprintln!("Invalid index: {}", index);
        return Ok(());
    }

    let removed_item = lines.remove(index - 1);
    println!("Removed \"{}\" from your shopping list.", removed_item);


    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .open("shopping_list.txt")?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn show_list() -> std::io::Result<()> {
    let file_handler = match File::open("shopping_list.txt") {
        Ok(f) => f,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            eprintln!("File 'shopping_list.txt' not found");
            return Ok(());
        }
        Err(err) => return Err(err),
    };

    let reader = BufReader::new(file_handler);
    let lines: Vec<String> = reader.lines().collect::<std::io::Result<Vec<_>>>()?;
    
    if lines.is_empty() {
        println!("Your shopping list is empty.");
    }
    else {
        for (i, line) in lines.iter().enumerate() {
            println!("{}. {}", i + 1, line);
        }
    }
        
    Ok(())
}
