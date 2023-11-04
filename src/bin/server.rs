use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};

use rusty_dreams::{handle_client, send_message, MessageType};

fn main() {
    listen_and_broadcast("127.0.0.1:11111", true)
}

fn listen_and_broadcast(address: &str, broadcast: bool) {
    let listener = TcpListener::bind(address).expect("Failed to bind to address");

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    for connection in listener.incoming() {
        let connection = connection.unwrap(); // todo
        let addr = connection.peer_addr().unwrap();
        //connection.set_nonblocking(true).unwrap(); // todo
        clients.insert(addr, connection);

        let mut connection_wrapper = clients.get(&addr).unwrap().try_clone().unwrap();

        let my_message = handle_client(&mut connection_wrapper); // todo

        if broadcast {
            broadcast_message(&mut clients, &my_message, addr);
            println!("{:?}", clients);
            println!("{:?}", my_message); //remove l8r
        } else {
            println!("{:?}", my_message);
        }
    }
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
