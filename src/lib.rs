use bincode::Error as BincodeError;
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

impl MessageType {
    pub fn serialize(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    pub fn deserialize(from: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(from)
    }
}

pub fn handle_client(connection: &mut TcpStream) -> Result<MessageType, Box<dyn Error>> {
    let mut len_bytes = [0u8; 4];

    connection
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    match connection.read_exact(&mut len_bytes) {
        Ok(()) => {
            let len = u32::from_be_bytes(len_bytes) as usize;
            let mut buffer = vec![0u8; len];
            connection.read_exact(&mut buffer)?;
            let msg = MessageType::deserialize(&buffer)?;
            println!("{:?}", msg);
            Ok(msg)
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn send_message(connection: &mut TcpStream, message: &MessageType) {
    let serialized = message.serialize().unwrap();

    let len = serialized.len() as u32;
    connection.write_all(&len.to_be_bytes()).unwrap();
    connection.write_all(&serialized).unwrap();
}
