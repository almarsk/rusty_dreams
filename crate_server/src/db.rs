use std::sync::Arc;

use flume::{Receiver, Sender};
use message::{HistoryDirection::*, LoginDirection, Message, Task};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use super::auth::check_login;

pub async fn database_task(rx: Receiver<Task>, tx: Sender<Task>, pool: Arc<Mutex<Pool<Postgres>>>) {
    let lock = &*pool.lock().await;

    while let Ok(task) = rx.recv_async().await {
        log::info!("task arrived to db");

        match task {
            Task::Message(m) => {
                log::info!("into db message from {}", m.username);
                match sqlx::query("INSERT INTO rusty_app_message (message, nick) VALUES ($1, $2)")
                    .bind(m.message)
                    .bind(m.username)
                    .execute(lock)
                    .await
                {
                    Ok(_) => log::info!("message inserted"),
                    Err(e) => log::error!("{e}"),
                };
            }
            Task::User(message::LoginDirection::Request(login_attempt)) => {
                log::info!("we gotta user: {:?}", login_attempt);

                log::info!("T O D O validate user");

                if tx
                    .send_async(Task::User(LoginDirection::Response(
                        check_login(login_attempt, lock).await,
                    )))
                    .await
                    .is_err()
                {
                    log::error!("issue returning login result")
                };
            }
            Task::History(_) => {
                if tx.send_async(get_history(lock).await).await.is_err() {
                    log::error!("issue returning history")
                };
            }
            _ => {
                log::error!("something fishy")
            }
        }
        log::info!("db task done, waiting for next one");
    }
}

async fn get_history(lock: &Pool<Postgres>) -> Task {
    match sqlx::query!("SELECT * FROM rusty_app_message")
        .fetch_all(lock)
        .await
    {
        Err(_) => Task::History(Response(vec![])),
        Ok(records) => Task::History(Response(
            records
                .into_iter()
                .map(|r| Message {
                    username: r.nick.unwrap_or("user".to_string()),
                    message: r.message.unwrap_or("...".to_string()),
                })
                .collect::<Vec<Message>>(),
        )),
    }
}
