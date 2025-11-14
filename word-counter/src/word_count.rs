use std::{collections::{BinaryHeap, HashMap}, fs::File, io::{BufRead, BufReader, ErrorKind}};

use crate::{config::Config};

pub fn run(config: &Config) -> std::io::Result<()> {
    let info = count_words(&config.input)?;
    println!(
        "Lines: {}\nWords: {}\nCharacters: {}",
        info.lines, 
        info.word_count, 
        info.char_count
    );

    let mut heap: BinaryHeap<(usize, String)> = BinaryHeap::new();
    for (word, &count) in &info.word_freq {
        heap.push((count, word.clone()));
    }


    for i in 1..11 {
        if let Some((count, word)) = heap.pop() {
            println!("{}. {} ({} times)", i, word, count);
        }
    }

    Ok(())
}


struct Info {
    lines: usize,
    word_count: usize,
    char_count: usize,
    word_freq: HashMap<String, usize>,
}

fn open_file(file_path: &str) -> Result<File, std::io::Error> {
    match File::open(file_path) {
        Ok(f) => Ok(f),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("File not found: {}", file_path);
            return Err(e);
        }
        Err(e) => return Err(e),
    }
}

fn count_words(input: &str) -> Result<Info, std::io::Error> {
    let file = open_file(input)?;
    let reader = BufReader::new(file);
    
    let mut lines: usize = 0;
    let mut word_count: usize = 0;
    let mut char_count: usize = 0;
    let mut word_freq: HashMap<String, usize> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;
        // lines
        lines += 1;
        
        // words
        for word in line.split_whitespace() {
            let word = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
            if word.is_empty() { continue; }

            word_count += 1;
            word_freq
                .entry(word)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        // chars
        char_count += line.chars().count();
    };

    Ok(Info {
        lines,
        word_count,
        char_count,
        word_freq,
    })
}
