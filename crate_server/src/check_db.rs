use std::sync::Arc;

use message::ChatError;
use sqlx::PgPool;
use tokio::sync::Mutex;

pub async fn login_db(
    nick: &str,
    pass: &str,
    pool: Arc<Mutex<PgPool>>,
) -> Result<(i32, bool), ChatError> {
    // check db
    let lock = pool.lock().await;

    match sqlx::query!("SELECT * FROM rusty_app_user WHERE nick = $1", nick)
        .fetch_one(&*lock)
        .await
    {
        Err(_) => {
            log::info!("need to make a new user");
            let record = sqlx::query!(
                "INSERT INTO rusty_app_user (nick, pass) VALUES ($1, $2) RETURNING id",
                nick,
                pass
            )
            .fetch_one(&*lock)
            .await
            .map_err(|_| ChatError::DatabaseIssue)?;

            Ok((record.id, false))
        }
        Ok(record) => {
            log::info!("user exists, lets check if pass ok");
            if let Some(db_pass) = record.pass {
                if db_pass == pass {
                    log::info!("it is");
                    Ok((record.id, true))
                } else {
                    log::info!("wrong password {}", nick);
                    Err(ChatError::LoginIssue)
                }
            } else {
                Ok((record.id, true))
            }
        }
    }
}

pub async fn history(pool: Arc<Mutex<PgPool>>) -> String {
    let lock = pool.lock().await;

    match sqlx::query!("SELECT * FROM rusty_app_message")
        .fetch_all(&*lock)
        .await
    {
        Err(_) => "".to_string(),
        Ok(records) => records.into_iter().fold(String::from(""), |mut acc, r| {
            if let Some(message) = r.message {
                if let Some(nick) = r.nick {
                    acc.push_str(format!("\n{}: {}", nick, message).as_str());
                }
            }
            acc
        }),
    }
}
