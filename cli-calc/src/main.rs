use std::{env, i8, process};

#[derive(Debug)]
pub struct Config {
    lhs: i8,
    operator: char,
    rhs: i8,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let lhs = match args.next() {
            Some(num) => num.parse::<i8>().map_err(|_| "LHS must be an integer")?,
            None => return Err("Missing command line arg."),
        };

        let operator = match args.next() {
            Some(op) => {
                match op.as_str() {
                    "+" => '+',
                    "-" => '-',
                    "*" => '*',
                    "/" => '/',
                    _ => return Err("Invalid operator; Operator must be one of +, -, *, or /"),
                }
            },
            None => return Err("Missing command line arg."),
        };


        let rhs = match args.next() {
            Some(num) => num.parse::<i8>().map_err(|_| "RHS must be an integer")?,
            None => return Err("Missing command line arg."),
        };

        Ok(Config {
            lhs,
            operator,
            rhs
        })
    }   
}

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
    run(&config);
}

pub fn run(config: &Config) {
    let result = match config.operator {
        '+' => config.lhs + config.rhs,
        '-' => config.lhs - config.rhs,
        '*' => config.lhs * config.rhs,
        '/' => config.lhs / config.rhs,
        _ => {
            eprintln!("Invalid operator: {}", config.operator);
            return;
        }
    };
    println!(
        ">> {}",
        result
    );
}