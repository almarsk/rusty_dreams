use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use flume::Receiver;
use tokio::{io::WriteHalf, net::TcpStream};

use super::task_type::Task;

mod broadcast_message;
use broadcast_message::broadcast_message;

pub async fn accomodate_and_broadcast(rx_accomodate: Receiver<Task>, rx_broadcast: Receiver<Task>) {
    let clients: Arc<Mutex<HashMap<SocketAddr, WriteHalf<TcpStream>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let clients_a = clients.clone();

    // accomodation task
    tokio::task::spawn(async move {
        loop {
            while let Ok(t) = rx_accomodate.try_recv() {
                match t {
                    Task::ConnWrite(a, c) => {
                        if let Ok(mut h) = clients_a.clone().try_lock() {
                            h.insert(a, c);
                            // todo add user to user table
                        } else {
                            log::error!("Couldnt accomodate writer {}", a)
                        }
                    }
                    _ => log::error!("Something else than Writehal coming in accomodating task"),
                }
            }
        }
    });

    // broadcasting task
    tokio::task::spawn(async move {
        loop {
            while let Ok(t) = rx_broadcast.try_recv() {
                match t {
                    Task::Message(a, m) => broadcast_message(a, m, &clients).await,
                    _ => {
                        log::error!("Something else than message being sent to broadcasting task")
                    }
                }
            }
        }
    });
}
