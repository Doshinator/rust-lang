pub enum Mode {
    Log { degree: f32, unit: char },
    Show,
}

pub struct Config {
    pub mode: Mode,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        
        let mode_str = match args.next() {
            Some(m) => m,
            None => return Err("No mode provided"),
        };

        let mode = match mode_str.as_str() {
            "log" => {
                // parse degree
                let degree = match args.next() {
                    Some(d) => d.parse::<f32>().map_err(|_| { "Degree must be a float" })?,
                    None => return Err("No degree provided"),
                };

                let unit = match args.next() {
                    Some(c) => {
                        let ch = c.chars().next().ok_or("Unit must be a single character.")?;
                        match ch {
                            'C' | 'c' | 'F' | 'f' => ch,
                            _ => return Err("Invalid unit. Must be 'C' or 'F'."),
                        }
                    },
                    None => return Err("No unit provided"),
                };

                Mode::Log {
                    degree,
                    unit
                }
            },
            "show" => {
                Mode::Show
            },
            _ => return Err("Invalid mode. Must be 'log' or 'show'"),
        };

        Ok(Config {
            mode
        })
    }
}
