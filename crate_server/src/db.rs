use std::sync::Arc;

use flume::{Receiver, Sender};
use message::{HistoryDirection::*, Task};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

pub async fn database_task(rx: Receiver<Task>, tx: Sender<Task>, pool: Arc<Mutex<Pool<Postgres>>>) {
    let lock = &*pool.lock().await;

    loop {
        while let Ok(task) = rx.recv_async().await {
            match task {
                Task::Message(m) => {
                    log::info!("into db message from {}", m.username);
                    match sqlx::query(
                        "INSERT INTO rusty_app_message (message, nick) VALUES ($1, $2)",
                    )
                    .bind(m.message)
                    .bind(m.username)
                    .execute(lock)
                    .await
                    {
                        Ok(_) => log::info!("message inserted"),
                        Err(e) => log::error!("{e}"),
                    };
                }
                Task::User => log::info!("we gotta user"),
                Task::History(_) => {
                    if tx.send(get_history()).is_err() {
                        log::error!("issue returning history")
                    };
                }
            }
        }
    }
}

fn get_history() -> Task {
    Task::History(Response)
}
