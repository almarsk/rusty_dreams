use message::{
    logging_in::{LoginAttempt, LoginResult},
    User,
};
use sqlx::{Pool, Postgres};

pub async fn check_login(login_attempt: LoginAttempt, lock: &Pool<Postgres>) -> LoginResult {
    let (nick, pass) = login_attempt.dec();

    match sqlx::query!("SELECT * FROM rusty_app_user WHERE nick = $1", nick)
        .fetch_one(lock)
        .await
    {
        Err(_) => {
            log::info!("need to make a new user");
            // could work with whether there even should be a new user
            if sqlx::query!(
                "INSERT INTO rusty_app_user (nick, pass) VALUES ($1, $2) RETURNING id",
                nick,
                pass
            )
            .fetch_one(lock)
            .await
            .is_ok()
            {
                LoginResult::NewUser(User { nick })
            } else {
                LoginResult::InternalError
            }
        }
        Ok(record) => {
            log::info!("user exists, lets check if pass ok");
            if let Some(db_pass) = record.pass {
                if db_pass == pass {
                    log::info!("it is");
                    LoginResult::ReturningUser(User { nick })
                } else {
                    log::info!("wrong password {}", nick);
                    LoginResult::WrongPassword
                }
            } else {
                LoginResult::InternalError
            }
        }
    }
}
