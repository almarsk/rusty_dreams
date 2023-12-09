use std::sync::Arc;

use flume::Sender;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::check_db::login_db;

use super::task_type::Task;

use message::{
    get_buffer, send_message, Addressee::*, ChatError, MaybeSerializedMessage::*, Message,
    MessageType,
};

pub async fn accepting_task<'a>(
    listener: TcpListener,
    tx_broadcast: Sender<Task>,
    tx_listen: Sender<Task>,
    pool: Arc<sqlx::PgPool>,
) -> Result<(), ChatError> {
    loop {
        let (socket, address) = listener
            .accept()
            .await
            .map_err(|_| ChatError::AcceptanceIssue)?;

        let tx_clone_b = tx_broadcast.clone();
        let tx_clone_l = tx_listen.clone();
        let (mut reader, mut writer) = tokio::io::split(socket);

        // TODO

        // read pss and nick from reader
        // check validity
        // if valid save and do the rest
        // else print send not valid

        // reading login from client
        let mut buffer = get_buffer(&mut reader).await?;
        match reader.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                log::info!("checking validity of login from {}", address);
            }
            _ => {
                log::error!("issue accepting login");
                continue;
            }
        };

        // parsing login from client
        let (nick, pass) = if let Ok(login) = Message::deserialize(&buffer) {
            (login.nick, login.content)
        } else {
            log::error!("issue deserializing login");
            if send_message(
                &mut writer,
                ToSerialize(".refuse", "system"), // todo add nick
                Client(&address),
            )
            .await
            .is_err()
            {
                log::error!("issue sending refusal to client")
            };
            continue;
        };

        let pass = if let MessageType::Text(i) = pass {
            i
        } else {
            log::error!("issue in password transmission");
            "".to_string()
        };

        let client_id = match login_db(&nick, &pass, Arc::clone(&pool)).await {
            Err(_) => {
                log::error!("invalid login from {}", address);
                if let Err(e) = send_message(
                    &mut writer,
                    ToSerialize(".refuse", "system"),
                    Client(&address),
                )
                .await
                {
                    log::error!("issue sending login info to client {}", e)
                };
                continue;
            }
            Ok(id) => id,
        };

        if let Err(e) = send_message(
            &mut writer,
            ToSerialize(".accept", "system"),
            Client(&address),
        )
        .await
        {
            log::error!("issue sending login info to client {}", e)
        };

        // welcoming new client
        if let Err(e) = send_message(
            &mut writer,
            ToSerialize(format!("welcome, {}", nick).as_str(), "system"), // todo add nick
            Client(&address),
        )
        .await
        {
            log::error!("Greeting failed: {}", e)
        };

        tx_clone_b
            .send(Task::ConnWrite(address, writer))
            .map_err(|_| ChatError::AccomodationIssue)?;
        tx_clone_l
            .send(Task::ConnRead(address, reader, client_id))
            .map_err(|_| ChatError::AccomodationIssue)?;
    }
}
