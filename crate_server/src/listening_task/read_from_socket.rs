use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use flume::Sender;

use message::ChatError;
use tokio::{
    io::{AsyncReadExt, ReadHalf},
    net::TcpStream,
};

use crate::task::Task;

pub async fn read_from_socket(
    socket: &mut ReadHalf<TcpStream>,
    tx: Arc<Sender<Task>>,
    address: SocketAddr,
) -> Result<(), ChatError> {
    let mut buffer = vec![0; 1024];

    match socket.read(&mut buffer).await {
        Ok(0) => (),
        Ok(n) => tx
            .send(Task::Message(address, buffer[..n].to_vec()))
            .map_err(|_| ChatError::PassToSendIssue)?,

        _ => (),
    };
    Ok(())
}
