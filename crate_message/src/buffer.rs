use tokio::net::TcpStream;

use tokio::io::{AsyncReadExt, ReadHalf};

use crate::ChatError;

pub async fn get_buffer(reader: &mut ReadHalf<TcpStream>) -> Result<Vec<u8>, ChatError> {
    let mut len_bytes = [0u8; 4];
    loop {
        match reader.read_exact(&mut len_bytes).await {
            Ok(0) => (),
            Ok(_) => {
                let len = u32::from_be_bytes(len_bytes) as usize;
                return Ok(vec![0u8; len]);
            }
            Err(_) => {
                log::info!("oh shoot");
                return Err(ChatError::ReadingIssue);
            }
        }
    }
}
