use rusty_dreams::{handle_client, send_message, MessageType};
use std::io::{self, BufRead};
use std::net::TcpStream;
use std::{thread, time::Duration};

fn main() {
    let server_address = "127.0.0.1:11111";
    let mut connection = TcpStream::connect(server_address).unwrap();
    connection
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    loop {
        thread::sleep(Duration::from_millis(50));
        if let Ok(m) = handle_client(&mut connection) {
            println!("{:?}", m)
        }

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");

        let my_message = MessageType::Text(input);
        send_message(&mut connection, &my_message);

        /*
        let input_lines = stdin.lock().lines();
        for line in input_lines {
            match line {
                Ok(input) => {
                    let my_message = MessageType::Text(input);
                    send_message(&mut connection, &my_message);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        */
    }
}
