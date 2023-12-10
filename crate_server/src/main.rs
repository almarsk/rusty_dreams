use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
use flume::{bounded, Receiver, Sender};
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::Mutex, try_join};

mod accepting_task;
use accepting_task::accepting_task;
mod broadcasting_task;
use broadcasting_task::accomodate_and_broadcast;
mod listening_task;
use listening_task::listen;
pub mod task_type;
use task_type::Task;
mod check_db;

use std::{io::Write, net::SocketAddr, sync::Arc};

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

    // database setup
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    log::info!("{}", database_url);
    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .map_err(|e| anyhow::Error::new(e).context("Error connecting to database"))?,
    );

    sqlx::query(
        r#"
   CREATE TABLE IF NOT EXISTS rusty_app_user (
     id SERIAL PRIMARY KEY,
     nick text,
     pass text
   );"#,
    )
    .execute(&*pool)
    .await?;

    sqlx::query(
        r#"
   CREATE TABLE IF NOT EXISTS rusty_app_message (
     id SERIAL PRIMARY KEY,
     message TEXT,
     user_id SERIAL REFERENCES rusty_app_user(id)
   );"#,
    )
    .execute(&*pool)
    .await?;

    // server
    let listener = TcpListener::bind(address).await?;
    let (tx_accomodate, rx_accomodate): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_listen, rx_listen): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_send, rx_send): Senders = bounded(10);
    log::info!("starting a new server");

    let lock = Arc::new(Mutex::new(()));

    let accepting_task = tokio::task::spawn(accepting_task(
        listener,
        tx_accomodate,
        tx_listen,
        Arc::clone(&pool),
        Arc::clone(&lock),
    ));
    let broadcasting_task = tokio::task::spawn(accomodate_and_broadcast(
        rx_accomodate,
        rx_send,
        Arc::clone(&pool),
        Arc::clone(&lock),
    ));

    let listening_task = tokio::task::spawn(listen(rx_listen, tx_send));

    // not too happy with this
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
