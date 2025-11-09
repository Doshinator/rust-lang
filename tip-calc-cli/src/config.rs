pub struct Config {
    pub bill_amount: f32,
    pub tip_percentage: u8,
    pub number_of_people: u8,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        args.next();
        
        let bill_amount = match args.next() {
            Some(val) => val.parse::<f32>().map_err(|_| { "Value must be a float." })?,
            None => return Err("Invalid query string"),
        };

        let tip_percentage: u8 = 8;
        let number_of_people: u8 = 8;

        Ok(Config {
            bill_amount,
            tip_percentage,
            number_of_people,
        })
    }
}