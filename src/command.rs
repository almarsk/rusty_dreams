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
            "lowercase" => Ok(Self::LowerCase),
            "uppercase" => Ok(Self::UpperCase),
            "slugify" => Ok(Self::Slugify),
            "no-space" | "no-spaces" => Ok(Self::NoSpaces),
            "ale-ironicky" => Ok(Self::AleIronicky),
            "reverse" => Ok(Self::Reverse),
            "csv" => Ok(Self::Csv),
            _ => Err(ParseCommandError),
        }
    }
}
