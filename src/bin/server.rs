use std::collections::HashMap;
//use std::io::{self, ErrorKind};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use std::time::Duration;

use rusty_dreams::{handle_client, send_message, Message};

fn main() {
    listen_and_broadcast("0.0.0.0", "11111")
}

fn listen_and_broadcast(host: &str, port: &str) {
    let (tx, rx) = mpsc::channel();

    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        "127.0.0.1:11111".parse().unwrap()
    };

    let listener_thread = thread::spawn(move || {
        // I guess as it stands if this fails, I really do want to panic
        let listener = TcpListener::bind(address).expect("Failed to bind to address");

        for connection in listener.incoming() {
            let connection: TcpStream = if let Ok(c) = connection {
                c
            } else {
                continue;
            };
            let addr = connection.peer_addr().unwrap();
            eprintln!("connection found, {}", addr);
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

            let messages: Vec<(SocketAddr, Message)> = clients
                .iter_mut()
                .filter_map(|(addr, connection)| match handle_client(connection) {
                    Ok(message) => {
                        //println!("{:?}", message);
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
    message: &Message,
    sender_address: SocketAddr,
) {
    let mut clients_to_remove: Vec<SocketAddr> = vec![];

    clients
        .iter_mut()
        .filter(|c| *c.0 != sender_address)
        .for_each(|(address, connection)| {
            if send_message(connection, message).is_err() {
                clients_to_remove.push(*address);
                println!("removing {}", address)
            }
        });

    clients_to_remove.into_iter().for_each(|invalid_conn| {
        clients.remove(&invalid_conn);
    })
}
