use flume::{Receiver, Sender};
use message::ChatError;
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

    log::info!("test db task: {:?}", rx_user.recv());

    loop {
        while let Ok(dt) = rx_user.recv() {
            match dt {
                DatabaseTask::LoginRequest((nick, pass)) => {
                    let login_result = login_db(&nick, &pass, &pool).await;
                    if tx_user_confirm
                        .send(DatabaseTask::LoginConfirmation(login_result))
                        .is_err()
                    {
                        log::error!("couldnt send Login confirmation")
                    };
                }
                DatabaseTask::Message((message, user_id)) => {
                    message_into_db(&pool, user_id, &message).await;
                }
                _ => log::info!("Something fishy coming into database task"),
            }
        }
    }
}
