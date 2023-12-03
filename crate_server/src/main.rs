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
use task_type::Task;

use std::{io::Write, net::SocketAddr};

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port) = Args::parse().deconstruct();

    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        log::error!("cant use {}:{}, going default", host, port);
        "127.0.0.1:11111".parse()?
    };

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

    let listener = TcpListener::bind(address).await?;
    let (tx_accomodate, rx_accomodate): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_listen, rx_listen): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_send, rx_send): (Sender<Task>, Receiver<Task>) = bounded(10);
    log::info!("starting a new server");

    let accepting_task = tokio::task::spawn(accepting_task(listener, tx_accomodate, tx_listen));
    let broadcasting_task = tokio::task::spawn(accomodate_and_broadcast(rx_accomodate, rx_send));
    let listening_task = tokio::task::spawn(listen(rx_listen, tx_send));

    match try_join!(accepting_task, broadcasting_task, listening_task) {
        Ok(i) => {
            if let Err(e) = i.0 {
                log::error!("It failed: {}", e)
            };
        }
        Err(e) => log::error!("It failed: {}", e),
    }

    Ok(())
}
