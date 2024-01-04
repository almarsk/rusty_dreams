use std::sync::Arc;
use tokio::sync::Mutex;

use crate::check_db;

pub async fn check_login_get_answer(pool: Arc<Mutex<sqlx::PgPool>>, pass: String, nick: String) {
    log::info!("checking login");
    let q = check_db::login_db(nick.as_str(), pass.as_str(), pool).await;
    log::info!("{:?}", q)
}
