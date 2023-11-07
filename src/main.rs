use crossterm::{cursor, execute, terminal};

use std::error::Error;

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::{thread, time::Duration};

use rusty_dreams::{full_now, handle_client, now, send_message, Message, MessageType};

fn main() {
    send_and_receive("127.0.0.1", "11111")
}

fn send_and_receive(host: &str, port: &str) {
    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        "127.0.0.1:11111".parse().unwrap()
    };

    let nick = match std::env::args().nth(1) {
        Some(n) => Arc::new(if n.len() <= 14 {
            n.to_string()
        } else {
            n.chars().take(14).collect() // Truncate the string
        }),
        None => Arc::new(names::Generator::default().next().unwrap()),
    };

    let nick_clone = nick.clone();

    let mut connection = TcpStream::connect(address).unwrap();
    connection
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let (tx, rx) = mpsc::channel();

    print_nick(nick_clone.as_str(), None);

    let reading_thread = thread::spawn(move || loop {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let my_message: Result<Message, Box<dyn Error>> =
            build_message(input.as_str(), nick_clone.as_str());

        match my_message {
            Ok(m) => tx.send(m).unwrap(),
            Err(e) => {
                eprintln!("{:?}", e);
                print_nick(nick_clone.as_str(), None)
            }
        }
    });

    let receive_and_send = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(50));
        if let Ok(m) = handle_client(&mut connection) {
            let (nick_incoming, message, timestamp) = m.destructure();

            clear_last_line();

            match message {
                MessageType::Text(text) => {
                    print_nick(nick_incoming.as_str(), Some(timestamp));
                    println!("{}", text.trim_end_matches(|c| c == '\n'));
                }
                MessageType::Image(_) => {
                    println!("{} Receiving image from {}", timestamp, nick_incoming);
                    receive_and_save(message, nick_incoming).unwrap();
                }
                MessageType::File(name, content) => {
                    println!("{} Receiving {} from {}", timestamp, name, nick_incoming);
                    // to be able to destructure the file and print it's name
                    receive_and_save(MessageType::File(name, content), nick_incoming).unwrap();
                }
            }
            print_nick(nick.as_str(), None)
        }
        while let Ok(my_message) = rx.try_recv() {
            if let Err(e) = send_message(&mut connection, &my_message) {
                println!("{:?}", e)
            };
        }
    });

    reading_thread.join().unwrap();
    receive_and_send.join().unwrap();
}

fn receive_and_save(message: MessageType, nick: String) -> Result<(), Box<dyn Error>> {
    let path = format!("media/{}", nick);
    std::fs::create_dir_all(&path)?;

    match message {
        MessageType::File(name, file_content) => {
            let file_path = Path::new(&path).join(name);
            std::fs::write(file_path, file_content)?;
        }
        MessageType::Image(data) => {
            let timestamp = full_now();
            let file_path = Path::new(&path).join(timestamp);
            std::fs::write(file_path, data)?;
        }
        _ => (),
    }

    Ok(())
}

fn print_nick(nick: &str, timestamp: Option<String>) {
    let time = match timestamp {
        Some(t) => t,
        None => " ".repeat(8),
    };
    print!("{} {}: ", time, nick);
    io::stdout().flush().unwrap();
    move_to_message()
}

fn clear_last_line() {
    execute!(
        io::stdout(),
        cursor::MoveToColumn(0),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )
    .expect("Failed to clear last line");
}

fn move_to_message() {
    execute!(io::stdout(), cursor::MoveToColumn(28),).expect("Failed to move cursor")
}

fn replace_last_line(nick: &str, input: &str) {
    execute!(
        io::stdout(),
        cursor::MoveUp(1),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )
    .expect("Failed to clear last line");
    print!("{} {}: ", now(), nick);
    move_to_message();
    print!("{}", input);
    print_nick(nick, None)
}

fn build_message(input: &str, nick: &str) -> Result<Message, Box<dyn Error>> {
    if input.starts_with(".quit") {
        std::process::exit(0)
    } else if input.starts_with(".file") {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() > 1 {
            let path = Path::new(parts[1].trim_end());
            let mut file = std::fs::File::open(path)?;
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents)?;

            let file_name = if let Some(n) = path.file_name() {
                n
            } else {
                return Err(Box::new(std::io::Error::new(
                    io::ErrorKind::NotFound,
                    "invalid path",
                )));
            };
            replace_last_line(
                nick,
                format!("Sending {}\n", file_name.to_string_lossy()).as_str(),
            );
            Ok(Message::new(
                nick.to_string(),
                MessageType::File(file_name.to_string_lossy().to_string(), file_contents),
            ))
        } else {
            Err(Box::new(std::io::Error::new(
                io::ErrorKind::NotFound,
                "provide a path to file",
            )))
        }
    } else if input.starts_with(".image") {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() > 1 {
            let path = Path::new(parts[1]);
            let mut file = std::fs::File::open(path)?;
            let mut file_contents = vec![];
            file.read_to_end(&mut file_contents)?;

            replace_last_line(nick, "Sending image...");
            Ok(Message::new(
                nick.to_string(),
                MessageType::Image(file_contents),
            ))
        } else {
            Err(Box::new(std::io::Error::new(
                io::ErrorKind::NotFound,
                "provide a path to file",
            )))
        }
    } else {
        replace_last_line(nick, input);
        Ok(Message::new(
            nick.to_string(),
            MessageType::Text(input.to_string()),
        ))
    }
}

// todo errors
// todo try parse messagetype to send other stuff than text
// .png
