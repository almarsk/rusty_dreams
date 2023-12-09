use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;
use tokio::{io::WriteHalf, net::TcpStream};

use message::{send_message, Addressee::*, MaybeSerializedMessage::*};

type Clients = Arc<Mutex<HashMap<SocketAddr, WriteHalf<TcpStream>>>>;

pub async fn broadcast_message<'a>(address: SocketAddr, message: Vec<u8>, clients: &Clients) {
    let mut clients = clients.lock().unwrap();
    let clients_to_remove: Vec<SocketAddr> = vec![];
    clients
        .iter_mut()
        .filter(|(a, _)| **a != address)
        .for_each(|(a, writer)| {
            block_on(async {
                if let Err(e) = send_message(writer, Serialized(message.clone()), Client(a)).await {
                    log::error!("Error sending: {}", e)
                };
            });
        });
    clients_to_remove.into_iter().for_each(|c| {
        log::error!("removing {}", c);
        clients.remove(&c);
    })
}
