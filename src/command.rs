use rust_course::*;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    LowerCase,
    UpperCase,
    Slugify,
    NoSpaces,
    AleIronicky,
    Reverse,
    Csv,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCommandError;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // trying to avoid stringly-typed api, I still had to do this
            "lowercase" => Ok(Self::LowerCase),
            "uppercase" => Ok(Self::UpperCase),
            "slugify" => Ok(Self::Slugify),
            "no-space" => Ok(Self::NoSpaces),
            "ale-ironicky" => Ok(Self::AleIronicky),
            "reverse" => Ok(Self::Reverse),
            "csv" => Ok(Self::Csv),
            _ => Err(ParseCommandError),
        }
    }
}

pub fn execute_command(
    command: Command,
    input: String,
) -> Result<String, std::sync::Arc<dyn Error>> {
    let input = input.as_str();
    match command {
        Command::LowerCase => to_lowercase(Some(input)),
        Command::UpperCase => to_uppercase(Some(input)),
        Command::Slugify => to_slugified(Some(input)),
        Command::NoSpaces => no_spaces(Some(input)),
        Command::AleIronicky => ale_ironicky(Some(input)),
        Command::Reverse => reverse(Some(input)),
        Command::Csv => csv(Some(input)),
    }
}
