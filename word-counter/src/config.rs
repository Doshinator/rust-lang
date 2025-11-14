pub struct Config {
    pub input: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input = match args.next() {
            Some(val) => val,
            None => return Err("Invalid input. Please provide a string to count words."),
        };

        Ok(Config {
            input
        })
    }
}