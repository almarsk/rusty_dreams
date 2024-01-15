use flume::{Receiver, Sender};

use anyhow::Result;

use tokio::{io::AsyncReadExt, net::TcpStream};

use message::{get_buffer, send_message, ChatError, Task};

pub async fn web_task(
    mut socket: TcpStream,
    tx: Sender<Task>,
    rx: Receiver<Task>,
) -> Result<(), message::ChatError> {
    loop {
        let mut buffer = get_buffer(&mut socket).await?;
        match socket.read(&mut buffer).await {
            Ok(0) => Ok(()),
            Ok(_) => {
                let task = bincode::deserialize::<Task>(&buffer)
                    .map_err(|_| ChatError::DeserializingIssue)?;

                tx.send_async(task)
                    .await
                    .map_err(|_| ChatError::DatabaseIssue)?;

                if let Ok(h) = rx.clone().recv_async().await {
                    if send_message(&mut socket, h).await.is_err() {
                        log::error!("couldnt send task to db server")
                    };
                } else {
                    log::error!("couldnt receive task from db server")
                };

                //____
                // listen from socket and check whether it is user or message
                // if its message send it via txto database
                // (handling non text message will be done later, via paths and static files)
                // if its user, send back via tx to db task and check whether its a valid login
                //____

                Ok(())
            }
            _ => Err(ChatError::OtherEndIssue),
        }?;
    }
}
