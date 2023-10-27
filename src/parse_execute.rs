use crate::command::Command;
use crate::modifications;
use std::error::Error;

pub fn parse_command(str_cmd: String) -> Result<Command, Box<dyn Error>> {
    if let Ok(i) = str_cmd.parse() {
        Ok(i)
    } else {
        InvalidCommand::build_err()
    }
}

#[derive(Debug)]
struct InvalidCommand;
impl std::fmt::Display for InvalidCommand {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl Error for InvalidCommand {}
impl InvalidCommand {
    fn build_err<T>() -> Result<T, Box<dyn Error>> {
        Err(Box::new(InvalidCommand))
    }
}

pub fn execute_command(command: &Command, input: &str) -> Result<String, Box<dyn Error>> {
    match command {
        Command::LowerCase => modifications::to_lowercase(Some(input)),
        Command::UpperCase => modifications::to_uppercase(Some(input)),
        Command::Slugify => modifications::to_slugified(Some(input)),
        Command::NoSpaces => modifications::no_spaces(Some(input)),
        Command::AleIronicky => modifications::ale_ironicky(Some(input)),
        Command::Reverse => modifications::reverse(Some(input)),
        Command::Csv => modifications::csv(Some(input)),
    }
}
