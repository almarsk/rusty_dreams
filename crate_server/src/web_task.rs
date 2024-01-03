//use std::sync::Arc;

//use flume::Sender;
use tokio::io::AsyncReadExt;
//, sync::Mutex};
use tokio::net::TcpListener;

//use crate::check_db::{history, login_db};

//use super::task_type::Task;

use message::{get_buffer, ChatError};
//    get_buffer, send_message, Addressee::*, ChatError, MaybeSerializedMessage::*, Message,
//    MessageType,
//};

pub async fn web_task<'a>(listener: TcpListener) -> Result<(), ChatError> {
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
            }
            _ => {
                log::error!("issue accepting message from web server");
                continue;
            }
        };

        // match buffer on type of operation
    }
}
