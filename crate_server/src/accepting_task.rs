use flume::Sender;
use message::Message;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

use super::task_type::Task;
use message::ChatError;

pub async fn accepting_task<'a>(
    listener: TcpListener,
    tx_broadcast: Sender<Task>,
    tx_listen: Sender<Task>,
) -> Result<(), ChatError> {
    loop {
        let (mut socket, address) = listener
            .accept()
            .await
            .map_err(|_| ChatError::AcceptanceIssue)?;
        // saying hi
        log::info!("there is a new guy from: {}", address);

        if let Ok(m) = Message::new("hi, new guy", "system".to_string())?.serialize() {
            if socket.write_all(&m.len().to_be_bytes()).await.is_err() {
                log::error!("sending to server failed");
            } else {
                socket
                    .write_all(&m)
                    .await
                    .map_err(|_| ChatError::WritingIssue)?
                //writer.flush().await.unwrap();
            };
        }

        let _tx_clone_b = tx_broadcast.clone();
        let _tx_clone_l = tx_listen.clone();
        let (reader, writer) = tokio::io::split(socket);
        _tx_clone_b
            .send(Task::ConnWrite(address, writer))
            .map_err(|_| ChatError::AccomodationIssue)?;
        _tx_clone_l
            .send(Task::ConnRead(address, reader))
            .map_err(|_| ChatError::AccomodationIssue)?;
    }
}
