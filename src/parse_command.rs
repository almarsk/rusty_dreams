use super::command::Command;
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
