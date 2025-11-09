use crate::config::Config;

pub fn run(config: &Config) {
    match config.from_unit.convert_to(&config.to_unit, config.value_unit) {
        Ok(result) => println!("{} {} = {:.2} {}", 
            config.value_unit, 
            config.from_unit, 
            result, 
            config.to_unit),
        Err(e) => eprintln!("Error: {}", e),
    };
}
