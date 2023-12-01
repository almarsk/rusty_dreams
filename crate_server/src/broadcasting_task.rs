use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use flume::Receiver;
use tokio::{io::AsyncWriteExt, net::tcp::WriteHalf};

use super::task::Task;

pub async fn accomodate_and_broadcast<'a>(rx: Receiver<Task<'a>>) {
    let clients: Arc<Mutex<HashMap<SocketAddr, WriteHalf>>> = Arc::new(Mutex::new(HashMap::new()));

    while let Ok(t) = rx.try_recv() {
        match t {
            Task::Message(a, m) => {
                // todo broadcasting
                broadcast_message(a, m, &clients).await
            }
            Task::Connection(a, c) => {
                clients.lock().unwrap().insert(a, c);
            }
        }
    }
}

async fn broadcast_message<'a>(
    address: SocketAddr,
    message: Vec<u8>,
    clients: &Arc<Mutex<HashMap<SocketAddr, WriteHalf<'a>>>>,
) {
    let mut clients = clients.lock().unwrap();

    clients
        .iter_mut()
        .filter(|(a, _)| **a != address)
        .for_each(|(_, c)| {
            async {
                if let Err(e) = c.write_all(&message).await {
                    eprintln!("sending failed: {}", e);
                }
            };
        });
}
