use super::command::{execute_command, Command};
use super::parse_command;
use rustyline::{error::ReadlineError::Interrupted, Editor};
use std::sync::mpsc;
use std::thread;

pub fn interactive_mode(command: Option<String>) {
    let mut rl: Editor<(), rustyline::history::FileHistory> = Editor::new().unwrap();
    let mut prompt = command.map(|c| format!("{} ", c));

    let (tx, rx) = mpsc::channel::<(Command, String)>();

    let tx_copy = tx.clone();
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
                    Ok(c) => tx_copy.send((c, input_string)).unwrap(),
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                };
            }
            Err(e) => match e {
                Interrupted => break,
                _ => eprintln!("{e:?}"),
            },
        }
    });

    let modify_command = thread::spawn(move || loop {
        if let Ok((command, input)) = rx.recv() {
            match execute_command(command, input) {
                Ok(r) => println!("{}", r),
                Err(e) => eprintln!("{}", e),
            };
        }
    });
    let _ = modify_command.join();
    let _ = process_command.join();
}
