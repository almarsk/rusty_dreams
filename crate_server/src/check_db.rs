use std::sync::Arc;

use message::ChatError;
use sqlx::PgPool;

pub async fn login_db(nick: &str, pass: &str, pool: Arc<PgPool>) -> Result<(), ChatError> {
    // check db

    match sqlx::query!("SELECT * FROM rusty_app_user WHERE nick = $1", nick)
        .fetch_one(&*pool)
        .await
    {
        Err(_) => {
            log::info!("need to make a new user");
            sqlx::query!(
                "INSERT INTO rusty_app_user (nick, pass) VALUES ($1, $2)",
                nick,
                pass
            )
            .execute(&*pool)
            .await
            .map_err(|_| ChatError::DatabaseIssue)?;
            Ok(())
        }
        Ok(record) => {
            log::info!("user exists, lets check if pass ok");
            if let Some(db_pass) = record.pass {
                if db_pass == pass {
                    log::info!("it is");
                    Ok(())
                } else {
                    log::info!("wrong password {}", nick);
                    Err(ChatError::LoginIssue)
                }
            } else {
                Ok(())
            }
        }
    }
}
