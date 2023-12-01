use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use flume::Receiver;
use tokio::io::WriteHalf;

use super::task::Task;

mod broadcast_message;
use broadcast_message::broadcast_message;

#[allow(clippy::needless_lifetimes)] // turning off this lint because the compiler is angy when i obey it
pub async fn accomodate_and_broadcast<'a>(_rx_accomodate: Receiver<Task>, rx: Receiver<Task>) {
    let clients: Arc<Mutex<HashMap<SocketAddr, WriteHalf<Vec<u8>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // spawn a future per rx
    //
    // accomodation future should work with the arc and mutex
    //
    // broadcasting message has a function ready

    while let Ok(t) = rx.try_recv() {
        match t {
            Task::Message(a, m) => {
                // todo broadcasting
                broadcast_message(a, m, &clients).await
            }
            Task::ConnWrite(a, c) => {
                clients.lock().unwrap().insert(a, c);
            }
            _ => {
                println!("yoyo")
            }
        }
    }
}
