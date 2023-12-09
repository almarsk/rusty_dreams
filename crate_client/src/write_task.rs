use tokio::net::TcpStream;

use message::{clear_previous_line, send_message, Addressee, ChatError, MaybeSerializedMessage::*};
use tokio::io::WriteHalf;

pub async fn write(mut writer: WriteHalf<TcpStream>, nick: String) -> Result<(), ChatError> {
    let mut input = String::new();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim_end_matches('\n');

        // log user message
        clear_previous_line();
        log::info!("{}: {}", nick, input);

        send_message(
            &mut writer,
            ToSerialize(input, nick.as_str()),
            Addressee::Server,
        )
        .await?;
    }
}
