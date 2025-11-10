pub struct Config {
    pub degrees: f32,
    pub temperature: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let degree = match args.next() {
            Some(degree) => degree.parse::<f32>().map_err(|_| { "Degrees is not a valid flot."})?,
            None => return Err("Invalid query."),
        };

        let temp = match args.next() {
            Some(temp) => temp,
            None => return Err("Invalid query."),
        };

        Ok(Config { 
            degrees: degree, 
            temperature: temp,
        })
    }
}
