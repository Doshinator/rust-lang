use std::fmt;

pub struct Config {
    pub value_unit: f32,
    pub from_unit: Unit,
    pub to_unit: Unit,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum UnitCategory {
    Length,
    Weight,
    Temperature,
}

pub enum Unit {
    In,
    Cm,
    Lb,
    Kg,
    C,
    F,
    Mi,
    Km
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
            "mi" => Ok(Unit::Mi),
            "km" => Ok(Unit::Km),
            _ => return Err("Invalid or Unsupported unit."),
        }
    }

    pub fn convert_to(&self, target: &Unit, value: f32) -> Result<f32, &'static str> {
        if self.category() != target.category() {
            return Err("Incompatible units");
        }

        match (self, target) {
            // length
            (Unit::Cm, Unit::In) => Ok(value / 2.54),
            (Unit::In, Unit::Cm) => Ok(value * 2.54),
            (Unit::Mi, Unit::Km) => Ok(value * 1.60934),
            (Unit::Km, Unit::Mi) => Ok(value * 0.62137),
            // weight
            (Unit::Kg, Unit::Lb) => Ok(value * 2.20462),
            (Unit::Lb, Unit::Kg) => Ok(value / 2.20462),
            // temp
            (Unit::C, Unit::F) => Ok(value * 9.0 / 5.0 + 32.0),
            (Unit::F, Unit::C) => Ok((value - 32.0) * 5.0 / 9.0),
            (_, _) => Ok(value), // same unit
        }
    }

    pub fn category(&self) -> UnitCategory {
        match self {
            Unit::Cm | Unit::In | Unit::Mi | Unit::Km => UnitCategory::Length,
            Unit::Lb | Unit::Kg => UnitCategory::Weight,
            Unit::C | Unit::F => UnitCategory::Temperature,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Unit::In => "in",
            Unit::Cm => "cm",
            Unit::Lb => "lb",
            Unit::Kg => "kg",
            Unit::C => "C",
            Unit::F => "F",
            Unit::Mi => "mi",
            Unit::Km => "km",
        };
        write!(f, "{}", s)
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let value_unit = match args.next() {
            Some(val) => val.parse::<f32>().map_err(|_| { "Value must be a float"})?,
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