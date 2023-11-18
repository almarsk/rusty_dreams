use clap::Parser;
use crossterm::{cursor, execute, terminal};
use env_logger::Builder;

use std::error::Error;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::{thread, time::Duration};

use message::{chat_time_now, full_time_now, handle_client, send_message, Message, MessageType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = names::Generator::default().next().unwrap())]
    nick: String,
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String, String) {
        (self.host, self.port, self.nick)
    }
}

fn main() {
    let (host, port, nick) = Args::parse().deconstruct();

    Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            )
        })
        .init();

    send_and_receive(host, port, nick)
}

fn send_and_receive(host: String, port: String, nick: String) {
    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        log::error!("cant use {}:{}, going default", host, port);
        "127.0.0.1:11111".parse().unwrap()
    };

    let nick_owned: Arc<String> = Arc::new(nick);

    let mut connection = match TcpStream::connect(address) {
        Ok(t) => t,
        Err(e) => {
            log::error!("{e}");
            std::process::exit(1)
        }
    };

    let nick_read = nick_owned.clone();

    connection
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let (tx, rx) = mpsc::channel();

    print_nick(nick_read.as_str(), None);

    let reading_thread = thread::spawn(move || loop {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let my_message: Result<Message, Box<dyn Error>> =
            get_msg(input.as_str(), nick_read.as_str());

        match my_message {
            Ok(m) => tx.send(m).unwrap(),
            Err(e) => {
                log::error!("{}", e);
                print_nick(nick_read.clone().as_str(), None)
            }
        }
    });

    let nick_receive = nick_owned.clone();

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
                    receive_and_save(message, nick_receive.as_str()).unwrap();
                }
                MessageType::File(name, content) => {
                    println!("{} Receiving {} from {}", timestamp, name, nick_incoming);
                    // to be able to destructure the file and print it's name
                    receive_and_save(MessageType::File(name, content), nick_receive.as_str())
                        .unwrap();
                }
            }
            print_nick(nick_receive.as_str(), None)
        }
        while let Ok(my_message) = rx.try_recv() {
            if let Err(e) = send_message(&mut connection, &my_message) {
                log::error!("{}", e);
                print_nick(nick_receive.as_str(), None)
            };
        }
    });

    reading_thread.join().unwrap();
    receive_and_send.join().unwrap();
}

// _______Functions_______

fn receive_and_save(message: MessageType, nick: &str) -> Result<(), Box<dyn Error>> {
    let path = format!("media/users/{}", nick);
    std::fs::create_dir_all(&path)?;

    match message {
        MessageType::File(name, file_content) => {
            let file_path = Path::new(&path).join(name);
            std::fs::write(file_path, file_content)?;
        }
        MessageType::Image(data) => {
            let timestamp = full_time_now();
            let file_path = Path::new(&path).join(timestamp);
            std::fs::write(format!("{}.png", file_path.to_string_lossy()), data)?;
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
    execute!(io::stdout(), cursor::MoveToColumn(30),).expect("Failed to move cursor")
}

fn clear_previous_line() {
    execute!(
        io::stdout(),
        cursor::MoveUp(1),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )
    .expect("Failed to clear last line");
}

fn replace_last_line(nick: &str, input: &str) {
    clear_previous_line();
    print!("{} {}: ", chat_time_now(), nick);
    move_to_message();
    print!("{}", input);
    print_nick(nick, None)
}

fn get_msg(input: &str, nick: &str) -> Result<Message, Box<dyn Error>> {
    if input == ".quit\n" {
        // this is probably not the best way to go about this
        std::process::exit(0)
    } else if input.starts_with(".file ") {
        construct_message(nick, input, MessageType::File("".to_string(), vec![]))
    } else if input.starts_with(".image ") {
        construct_message(nick, input, MessageType::Image(vec![]))
    } else {
        replace_last_line(nick, input);
        Ok(Message::new(
            nick.to_string(),
            MessageType::Text(input.to_string()),
        ))
    }
}

fn construct_message(
    nick: &str,
    input: &str,
    mess_type: MessageType,
) -> Result<Message, Box<dyn Error>> {
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
        let content = match mess_type {
            MessageType::File(_, _) => {
                MessageType::File(file_name.to_string_lossy().to_string(), file_contents)
            }
            MessageType::Image(_) => MessageType::Image(file_contents),
            _ => {
                log::warn!("something fishy is goin on");
                MessageType::Text("".to_string())
            }
        };
        Ok(Message::new(nick.to_string(), content))
    } else {
        Err(Box::new(std::io::Error::new(
            io::ErrorKind::NotFound,
            "provide a path to file",
        )))
    }
}
