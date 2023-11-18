use clap::Parser;

use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use message::{handle_client, send_message, Message, MessageType};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String) {
        (self.host, self.port)
    }
}

fn main() {
    let (host, port) = Args::parse().deconstruct();

    if let Err(e) = simple_logger::SimpleLogger::new().env().init() {
        log::error!("{}", e);
        std::process::exit(1)
    } else {
        listen_and_broadcast(host, port)
    }
}

fn listen_and_broadcast(host: String, port: String) {
    let (tx, rx) = mpsc::channel();

    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        "127.0.0.1:11111".parse().unwrap()
    };

    let listener_thread = thread::spawn(move || {
        // I guess as it stands if this fails, I want to exit
        let listener = match TcpListener::bind(address) {
            Ok(t) => t,
            Err(e) => {
                log::error!("{}", e);
                std::process::exit(1)
            }
        };

        log::info!("server started on {}", address);

        for connection in listener.incoming() {
            let connection: TcpStream = if let Ok(c) = connection {
                c
            } else {
                continue;
            };
            let addr = connection.peer_addr().unwrap();
            log::info!("connection found, {}", addr);
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
                        //log::info!("{:?}", message);
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

    let message_type = match message.content {
        MessageType::Text(_) => "text",
        MessageType::Image(_) => "image",
        MessageType::File(_, _) => "file",
    };

    log::info!("broadcasting {} from {}", message_type, sender_address);

    clients
        .iter_mut()
        .filter(|c| *c.0 != sender_address)
        .for_each(|(address, connection)| {
            if send_message(connection, message).is_err() {
                clients_to_remove.push(*address);
                // not sure why this prints only the second time after send_message is attempted to a closed tcpstream
                log::info!("removing {}", address);
            }
        });

    clients_to_remove.into_iter().for_each(|invalid_conn| {
        clients.remove(&invalid_conn);
    })
}
