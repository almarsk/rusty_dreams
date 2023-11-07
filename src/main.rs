use rusty_dreams::{handle_client, send_message, MessageType};
use std::io;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::mpsc;
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

    let mut connection = TcpStream::connect(address).unwrap();
    connection
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let (tx, rx) = mpsc::channel();

    let reading_thread = thread::spawn(move || loop {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        if input == ".quit\n" {
            std::process::exit(0)
        }
        let my_message = MessageType::Text(input);
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

            match m {
                MessageType::Text(t) => println!("{t}"),
                MessageType::Image(_) => {
                    println!("Receiving image...");
                    receive_and_save(m, local_addr)
                }
                MessageType::File(name, content) => {
                    println!("Receiving {name}...");
                    // to be able to destructure the file and print it's name
                    receive_and_save(MessageType::File(name, content), local_addr)
                }
            }
        }
        while let Ok(my_message) = rx.try_recv() {
            send_message(&mut connection, &my_message);
        }
    });

    reading_thread.join().unwrap();
    receive_and_send.join().unwrap();
}

// todo make folder by ip address
// image is saved in it with timestamp .png (find the crate)
fn receive_and_save(message: MessageType, local_address: String) {
    println!("{:?}", message);
    println!("{}", local_address)
}

// todo errors
// todo try parse messagetype to send other stuff than text
// put messagetype into a struct with timestamp and username
// print all of those things
