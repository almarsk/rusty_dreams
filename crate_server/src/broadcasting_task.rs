use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use flume::Receiver;
use futures::executor::block_on;
use tokio::io::{AsyncWriteExt, WriteHalf};

use super::task::Task;

#[allow(clippy::needless_lifetimes)] // turning off this lint because the compiler is angy when i obey it
pub async fn accomodate_and_broadcast<'a>(_rx_accomodate: Receiver<Task>, rx: Receiver<Task>) {
    let mut clients: Arc<Mutex<HashMap<SocketAddr, WriteHalf<Vec<u8>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    while let Ok(t) = rx.try_recv() {
        match t {
            Task::Message(a, m) => {
                // todo broadcasting
                broadcast_message(a, m, &clients).await
            }
            Task::Conn_Write(a, c) => {
                clients.lock().unwrap().insert(a, c);
            }
            _ => {
                println!("yoyo")
            }
        }
    }
}

async fn broadcast_message<'a>(
    address: SocketAddr,
    message: Vec<u8>,
    clients: &Arc<Mutex<HashMap<SocketAddr, WriteHalf<Vec<u8>>>>>,
) {
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
