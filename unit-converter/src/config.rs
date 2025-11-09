pub struct Config {
    value_unit: f32,
    from_unit: Unit,
    to_unit: Unit,
}

pub enum Unit {
    In,
    Cm,
    Lb,
    Kg,
    C,
    F,
}

impl Unit {
    pub fn from_str(s: &str) -> Result<Unit, &'static str> {
        match s.to_lowercase().as_str() {
            "in" => Ok(Unit::In),
            "cm" => Ok(Unit::Cm),
            "lb" => Ok(Unit::Lb),
            "kg" => Ok(Unit::Kg),
            "c" => Ok(Unit::C),
            "f" => Ok(Unit::F),
            _ => return Err("Invalid or Unsupported unit."),
        }
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let value_unit = match args.next() {
            Some(val) => val.parse::<f32>().map_err(|_| { "Value must be a float,"})?,
            None => return Err("Did not get a query string."),
        };

        let from_unit = match args.next() {
            Some(unit) => Unit::from_str(&unit)?,
            None => return Err("Missing 'from' unit.")
        };

        let to_unit = match args.next() {
            Some(unit) => Unit::from_str(&unit)?,
            None => return Err("Missing 'to' unit.")
        };

        Ok(Config { 
            value_unit,
            from_unit,
            to_unit,
        })
    }
}