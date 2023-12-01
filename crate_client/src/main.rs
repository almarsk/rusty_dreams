use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use message::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("localhost:6666").await?;

    let (mut reader, mut writer) = tokio::io::split(stream);

    let write_task = tokio::spawn(async move {
        let mut input = String::new();

        loop {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("input: {}", input);

            if let Ok(ser_inp) = Message::new(input.as_str()).serialize() {
                println!("sendind {:?}", Message::deserialize(&ser_inp).unwrap());
                writer
                    .write_all(&ser_inp)
                    .await
                    .expect("failed to send bytes");
            }
        }
    });

    let read_task = tokio::spawn(async move {
        let mut buffer = vec![0; 1024];

        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => println!("nada"),
                Ok(n) => {
                    println!(
                        "received message: {}",
                        String::from_utf8((&buffer[..n]).into()).unwrap()
                    )
                }
                Err(e) => {
                    eprintln!("It failed: {}", e);
                    return;
                }
            };
        }
    });

    let _ = tokio::try_join!(write_task, read_task)?;
    Ok(())
}
