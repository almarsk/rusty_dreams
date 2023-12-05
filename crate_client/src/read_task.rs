use message::{ChatError, MessageType};
use tokio::io::{AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;

use super::save_file::receive_and_save;

pub async fn read(mut reader: ReadHalf<TcpStream>, nick: String) -> Result<(), ChatError> {
    loop {
        let mut len_bytes = [0u8; 4];
        let buffer_result = match reader.read_exact(&mut len_bytes).await {
            Ok(_) => {
                let len = u32::from_be_bytes(len_bytes) as usize;
                log::info!("creating buffer to read {} bytes", len);
                Ok(vec![0u8; len])
            }
            Err(_) => Err(ChatError::ReadingIssue),
        };

        if let Ok(mut buffer) = buffer_result {
            match reader.read(&mut buffer).await {
                Ok(0) => Ok(()),
                Ok(n) => {
                    let incoming_message = match message::Message::deserialize(&buffer[..n]) {
                        Ok(m) => m,
                        Err(_) => {
                            log::error!("error deserializing");
                            continue;
                        }
                    };
                    match incoming_message.content {
                        MessageType::Text(text) => {
                            log::info!("{}: {}", incoming_message.nick, text)
                        }
                        MessageType::File(name, data) => {
                            log::info!("incoming file from {}", incoming_message.nick);
                            receive_and_save(MessageType::File(name, data), &nick)?;
                        }
                        MessageType::Image(data) => {
                            log::info!("incoming image from {}", incoming_message.nick);
                            receive_and_save(MessageType::Image(data), &nick)?;
                        }
                    };

                    Ok(())
                }
                Err(e) => {
                    log::error!("It failed: {}", e);
                    Err(ChatError::OtherEndIssue)
                }
            }?;
        }
    }
}
