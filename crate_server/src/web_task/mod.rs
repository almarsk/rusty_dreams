use std::sync::Arc;

use message::{get_buffer, ChatError, Message};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

mod check_login_return_answer;
use check_login_return_answer::check_login_get_answer;
use tokio::sync::Mutex;

pub async fn web_task<'a>(
    listener: TcpListener,
    pool: Arc<Mutex<sqlx::PgPool>>,
) -> Result<(), ChatError> {
    log::info!("listening for web server");

    let (socket, address) = listener
        .accept()
        .await
        .map_err(|_| ChatError::AcceptanceIssue)?;

    log::info!("connected web server on {}", address);

    let (mut reader, mut _writer) = tokio::io::split(socket);

    loop {
        // reading login from client
        let mut buffer = get_buffer(&mut reader).await?;
        match reader.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                log::info!("processing message from web server on {}", address);
                log::info!("{:?}", Message::deserialize(&buffer));
                if let Ok(login) = Message::deserialize(&buffer) {
                    match login.content {
                        message::MessageType::Pass(p) => {
                            log::info!("{} is trying to log in with {}", login.nick, p);
                            check_login_get_answer(Arc::clone(&pool), p, login.nick).await
                        }
                        _ => {
                            log::info!("huh")
                            // and probably send to channels, which take care of this
                        }
                    }

                    //let _ = check_db::login_db(&login.nick, login.content.into(), pool);
                }
            }
            _ => {
                log::error!("issue accepting message from web server");
                continue;
            }
        };

        // match buffer on type of operation
    }
}
