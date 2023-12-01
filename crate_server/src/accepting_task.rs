use message::Message;
use tokio::net::TcpListener;

pub async fn accepting_task(listener: TcpListener) -> Result<()> {
    let (mut socket, address) = listener.accept().await?;
    let (tx_clone, rx_clone) = (tx.clone(), rx.clone());
    // saying hi
    println!("there is a new guy from: {}", address);
    if let Ok(m) = Message::new("server: hi, new guy").serialize() {
        socket.write_all(&m).await?;
    }
    let (mut reader, mut writer) = tokio::io::split(socket);
}
