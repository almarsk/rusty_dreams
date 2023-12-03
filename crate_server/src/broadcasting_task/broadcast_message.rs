use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;
use tokio::{
    io::{AsyncWriteExt, WriteHalf},
    net::TcpStream,
};

type Clients = Arc<Mutex<HashMap<SocketAddr, WriteHalf<TcpStream>>>>;

pub async fn broadcast_message<'a>(address: SocketAddr, message: Vec<u8>, clients: &Clients) {
    let mut clients = clients.lock().unwrap();
    let mut clients_to_remove: Vec<SocketAddr> = vec![];
    clients
        .iter_mut()
        .filter(|(a, _)| **a != address)
        .for_each(|(a, c)| {
            block_on(async {
                if let Err(e) = c.write_all(&message).await {
                    log::error!("sending to {} failed: {}", a, e);
                    clients_to_remove.push(*a)
                }
            });
        });
    clients_to_remove.into_iter().for_each(|c| {
        log::error!("removing {}", c);
        clients.remove(&c);
    })
}
