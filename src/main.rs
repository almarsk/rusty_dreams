use rust_course::parse_execute::parse_command;
use rust_course::{interactive_mode, modify, parse_second_arg};
use std::env;
pub mod command;
pub mod parse_csv;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        match parse_command(args[1].clone()) {
            Ok(c) => {
                // parse_second_arg enters interactive mode if arg 2 is mising
                let input = parse_second_arg(None).unwrap();
                println!("{}", args[1..3].join(" "));
                modify(c, input)
            }
            Err(_) => interactive_mode(None),
        }
    } else {
        interactive_mode(None)
    }
}
