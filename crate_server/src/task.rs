use std::net::SocketAddr;
use tokio::{io::ReadHalf, io::WriteHalf};

pub enum Task {
    ConnWrite(SocketAddr, WriteHalf<Vec<u8>>),
    ConnRead(SocketAddr, ReadHalf<Vec<u8>>),
    Message(SocketAddr, Vec<u8>),
}
