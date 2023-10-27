use std::env;
mod parse_command;
use parse_command::parse_command;
mod interactive;
use interactive::interactive_mode;
mod command;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        #[allow(clippy::match_single_binding)]
        match parse_command(args[1].clone()) {
            _ => todo!("oneshot functionality"),
        }
    } else {
        interactive_mode(None)
    }
}
