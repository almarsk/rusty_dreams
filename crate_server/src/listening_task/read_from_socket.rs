use std::net::SocketAddr;

use anyhow::Result;
use flume::Sender;

use message::ChatError;
use tokio::{
    io::{AsyncReadExt, ReadHalf},
    net::TcpStream,
};

use crate::task_type::Task;

pub async fn read_from_socket(
    socket: &mut ReadHalf<TcpStream>,
    tx: Sender<Task>,
    address: SocketAddr,
) -> Result<(), ChatError> {
    log::info!("starting a new listener on {}", address);
    loop {
        // TODO dynamic message length
        let mut buffer = vec![0; 1024];

        match socket.read(&mut buffer).await {
            Ok(0) => Ok(()),
            Ok(n) => {
                log::info!(
                    "new message from {}: {:?}",
                    address,
                    message::Message::deserialize(&buffer[..n])
                        .map_err(|_| ChatError::DeserializingIssue)?
                );
                tx.send(Task::Message(address, buffer[..n].to_vec()))
                    .map_err(|_| ChatError::PassToSendIssue)
            }
            _ => Err(ChatError::OtherEndIssue),
        }?;
    }
}
