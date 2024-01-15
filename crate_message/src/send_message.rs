use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{task::Task, ChatError};

#[allow(clippy::needless_lifetimes)]
pub async fn send_message(writer: &mut TcpStream, input: Task) -> Result<(), ChatError>
where
{
    let input = bincode::serialize(&input).map_err(|_| ChatError::SerializingIssue)?;

    let len = input.len() as u32;
    if writer.write_all(&len.to_be_bytes()).await.is_err() {
        log::error!("sending to webserver failed");
    } else {
        writer.write_all(&input).await.map_err(|_| {
            log::error!("issue writing");
            ChatError::WritingIssue
        })?
    }

    Ok(())
}
