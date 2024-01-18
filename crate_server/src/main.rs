use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use flume::bounded;
use flume::{Receiver, Sender};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::Mutex, try_join};

use std::{io::Write, net::SocketAddr, sync::Arc};

use message::{ChatError, Task};

mod db;
use db::database_task;
mod web;
use web::web_task;
mod auth;

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

    // database setup
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    log::info!("{}", database_url);
    let pool = Arc::new(Mutex::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| anyhow::Error::new(e).context("Error connecting to database"))?,
    ));

    //let lock = &*pool.lock().await;

    // server
    let listener = TcpListener::bind(address).await?;
    let (socket, _) = listener
        .accept()
        .await
        .map_err(|_| ChatError::AcceptanceIssue)?;

    log::info!("starting a new server");

    let (tx_db, rx_db): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_db_return, rx_db_return): (Sender<Task>, Receiver<Task>) = bounded(10);

    let web_task = tokio::task::spawn(web_task(socket, tx_db, rx_db_return));
    let db_task = tokio::task::spawn(database_task(rx_db, tx_db_return, pool));

    // not too happy with this
    if try_join!(web_task, db_task).is_err() {
        log::error!("server problem")
    };

    Ok(())
}
