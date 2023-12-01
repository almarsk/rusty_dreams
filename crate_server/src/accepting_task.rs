use flume::Sender;
use message::Message;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

use super::task::Task;
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
        println!("there is a new guy from: {}", address);
        if let Ok(m) = Message::new("server: hi, new guy").serialize() {
            socket
                .write_all(&m)
                .await
                .map_err(|_| ChatError::AccomodationIssue)?;
        }
        let _tx_clone_b = tx_broadcast.clone();
        let _tx_clone_l = tx_listen.clone();
        let (mut _reader, mut _writer) = tokio::io::split(socket);
        // todo send reader to tx_clone_l
        // todo send writer to tx_clone_b
    }
}
