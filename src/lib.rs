use bincode::Error as BincodeError;
use chrono::{DateTime, Local};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>), // Filename and its content as bytes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub nick: String,
    content: MessageType,
    timestamp: String,
}

impl MessageType {
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    pub fn deserialize(from: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(from)
    }
}

impl Message {
    pub fn new(nick: String, content: MessageType) -> Self {
        Message {
            nick,
            content,
            timestamp: now(),
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

pub fn handle_client(connection: &mut TcpStream) -> Result<Message, Box<dyn Error>> {
    let mut len_bytes = [0u8; 4];

    match connection.read_exact(&mut len_bytes) {
        Ok(()) => {
            let len = u32::from_be_bytes(len_bytes) as usize;
            let mut buffer = vec![0u8; len];
            connection.read_exact(&mut buffer)?;
            let msg = Message::deserialize(&buffer)?;
            Ok(msg)
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn send_message(connection: &mut TcpStream, message: &Message) -> Result<(), Box<dyn Error>> {
    let serialized = message.serialize().unwrap();
    let len = serialized.len() as u32;
    // was getting a harmless but annoying lint with the write method
    connection.write_all(&len.to_be_bytes())?;
    connection.write_all(&serialized)?;
    if let Err(e) = connection.flush() {
        println!("{e:?}")
    }
    Ok(())
}

pub fn full_now() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn now() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%H:%M:%S").to_string()
}
