use std::net::SocketAddr;

use anyhow::Result;
use flume::Sender;

use message::ChatError;
use tokio::{
    io::{AsyncReadExt, ReadHalf},
    net::TcpStream,
};

use crate::task_type::Task;
use message::get_buffer;

pub async fn read_from_socket(
    socket: &mut ReadHalf<TcpStream>,
    tx: Sender<(Task, i32)>,
    address: SocketAddr,
    client_id: i32,
) -> Result<(), ChatError> {
    log::info!("starting a new listener on {}", address);
    loop {
        let mut buffer = get_buffer(socket).await?;
        match socket.read(&mut buffer).await {
            Ok(0) => Ok(()),
            Ok(n) => {
                log::info!("new message from {}", address);
                tx.send((Task::Message(address, buffer[..n].to_vec()), client_id))
                    .map_err(|_| ChatError::PassToSendIssue)
            }
            _ => Err(ChatError::OtherEndIssue),
        }?;
    }
}
