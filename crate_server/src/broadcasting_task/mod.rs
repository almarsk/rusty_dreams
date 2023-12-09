#![allow(unused)]

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use dotenv::dotenv;
use flume::Receiver;

use message::Message;
use sqlx::postgres::PgPoolOptions;
use tokio::{io::WriteHalf, net::TcpStream};

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

    /*
     dotenv().ok();
     let database_url = match std::env::var("DATABASE_URL") {
         Ok(d) => d,
         Err(_) => return,
     };
     let pool = match PgPoolOptions::new()
         .max_connections(5)
         .connect(&database_url)
         .await
     {
         Ok(p) => p,
         Err(_) => return,
     };
    */

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
            while let Ok(t) = rx_broadcast.try_recv() {
                match t.0 {
                    Task::Message(a, m) => {
                        broadcast_message(a, m.clone(), &clients).await;

                        /*
                        match sqlx::query!(
                            "INSERT INTO rusty_app_message (message, user_id) VALUES ($1, $2)",
                            &Message::deserialize(&m).unwrap().into_db(),
                            t.1
                        )
                        .execute(&pool)
                        .await
                        {
                            Ok(_) => (),
                            Err(e) => log::error!("{e}"),
                        };
                        */
                    }
                    _ => {
                        log::error!("Something else than message being sent to broadcasting task")
                    }
                }
            }
        }
    });
}
