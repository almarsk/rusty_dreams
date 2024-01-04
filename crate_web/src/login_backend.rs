use message::{send_message, Addressee, MaybeSerializedMessage::*};
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::TcpStream,
};

use crate::LoginForm;

pub async fn is_valid(
    _reader: &mut ReadHalf<&mut TcpStream>,
    writer: &mut WriteHalf<&mut TcpStream>,
    login: LoginForm,
) -> bool {
    if send_message(
        writer,
        ToSerializeLogin(
            message::MessageType::Pass(login.password),
            login.username.as_str(),
        ),
        Addressee::Server,
    )
    .await
    .is_err()
    {
        log::error!("failed sending the message");
    };
    false
}
