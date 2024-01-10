use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use env_logger::Builder;
//use flume::{Receiver, Sender};
//use flume::bounded;
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, sync::Mutex, try_join};

use std::{io::Write, net::SocketAddr, sync::Arc};

use message::ChatError;

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

    {
        let lock = &*pool.lock().await;

        sqlx::query(
            r#"
   CREATE TABLE IF NOT EXISTS rusty_app_user (
     id SERIAL PRIMARY KEY,
     nick text,
     pass text
   );"#,
        )
        .execute(lock)
        .await?;

        sqlx::query(
            r#"
   CREATE TABLE IF NOT EXISTS rusty_app_message (
     id SERIAL PRIMARY KEY,
     message TEXT,
     nick TEXT
   );"#,
        )
        .execute(lock)
        .await?;
    }

    // server
    let listener = TcpListener::bind(address).await?;
    let (_socket, _address) = listener
        .accept()
        .await
        .map_err(|_| ChatError::AcceptanceIssue)?;

    log::info!("starting a new server");

    let web_task = tokio::task::spawn(async { log::info!("web_task") });

    // not too happy with this
    match try_join!(web_task) {
        Ok(_) => {}
        Err(e) => log::error!("It failed: {}", e),
    }

    Ok(())
}
