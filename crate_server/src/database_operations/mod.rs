use flume::{Receiver, Sender};
use message::{ChatError, Message};
use sqlx::postgres::PgPoolOptions;

use crate::task_type::DatabaseTask;

mod check_db;
use check_db::login_db;
mod message_into_db;
use message_into_db::message_into_db;

pub async fn database_operations(
    rx_user: Receiver<DatabaseTask>,
    tx_user_confirm: Sender<DatabaseTask>,
) -> Result<(), ChatError> {
    // database setup
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").map_err(|_| ChatError::DatabaseIssue)?;
    log::info!("{}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| anyhow::Error::new(e).context("Error connecting to database"))
        .map_err(|_| ChatError::DatabaseIssue)?;

    sqlx::query(
        r#"
   CREATE TABLE IF NOT EXISTS rusty_app_user (
     id SERIAL PRIMARY KEY,
     nick text,
     pass text
   );"#,
    )
    .execute(&pool)
    .await
    .map_err(|_| ChatError::DatabaseIssue)?;

    sqlx::query(
        r#"
   CREATE TABLE IF NOT EXISTS rusty_app_message (
     id SERIAL PRIMARY KEY,
     message TEXT,
     user_id SERIAL REFERENCES rusty_app_user(id)
   );"#,
    )
    .execute(&pool)
    .await
    .map_err(|_| ChatError::DatabaseIssue)?;

    log::info!("databse all setup");

    loop {
        log::info!("waiting for databse task");
        if let Ok(dt) = rx_user.recv_async().await {
            log::info!("new db task");
            match dt {
                DatabaseTask::LoginRequest((nick, pass)) => {
                    let login_result = login_db(&nick, &pass, &pool).await;
                    if tx_user_confirm
                        .send_async(DatabaseTask::LoginConfirmation(login_result))
                        .await
                        .is_err()
                    {
                        log::error!("couldnt send Login confirmation")
                    };
                }
                DatabaseTask::Message((message, user_id)) => {
                    log::info!("its a message");

                    let message = Message::deserialize(&message)
                        .map_err(|_| ChatError::DeserializingIssue)?
                        .into_db();

                    if let Some(m) = message {
                        message_into_db(&pool, user_id, &m).await;
                    } else {
                        log::info!("Something fishy coming into database task");
                        return Err(ChatError::WritingIssue);
                    }
                }
                _ => log::info!("Something fishy coming into database task"),
            }
        } else {
            log::error!("issue writing message in db")
        }
    }
}
