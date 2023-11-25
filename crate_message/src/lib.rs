use bincode::Error as BincodeError;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use std::io::{Read, Write};
use std::net::TcpStream;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
}

impl MessageType {
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    pub fn deserialize(from: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(from)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub nick: String,
    pub content: MessageType,
    timestamp: String,
}

impl Message {
    pub fn new(nick: String, content: MessageType) -> Self {
        Message {
            nick,
            content,
            timestamp: chat_time_now(),
        }
    }
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    pub fn deserialize(from: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(from)
    }
    pub fn destructure(self) -> (String, MessageType, String) {
        (self.nick, self.content, self.timestamp)
    }
}

#[derive(Error, Debug)]
pub enum Issue {
    #[error("Error serializing message")]
    SerializationError,
    #[error("Error deserializing message")]
    DeserializationError,
    #[error("Error reading from server")]
    ReadingError,
    #[error("Error sending to server")]
    SendingError,
}

pub fn handle_client(connection: &mut TcpStream) -> Result<Message, Issue> {
    let mut len_bytes = [0u8; 4];

    match connection.read_exact(&mut len_bytes) {
        Ok(()) => {
            let len = u32::from_be_bytes(len_bytes) as usize;
            let mut buffer = vec![0u8; len];
            connection
                .read_exact(&mut buffer)
                .map_err(|_| Issue::ReadingError)?;
            let msg = Message::deserialize(&buffer).map_err(|_| Issue::DeserializationError)?;
            Ok(msg)
        }
        Err(_) => Err(Issue::ReadingError), // TODO thiserror
    }
}

pub fn send_message(connection: &mut TcpStream, message: &Message) -> Result<(), Issue> {
    let serialized = message.serialize().map_err(|_| Issue::SerializationError)?;
    let len = serialized.len() as u32;
    // was getting a harmless but annoying lint with the write method
    connection
        .write_all(&len.to_be_bytes())
        .map_err(|_| Issue::SendingError)?;
    // these can unwrap because the question mark operator on the previous line
    // makes or breaks all of these operations
    connection.write_all(&serialized).unwrap();
    connection.flush().unwrap();
    Ok(())
}

// for naming the sent images
pub fn full_time_now() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

// for the handcrafted message logging
pub fn chat_time_now() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%H:%M:%S").to_string()
}
