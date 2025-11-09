pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut vector = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         vector.push(line);
    //     }
    // }   

    // vector
    // no need for mut vector - below is a shorter + better way (same thing though in terms of performance)
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result: Vec<&str> = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }

    // result

    // no need for mut vector - below is a shorter + better way (same thing though in terms of performance)
    contents
        .lines()
        .filter(|lines| lines.to_lowercase().contains(query))
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}