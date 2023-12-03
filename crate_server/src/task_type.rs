use std::net::SocketAddr;
use tokio::{io::ReadHalf, io::WriteHalf, net::TcpStream};

pub enum Task {
    ConnWrite(SocketAddr, WriteHalf<TcpStream>),
    ConnRead(SocketAddr, ReadHalf<TcpStream>),
    Message(SocketAddr, Vec<u8>),
}
