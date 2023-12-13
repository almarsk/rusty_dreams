use flume::{Receiver, Sender};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::task_type::DatabaseTask;

use super::task_type::Task;

use message::{
    get_buffer, send_message, Addressee::*, ChatError, MaybeSerializedMessage::*, Message,
    MessageType,
};

pub async fn accepting_task<'a>(
    listener: TcpListener,
    tx_broadcast: Sender<Task>,
    tx_listen: Sender<Task>,
    tx_user: Sender<DatabaseTask>,
    rx_user_confirm: Receiver<DatabaseTask>,
) -> Result<(), ChatError> {
    loop {
        log::info!("listening for knocks");

        let (socket, address) = listener
            .accept()
            .await
            .map_err(|_| ChatError::AcceptanceIssue)?;

        log::info!("someone knocking");

        let tx_clone_b = tx_broadcast.clone();
        let tx_clone_l = tx_listen.clone();
        let (mut reader, mut writer) = tokio::io::split(socket);

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

        tx_user
            .send_async(DatabaseTask::LoginRequest((nick.clone(), pass.clone())))
            .await
            .map_err(|_| ChatError::DatabaseIssue)?;

        log::info!("waiting for response from database task on login validity");
        let (client_id, back) = match rx_user_confirm.recv_async().await {
            Err(_) => {
                log::error!("internal issue {}", address);
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
            Ok(db_task) => match db_task {
                DatabaseTask::LoginConfirmation(r) => match r {
                    Ok((client_id, back)) => (client_id, back),
                    Err(e) => {
                        log::error!("invalid login from {} {}", address, e);
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
                },

                e => {
                    log::error!("something fishy coming instead login confirmation: {:?}", e);
                    continue;
                }
            },
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

        let back = match back {
            true => " back",
            false => "",
        };

        // welcoming new client
        if let Err(e) = send_message(
            &mut writer,
            ToSerialize(format!("welcome{}, {}", back, nick).as_str(), "system"), // todo add nick
            Client(&address),
        )
        .await
        {
            log::error!("Greeting failed: {}", e)
        };

        tx_clone_b
            .send_async(Task::ConnWrite(address, writer))
            .await
            .map_err(|_| ChatError::AccomodationIssue)?;
        tx_clone_l
            .send_async(Task::ConnRead(address, reader, client_id))
            .await
            .map_err(|_| ChatError::AccomodationIssue)?;
    }
}
