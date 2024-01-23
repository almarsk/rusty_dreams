use std::sync::Arc;

use flume::{Receiver, Sender};
use message::{LoginDirection, Message, Task, TaskDirection::*};
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use super::auth::check_login;

pub async fn database_task(rx: Receiver<Task>, tx: Sender<Task>, pool: Arc<Mutex<Pool<Postgres>>>) {
    let lock = &*pool.lock().await;

    while let Ok(task) = rx.recv_async().await {
        log::info!("task arrived to db {:?}", task);

        match task {
            Task::Message(m) => {
                log::info!("into db message from {}", m.username);
                match sqlx::query(
                    "INSERT INTO rusty_app_message (message, nick, date) VALUES ($1, $2, $3)",
                )
                .bind(m.message)
                .bind(m.username)
                .bind(m.date)
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
            Task::Mannschaft(_) => {
                if tx.send_async(get_mannschaft(lock).await).await.is_err() {
                    log::error!("issue returning history")
                };
            }
            Task::Delete(user) => delete_user(lock, user).await,
            _ => {
                log::error!("something fishy")
            }
        }
        log::info!("db task done, waiting for next one");
    }
}

async fn get_history(lock: &Pool<Postgres>) -> Task {
    let deleted_users: Vec<_> = match sqlx::query!("SELECT * FROM rusty_app_user")
        .fetch_all(lock)
        .await
    {
        Err(_) => return Task::History(Response(vec![])),
        Ok(records) => records
            .into_iter()
            .filter_map(|r| {
                if r.deleted.unwrap_or_default() {
                    r.nick
                } else {
                    None
                }
            })
            .collect(),
    };

    match sqlx::query!("SELECT * FROM rusty_app_message")
        .fetch_all(lock)
        .await
    {
        Err(_) => Task::History(Response(vec![])),
        Ok(records) => Task::History(Response(
            records
                .into_iter()
                .map(|r| {
                    let username = r.nick.unwrap_or("user".to_string());

                    Message {
                        deleted: deleted_users.contains(&username),
                        username,
                        message: r.message.unwrap_or("...".to_string()),
                        date: r.date.unwrap_or("?????".to_string()),
                    }
                })
                .collect::<Vec<Message>>(),
        )),
    }
}

async fn get_mannschaft(lock: &Pool<Postgres>) -> Task {
    match sqlx::query!("SELECT * FROM rusty_app_user")
        .fetch_all(lock)
        .await
    {
        Err(_) => Task::Mannschaft(Response(vec![])),
        Ok(records) => Task::Mannschaft(Response(
            records
                .into_iter()
                .map(|r| r.nick.unwrap_or_default())
                .collect::<Vec<String>>(),
        )),
    }
}

async fn delete_user(lock: &Pool<Postgres>, user: String) {
    log::info!("lets delete {}", user);
    match sqlx::query!(
        "UPDATE rusty_app_user
    SET deleted = true
    WHERE nick = $1",
        user
    )
    .fetch_all(lock)
    .await
    {
        Err(_) => log::error!("couldnt delete user {}", user),
        Ok(_) => log::info!("user {} deleted", user),
    }
}
