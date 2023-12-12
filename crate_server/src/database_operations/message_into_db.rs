use message::Message;
use sqlx::{Pool, Postgres};

pub async fn message_into_db(pool: &Pool<Postgres>, user_id: i32, message: &[u8]) {
    match sqlx::query("INSERT INTO rusty_app_message (message, user_id) VALUES ($1, $2)")
        .bind(&Message::deserialize(message).unwrap().into_db())
        .bind(user_id)
        .execute(pool)
        .await
    {
        Ok(_) => log::info!("message inserted"),
        Err(e) => log::error!("{e}"),
    };
}
