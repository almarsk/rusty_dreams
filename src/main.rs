use crossterm::{cursor, execute, terminal};
use rusty_dreams::now;
use rusty_dreams::{handle_client, send_message, Message, MessageType};
use std::io;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::{thread, time::Duration};

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
        Some(n) => Arc::new(n),
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
        if input == ".quit\n" {
            std::process::exit(0)
        };

        replace_last_line(nick_clone.as_str(), input.as_str());

        let nick_outgoing = format!("{}", nick_clone);
        let my_message = Message::new(nick_outgoing, MessageType::Text(input));
        tx.send(my_message).unwrap()
    });

    let receive_and_send = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(50));
        if let Ok(m) = handle_client(&mut connection) {
            let local_addr: String = if let Ok(la) = connection.local_addr() {
                la.to_string()
            } else {
                "default".to_string()
            };

            let (nick_incoming, message, timestamp) = m.destructure();

            clear_last_line();

            match message {
                MessageType::Text(text) => {
                    print_nick(nick_incoming.as_str(), Some(timestamp));
                    println!("{}", text.trim_end_matches(|c| c == '\n'));
                }
                MessageType::Image(_) => {
                    println!("{} Receiving image from {}", timestamp, nick_incoming);
                    receive_and_save(message, local_addr)
                }
                MessageType::File(name, content) => {
                    println!("{} Receiving {} from {}", timestamp, name, nick_incoming);
                    // to be able to destructure the file and print it's name
                    receive_and_save(MessageType::File(name, content), local_addr)
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

fn receive_and_save(message: MessageType, local_address: String) {
    println!("{:?}", message);
    println!("{}", local_address)
}

fn print_nick(nick: &str, timestamp: Option<String>) {
    let time = match timestamp {
        Some(t) => t,
        None => now(),
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

// todo make folder by ip address
// image is saved in it with timestamp .png (find the crate)
// todo errors
// todo try parse messagetype to send other stuff than text
