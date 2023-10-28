use rustyline::{error::ReadlineError::Interrupted, history, Editor};
use std::sync::mpsc;
use std::thread;
pub mod command;
use command::Command;
pub mod parse_execute;
use parse_execute::{execute_command, parse_command};
pub mod modifications;
mod parse_csv;
use std::{env, error::Error};

pub fn interactive_mode(command: Option<String>) {
    let mut rl: Editor<(), history::FileHistory> = Editor::new().unwrap();
    let mut prompt = command.map(|c| format!("{} ", c));

    let (tx, rx) = mpsc::channel::<Option<(Command, String)>>();

    // slightly beyond the scope of the assignment
    // if only command is given and no input upon startup
    // interactive mode is entered with the (modifiable) command ready
    let process_command = thread::spawn(move || loop {
        let readline = if let Some(i) = prompt.take() {
            rl.readline_with_initial("", (&i, ""))
        } else {
            rl.readline("")
        };
        match readline {
            Ok(i) => {
                let mut iter = i.splitn(2, ' ');
                let (command_string, input_string) = {
                    let c = iter.next().unwrap_or_default().to_string();
                    let i = iter.next().unwrap_or_default().to_string();
                    (c, i)
                };
                match parse_command(command_string) {
                    // empty input string is still being sent to the processing command
                    Ok(c) => tx.send(Some((c, input_string))).unwrap(),
                    Err(e) => {
                        eprintln!("{e:?}\n");
                    }
                };
            }
            Err(e) => match e {
                Interrupted => {
                    // Sending None to signal to the other thread to stop as well.
                    // Otherwise I had to press ctrl+c twice
                    tx.send(None).unwrap();
                    break;
                }
                _ => eprintln!("{e:?}\n"),
            },
        }
    });
    let modify_command = thread::spawn(move || loop {
        if let Ok(m) = rx.recv() {
            if let Some((command, input)) = m {
                modify(command, input)
            } else {
                break;
            }
        }
    });
    let _ = modify_command.join();
    let _ = process_command.join();
}

pub fn modify(command: Command, input: String) {
    match execute_command(&command, input.as_str()) {
        Ok(r) => println!("{}\n", r),
        Err(e) => eprintln!("{}", e),
    };
}

pub fn parse_second_arg(interactive_input: Option<&str>) -> Result<String, Box<dyn Error>> {
    if let Some(i) = interactive_input {
        Ok(i.to_string())
    } else {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            interactive_mode(Some(args[1].clone()));
            std::process::exit(0);
        } else {
            Ok(args[2].clone())
        }
    }
}
