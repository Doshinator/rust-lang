pub struct Config {
    input: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        println!("hello config");
        let input = String::from("some arg");

        Ok(Config {
            input
        })
    }
}