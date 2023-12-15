use std::{io::Read, path::Path};

use crate::{ChatError, Message, MessageType};

/// Builds Message types which require a Path check and reading a file
///
/// This function takes user input and nick and goal MessageType. If it is the Image or File MessageType it
/// checks for valid path and reads the desired file into the message
///
/// # Examples
///
/// ```rust
///# use message::MessageType;
///# use message::build_message::build_message_w_path;
/// let nick = "nick".to_string();
/// let input = "hi";
/// let mess_type = MessageType::Image(vec![]);
/// let message = build_message_w_path(nick,input, mess_type);
/// ```
pub fn build_message_w_path(
    nick: String,
    input: &str,
    mess_type: MessageType,
) -> Result<Message, ChatError> {
    if match mess_type {
        MessageType::File(_, _) => Some(()),
        MessageType::Image(_) => Some(()),
        _ => None,
    }
    .is_none()
    {
        return Err(ChatError::MessageTypePathIssue);
    }

    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    if parts.len() <= 1 {
        Err(ChatError::NoPathIssue)
    } else {
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
    }
}
