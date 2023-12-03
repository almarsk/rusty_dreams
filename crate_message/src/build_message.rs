use std::{io::Read, path::Path};

use crate::{ChatError, Message, MessageType};

pub fn build_message(
    nick: String,
    input: &str,
    mess_type: MessageType,
) -> Result<Message, ChatError> {
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    if parts.len() > 1 {
        let path = Path::new(parts[1].trim_end());
        let mut file = std::fs::File::open(path).map_err(|_| ChatError::UserDirectoryIssue)?;
        let mut file_contents = vec![];
        file.read_to_end(&mut file_contents)
            .map_err(|_| ChatError::ReadingIssue)?;

        let file_name = if let Some(n) = path.file_name() {
            n
        } else {
            return Err(ChatError::PathIssue);
        };

        let content = match mess_type {
            MessageType::File(_, _) => {
                MessageType::File(file_name.to_string_lossy().to_string(), file_contents)
            }
            MessageType::Image(_) => MessageType::Image(file_contents),
            _ => {
                log::warn!("something fishy is goin on");
                MessageType::Text("".to_string())
            }
        };
        Ok(Message { content, nick })
    } else {
        Err(ChatError::NoPathIssue)
    }
}
