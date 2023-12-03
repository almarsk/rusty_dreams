use message::{ChatError, MessageType};
use tokio::io::{AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;

use super::save_file::receive_and_save;

pub async fn read(mut reader: ReadHalf<TcpStream>, nick: String) -> Result<(), ChatError> {
    let mut buffer = vec![0; 1024];

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => Ok(()),
            Ok(n) => {
                let incoming_message = message::Message::deserialize(&buffer[..n])
                    .map_err(|_| ChatError::DeserializingIssue)?;
                match incoming_message.content {
                    MessageType::Text(text) => log::info!("{}: {}", incoming_message.nick, text),
                    MessageType::File(name, data) => {
                        log::info!("incoming file from {}", incoming_message.nick);
                        receive_and_save(MessageType::File(name, data), &nick)?;
                    }
                    MessageType::Image(data) => {
                        log::info!("incoming image from {}", incoming_message.nick);
                        receive_and_save(MessageType::Image(data), incoming_message.nick.as_str())?;
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
