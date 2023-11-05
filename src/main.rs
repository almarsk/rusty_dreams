use rusty_dreams::{handle_client, send_message, MessageType};
use std::io;
use std::net::TcpStream;

fn main() {
    let server_address = "127.0.0.1:11111";
    let mut connection = TcpStream::connect(server_address).unwrap();

    loop {
        dbg!("collect and print");
        if let Ok(m) = handle_client(&mut connection) {
            println!("{:?}", m)
        }

        dbg!("read");
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).expect("Failed to read line");
        let my_message = MessageType::Text(input);
        dbg!("send");
        send_message(&mut connection, &my_message);
    }
}
