use std::net::SocketAddr;
use tokio::{io::ReadHalf, io::WriteHalf, net::TcpStream};

#[derive(Debug)]
pub enum Task {
    ConnWrite(SocketAddr, WriteHalf<TcpStream>),
    ConnRead(SocketAddr, ReadHalf<TcpStream>, String),
    Message(SocketAddr, Vec<u8>, String),
}
