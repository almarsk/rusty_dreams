use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;
use tokio::io::{AsyncWriteExt, WriteHalf};

type Clients = Arc<Mutex<HashMap<SocketAddr, WriteHalf<Vec<u8>>>>>;

pub async fn broadcast_message<'a>(address: SocketAddr, message: Vec<u8>, clients: &Clients) {
    let mut clients = clients.lock().unwrap();

    clients
        .iter_mut()
        .filter(|(a, _)| **a != address)
        .for_each(|(_, c)| {
            block_on(async {
                if let Err(e) = c.write_all(&message).await {
                    eprintln!("sending failed: {}", e);
                }
            });
        });
}
