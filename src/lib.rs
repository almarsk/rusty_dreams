use slug::slugify;
mod parse_csv;
use parse_csv::parse_into_ascii_table;
mod my_error;
use std::{env, error::Error};

pub fn to_lowercase() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    Ok(text.to_lowercase())
}
pub fn to_uppercase() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    Ok(text.to_uppercase())
}
pub fn to_slugified() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    Ok(slugify(text))
}
pub fn no_spaces() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    Ok(text.replace(' ', ""))
}
pub fn ale_ironicky() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    let output = text
        .chars()
        .enumerate()
        .fold(String::new(), |mut sparkle, (i, c)| {
            let transformed_char = if i % 2 == 0 {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            };
            sparkle.push(transformed_char);
            sparkle
        });
    Ok(output)
}
pub fn reverse() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    Ok(text.chars().rev().collect())
}
pub fn csv() -> Result<String, Box<dyn Error>> {
    let text = parse_second_arg()?;
    let parsed = parse_into_ascii_table(text)?;
    Ok(parsed)
}

fn parse_second_arg() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        todo!("spin up concurrent interactive mode, command prompt")
    } else {
        Ok(args[2].clone())
    }
}
