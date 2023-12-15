use bincode::Error as BincodeError;
use serde::{Deserialize, Serialize};

use crate::ChatError;

use super::build_message::build_message_w_path;

/// This enum hold the message and it's metadata.
///
/// The Welcome variant is for when the server either accepts or refuses the login
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
    Welcome(Result<(), ChatError>),
}

impl MessageType {
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    pub fn deserialize(from: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(from)
    }
    pub fn into_db(&self) -> Option<String> {
        match self {
            MessageType::File(name, _) => Some(format!("incoming file called {}", name)),
            MessageType::Text(text) => Some(text.clone()),
            MessageType::Image(_) => Some("incoming image".to_string()),
            MessageType::Welcome(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: MessageType,
    pub nick: String,
}

impl Message {
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    pub fn deserialize(from: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(from)
    }
    pub fn new(input: &str, nick: String) -> Result<Message, ChatError> {
        if input.starts_with(".quit") {
            std::process::exit(0)
        } else if input.starts_with(".file ") {
            build_message_w_path(nick, input, MessageType::File("".to_string(), vec![]))
        } else if input.starts_with(".image ") {
            build_message_w_path(nick, input, MessageType::Image(vec![]))
        } else if input.starts_with(".accept") {
            log::info!("they lettin us in");
            Ok(Message {
                content: MessageType::Welcome(Ok(())),
                nick: "system".to_string(),
            })
        } else if input.starts_with(".refuse") {
            // space here to add reasong of failure
            Ok(Message {
                content: MessageType::Welcome(Err(ChatError::LoginIssue)),
                nick: "system".to_string(),
            })
        } else {
            Ok(Message {
                content: MessageType::Text(input.to_string()),
                nick,
            })
        }
    }

    pub fn into_db(&self) -> Option<String> {
        self.content.into_db()
    }
}
