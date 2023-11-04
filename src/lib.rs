use bincode::Error as BincodeError;
use serde::{Deserialize, Serialize};

use std::io::{Read, Write};
use std::net::TcpStream;

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

pub fn handle_client(mut connection: TcpStream) -> MessageType {
    let mut len_bytes = [0u8; 4];

    connection.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    connection.read_exact(&mut buffer).unwrap();

    MessageType::deserialize(&buffer).unwrap()
}

pub fn send_message(connection: &mut TcpStream, message: &MessageType) {
    let serialized = message.serialize().unwrap();

    let len = serialized.len() as u32;
    let _ = connection.write(&len.to_be_bytes()).unwrap();
    connection.write_all(&serialized).unwrap();
}
