use rusty_dreams::{handle_client, send_message, MessageType};
use std::io;
use std::net::TcpStream;
use std::sync::mpsc;
use std::{thread, time::Duration};

fn main() {
    let server_address = "127.0.0.1:11111";
    let mut connection = TcpStream::connect(server_address).unwrap();
    connection
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let (tx, rx) = mpsc::channel();

    let reading_thread = thread::spawn(move || loop {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let my_message = MessageType::Text(input);
        tx.send(my_message).unwrap()
    });

    let receive_and_send = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(50));
        if let Ok(m) = handle_client(&mut connection) {
            println!("{:?}", m)
        }
        while let Ok(my_message) = rx.try_recv() {
            send_message(&mut connection, &my_message);
        }
    });

    reading_thread.join().unwrap();
    receive_and_send.join().unwrap();
}
