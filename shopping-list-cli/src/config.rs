pub struct Config {
    pub command: Command,
}

pub enum Command {
    Add(String),
    Remove(usize),
    List,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next().as_deref() {
            Some("add") => {
                let item = args.next().ok_or("No item provided for 'add'")?;
                Command::Add(item)
            },
            Some("remove") => {
                let index = match args.next() {
                    Some(i) => i.parse::<usize>()
                        .map_err(|_| "No index provided for 'remove'")?,
                    None => return Err("Index must be a number"),
                };
                Command::Remove(index)
            },
            Some("list") => {
                Command::List
            },
            _ => return Err("Unknown command"),
        };
        Ok(Config {
            command
        })
    }
}