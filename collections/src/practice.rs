// Given a list of integers, use a vector and return the median (when sorted, 
//      the value in the middle position) and mode (the value that occurs most 
//      often; a hash map will be helpful here) of the list.

use std::{cmp::max, collections::HashMap, i32::MIN};

pub fn median(nums: &Vec<i32>) -> i32 {
    let mut copy = nums.clone();
    copy.sort();
    let half = copy.len() / 2;
    if copy.len() % 2 == 1 {
        copy[half]
    }
    else {
        (copy[half - 1] + copy[half]) / 2
    }
}

pub fn mode(nums: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for &item in nums {
        let it_second = map.entry(item).or_insert(1);
    }


    let mut ans = MIN;
    let mut most = MIN;
    for (&num, &freq) in map.iter() {
        if freq > most {
            ans = num;
            most = freq;
        }
    }
    ans
}


// Convert strings to pig latin. The first consonant of each word is moved to
//      the end of the word and ay is added, so first becomes irst-fay. Words that start
//      with a vowel have hay added to the end instead (apple becomes apple-hay). Keep 
//      in mind the details about UTF-8 encoding!

pub fn pig_latin(str: &str) -> String {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    let text = str.trim();
    let mut chars = text.chars();
    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new()
    };

    let vowel = first_char.to_ascii_lowercase();
    if vowels.contains(&vowel) {
        return format!("{}-hay", text);
    }
    else {
        let rest_of_word = &text[first_char.len_utf8()..];
        return format!("{}-{}ay",rest_of_word, first_char);
    }
}

// Using a hash map and vectors, create a text interface to allow a user to add 
//      employee names to a department in a company; for example, “Add Sally to 
//      Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of 
//      all people in a department or all people in the company by department, sorted alphabetically.


pub struct Ledger {
    employee_ledger: HashMap<String, Vec<String>>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            employee_ledger: HashMap::new()
        }
    }

    pub fn add_employee(&mut self, department: &str, name: &str) {
        // if we find key, returns map_it_ref, otherwise insert and returns the it that we just inserted into
        let map_it_ref = self.employee_ledger.entry(department.to_string()).or_insert(Vec::new());
        map_it_ref.push(name.to_string());
    }

    pub fn retrieve(&self, department: &str) -> Vec<String> {
        match self.employee_ledger.get(department) {
            Some(v) => {
                let mut employee_list = v.clone();
                employee_list.sort();
                employee_list
            },
            None => Vec::new(),
        }
    }
}

