use std::{fs::File, io::{BufRead, BufReader, ErrorKind}};

use crate::{config::Config, main, word_count};

pub fn run(config: &Config) -> std::io::Result<()> {
    println!("{}", &config.input);
    // lines
    // words
    // characters

    // top 3 word
    // 1.
    // 2.
    // 3.

    match count_words(&config.input) {
        Ok(_) => println!("----- Ok -----"),
        Err(_) => println!("----- Err -----"),
    };

    Ok(())
}

fn open_file(file_path: &str) -> Result<File, std::io::Error> {
    match File::open(file_path) {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("{}", e);
            return Err(e);
        }
        Err(e) => return Err(e),
    }
}

fn count_words(input: &str) -> Result<(), std::io::Error> {
    let file = open_file(input)?;
    let reader = BufReader::new(file);
    
    let mut lines: usize = 0;
    let mut word_count: usize = 0;
    let mut char_count: usize = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        lines += 1;
        word_count += line.split_whitespace().count();
        char_count += line.chars().count();
    };

    println!("---- File Results ----
        \nLines: {}
        \nWords: {}
        \nCharacters: {}
        \n", 
        lines,
        word_count,
        char_count);

    Ok(())
}


