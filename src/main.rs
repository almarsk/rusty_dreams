use rusty_dreams::{send_message, MessageType};
use std::io::{self, Read};
use std::net::TcpStream;
use std::thread;

fn main() {
    let server_address = "127.0.0.1:11111";
    let mut connection = TcpStream::connect(server_address).unwrap();

    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).expect("Failed to read line");
        let my_message = MessageType::Text(input);

        // thread::spawn(|| for message in connection.read() {});

        send_message(&mut connection, &my_message);
    }
}
