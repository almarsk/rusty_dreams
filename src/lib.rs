use slug::slugify;
mod parse_csv;
use parse_csv::parse_into_ascii_table;
use std::sync::Arc;
use std::{env, error::Error};

pub fn to_lowercase(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
    Ok(text.to_lowercase())
}
pub fn to_uppercase(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
    Ok(text.to_uppercase())
}
pub fn to_slugified(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
    Ok(slugify(text))
}
pub fn no_spaces(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
    Ok(text.replace(' ', ""))
}
pub fn ale_ironicky(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
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
pub fn reverse(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
    Ok(text.chars().rev().collect())
}
pub fn csv(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    let text = parse_second_arg(interactive_input)?;
    let parsed = parse_into_ascii_table(text)?;
    Ok(parsed)
}

fn parse_second_arg(interactive_input: Option<&str>) -> Result<String, Arc<dyn Error>> {
    if let Some(i) = interactive_input {
        Ok(i.to_string())
    } else {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            todo!("spin up concurrent interactive mode, command prompt")
        } else {
            Ok(args[2].clone())
        }
    }
}
