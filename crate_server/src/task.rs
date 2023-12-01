use std::net::SocketAddr;
use tokio::{io::ReadHalf, io::WriteHalf};

pub enum Task {
    Conn_Write(SocketAddr, WriteHalf<Vec<u8>>),
    Conn_Read(SocketAddr, ReadHalf<Vec<u8>>),
    Message(SocketAddr, Vec<u8>),
}
