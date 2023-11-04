use rusty_dreams::{handle_client, send_message, MessageType};
use std::io;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let server_address = "127.0.0.1:11111";
    let connection = TcpStream::connect(server_address).unwrap();

    /*
    let connection_wrap = Arc::new(Mutex::new(connection));
    let connection_clone = Arc::clone(&connection_wrap);

    thread::spawn(move || loop {
        if let Ok(mut stream) = connection_clone.lock() {
            let result = handle_client(&mut stream);
            println!("{:?}", result);
        }
    });
    */

    let mut stream_to_send = connection; //_wrap.lock().unwrap();
    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input).expect("Failed to read line");
        let my_message = MessageType::Text(input);
        dbg!(&my_message);

        dbg!("sendin message");
        send_message(&mut stream_to_send, &my_message);
    }
}
