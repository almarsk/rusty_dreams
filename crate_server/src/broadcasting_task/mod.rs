#![allow(unused)]

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use dotenv::dotenv;
use flume::Receiver;

use message::{ChatError, Message};
use sqlx::postgres::PgPoolOptions;
use tokio::{io::WriteHalf, net::TcpStream, sync::Mutex as TokioMutex};

use super::task_type::Task;

mod broadcast_message;
use broadcast_message::broadcast_message;

pub async fn accomodate_and_broadcast(
    rx_accomodate: Receiver<Task>,
    rx_broadcast: Receiver<(Task, i32)>,
) {
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
            while let Ok(t) = rx_broadcast.recv() {
                match t.0 {
                    Task::Message(a, m) => {
                        log::info!("TODO!! SEND MESSAGE TO DATABASE TASK");

                        log::info!("new message lets broadcast");
                        broadcast_message(a, m.clone(), &clients).await;
                        log::info!("waiting fo the next message to broadcast");
                    }
                    _ => {
                        log::error!("Something else than message being sent to broadcasting task")
                    }
                }
            }
        }
    });
}
