use std::{collections::HashMap, net::SocketAddr};

use flume::{bounded, Receiver, Sender};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::WriteHalf, TcpListener},
};

mod accepting_task;
mod broadcasting_task;
use broadcasting_task::accomodate_and_broadcast;
pub mod task;
use task::Task;

use message::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6666").await?;
    let (tx, rx): (Sender<Task>, Receiver<Task>) = bounded(10);
    println!("starting a new thing");

    let rx_broadcast = rx.clone();

    let accepting_task = tokio::task::spawn(async move {
        let (mut socket, address) = listener.accept().await?;
        let (tx_clone, rx_clone) = (tx.clone(), rx.clone());
        // saying hi
        println!("there is a new guy from: {}", address);
        if let Ok(m) = Message::new("server: hi, new guy").serialize() {
            socket.write_all(&m).await?;
        }
        let (mut reader, mut writer) = tokio::io::split(socket);
    });

    let broadcasting_task = tokio::task::spawn(accomodate_and_broadcast(rx_broadcast));
    Ok(())

    /*
    tokio::spawn(async move {
        loop {
            let mut buffer = vec![0; 1024];

            if let Ok(n) = reader.read(&mut buffer).await {
                broadcast(address, buffer, n, tx_clone.clone()).await
            };
        }
    });
    */
}
