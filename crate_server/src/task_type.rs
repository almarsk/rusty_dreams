use std::net::SocketAddr;
use tokio::{io::ReadHalf, io::WriteHalf, net::TcpStream};

pub enum Task {
    ConnWrite(SocketAddr, WriteHalf<TcpStream>),
    // the i32 is user id
    ConnRead(SocketAddr, ReadHalf<TcpStream>, i32),
    Message(SocketAddr, Vec<u8>),
}
