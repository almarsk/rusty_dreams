use std::collections::HashMap;
//use std::io::{self, ErrorKind};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use std::time::Duration;

use rusty_dreams::{handle_client, send_message, MessageType};

fn main() {
    listen_and_broadcast("127.0.0.1:11111".parse().unwrap())
}

fn listen_and_broadcast(address: SocketAddr) {
    let (tx, rx) = mpsc::channel();

    let listener_thread = thread::spawn(move || {
        let listener = TcpListener::bind(address).expect("Failed to bind to address");

        for connection in listener.incoming() {
            eprintln!("connection found");
            let connection = connection.unwrap(); // todo
            let addr = connection.peer_addr().unwrap();
            tx.send((addr, connection)).unwrap();
        }
    });

    let handler_thread = thread::spawn(move || {
        let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

        loop {
            while let Ok((addr, connection)) = rx.try_recv() {
                connection
                    .set_nonblocking(true)
                    .expect("set_nonblocking call failed");
                clients.insert(addr, connection);
            }

            let messages: Vec<(SocketAddr, MessageType)> = clients
                .iter_mut()
                .filter_map(|(addr, connection)| match handle_client(connection) {
                    Ok(message) => {
                        println!("{:?}", message);
                        Some((*addr, message))
                    }
                    Err(_) => None,
                })
                .collect();

            for (sender, message) in messages {
                broadcast_message(&mut clients, &message, sender);
            }

            thread::sleep(Duration::from_millis(50))
        }
    });

    listener_thread.join().unwrap();
    handler_thread.join().unwrap();
}

fn broadcast_message(
    clients: &mut HashMap<SocketAddr, TcpStream>,
    message: &MessageType,
    sender_address: SocketAddr,
) {
    clients
        .iter_mut()
        .filter(|c| *c.0 != sender_address)
        .for_each(|(_, connection)| send_message(connection, message))
}
