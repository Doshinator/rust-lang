pub enum Command {
    Add(String),
    Remove(usize),
    List,
    Complete(usize),
}

pub struct Config {
    command: Command,
}

impl Config {
    pub fn build(mut args: impl Iterator <Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next().as_deref() {
            Some("add") => {
                let description = args.next()
                    .ok_or("No task description provided")?;
                Command::Add(description)
            },
            Some("remove") => {
                let index = args.next()
                    .ok_or("No index provided to remove item from list.")?
                    .parse::<usize>()
                    .map_err(|_| {
                        "Input is not an integer."
                    })?;
                
                Command::Remove(index)
            },
            Some("list") => Command::List,
            Some("complete") => {
                let index = args.next()
                    .ok_or("No index provided to remove item from list.")?
                    .parse::<usize>()
                    .map_err(|_| {
                        "Input is not an integer."
                    })?;
                Command::Complete(index)
            },
            _ => return Err("Unknown command. Commad available. \n--Add, \n--Remove, \n--List, \n--Complete."),
        };

        Ok(Config {
            command
        })
    }
}