use std::{fmt, net::SocketAddr};

use tokio::{
    io::{AsyncWriteExt, WriteHalf},
    net::TcpStream,
};

use crate::{ChatError, Message};

use MaybeSerializedMessage::*;

#[allow(clippy::needless_lifetimes)]
pub async fn send_message<'a>(
    writer: &mut WriteHalf<TcpStream>,
    input: MaybeSerializedMessage<'a>,
    addressee: Addressee<'a>,
) -> Result<(), ChatError> {
    let input = match input {
        Serialized(i) => i,
        ToSerialize(t, nick) => Message::new(t, nick.to_string())?
            .serialize()
            .map_err(|_| {
                log::error!("issue serializing");
                ChatError::SerializingIssue
            })?,
    };

    let len = input.len() as u32;
    if writer.write_all(&len.to_be_bytes()).await.is_err() {
        log::error!("sending to {} failed", addressee);
    } else {
        writer.write_all(&input).await.map_err(|_| {
            log::error!("issue writing");
            ChatError::WritingIssue
        })?
    }

    Ok(())
}

pub enum MaybeSerializedMessage<'a> {
    Serialized(Vec<u8>),
    ToSerialize(&'a str, &'a str),
}

pub enum Addressee<'a> {
    Server,
    Client(&'a SocketAddr),
}

impl fmt::Display for Addressee<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Addressee::Client(a) => write!(f, "{}", a),
            Addressee::Server => write!(f, "server"),
        }
    }
}
