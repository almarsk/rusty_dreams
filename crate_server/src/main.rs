use anyhow::Result;
use clap::Parser;
use env_logger::Builder;
use flume::{bounded, Receiver, Sender};
use tokio::{net::TcpListener, try_join};

mod accepting_task;
use accepting_task::accepting_task;
mod broadcasting_task;
use broadcasting_task::accomodate_and_broadcast;
mod listening_task;
use listening_task::listen;
pub mod task_type;
use task_type::{DatabaseTask, Task};

mod database_operations;
use database_operations::database_operations;

use std::{io::Write, net::SocketAddr};

type Senders = (Sender<(Task, i32)>, Receiver<(Task, i32)>);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String) {
        (self.host, self.port)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (host, port) = Args::parse().deconstruct();

    // env_logger as backend for log here
    Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.args()
            )
        })
        .init();

    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        log::error!("cant use {}:{}, going default", host, port);
        "127.0.0.1:11111".parse()?
    };

    // server
    let listener = TcpListener::bind(address).await?;

    // im not actually sure if all this is necessary, avoiding mpmc
    let (tx_accomodate, rx_accomodate): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_listen, rx_listen): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_send, rx_send): Senders = bounded(10);
    let (tx_user, rx_user): (Sender<DatabaseTask>, Receiver<DatabaseTask>) = bounded(10);
    let (tx_user_confirm, rx_user_confirm): (Sender<DatabaseTask>, Receiver<DatabaseTask>) =
        bounded(10);
    log::info!("starting a new server");

    let database_task = tokio::task::spawn(database_operations(rx_user, tx_user_confirm));
    let broadcasting_task = tokio::task::spawn(accomodate_and_broadcast(
        rx_accomodate,
        rx_send,
        tx_user.clone(),
    ));
    let accepting_task = tokio::task::spawn(accepting_task(
        listener,
        tx_accomodate,
        tx_listen,
        tx_user,
        rx_user_confirm,
    ));
    let listening_task = tokio::task::spawn(listen(rx_listen, tx_send));

    // not too happy with this
    match try_join!(
        accepting_task,
        broadcasting_task,
        listening_task,
        database_task
    ) {
        Ok(i) => {
            if let Err(e) = i.0 {
                log::error!("It failed: {}", e)
            };
        }
        Err(e) => log::error!("It failed: {}", e),
    }

    Ok(())
}
