use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use message::{clear_previous_line, ChatError, Message};
use tokio::io::WriteHalf;

pub async fn write(mut writer: WriteHalf<TcpStream>, nick: String) -> Result<(), ChatError> {
    let mut input = String::new();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim_end_matches('\n');

        clear_previous_line();
        log::info!("{}: {}", nick, input);

        if let Ok(ser_inp) = Message::new(input, nick.clone())?.serialize() {
            let len = ser_inp.len() as u32;
            if writer.write_all(&len.to_be_bytes()).await.is_err() {
                log::error!("sending to server failed");
            } else {
                writer
                    .write_all(&ser_inp)
                    .await
                    .map_err(|_| ChatError::WritingIssue)?
                //writer.flush().await.unwrap();
            };

            // writer.write_exact(ser_inp.len())
        }
    }
}
