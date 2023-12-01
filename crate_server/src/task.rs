use std::net::SocketAddr;
use tokio::net::tcp::WriteHalf;

pub enum Task<'a> {
    Connection(SocketAddr, WriteHalf<'a>),
    Message(SocketAddr, Vec<u8>),
}
