use slug::slugify;

use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("E R R O R - not enuff args");
        return;
    }
    // I am looking forward to get further insight into how the Box dyn trick stuff works and why
    // type declaration was Clippys idea
    type Instructions<'a> = HashMap<&'a str, Box<dyn FnOnce(String) -> String>>;
    let mut options: Instructions = HashMap::new();
    options.insert("lowercase", Box::new(|text| text.to_lowercase()));
    options.insert("uppercase", Box::new(|text| text.to_uppercase()));

    #[allow(clippy::redundant_closure)]
    // https://github.com/rust-lang/rust-clippy/issues/3071
    options.insert("slugify", Box::new(|text| slugify(text)));
    options.insert("no-spaces", Box::new(|text| text.replace(' ', "")));
    options.insert(
        "ale-ironicky",
        Box::new(|text| {
            text.chars()
                .enumerate()
                .fold(String::new(), |mut sparkle, (i, c)| {
                    match i % 2 == 0 {
                        true => sparkle.push(c.to_lowercase().next().unwrap()),
                        false => sparkle.push(c.to_uppercase().next().unwrap()),
                    };
                    sparkle
                })
        }),
    );
    options.insert("reverse", Box::new(|text| text.chars().rev().collect()));

    if let Some(transmute) = options.remove(args[1].as_str()) {
        let input = args[2].clone();
        println!("{}", transmute(input));
    } else {
        println!("E R R O R - unknown instruction")
    }
}
