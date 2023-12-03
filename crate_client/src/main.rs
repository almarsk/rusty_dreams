use clap::Parser;
use env_logger::Builder;
use tokio::net::TcpStream;

use std::{io::Write, net::SocketAddr};

mod read_task;
mod write_task;
use read_task::read;
use write_task::write;
mod save_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = names::Generator::default().next().unwrap())]
    nick: String,
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String, String) {
        (self.host, self.port, self.nick)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port, nick) = Args::parse().deconstruct();

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

    let stream = TcpStream::connect(address).await?;
    let (reader, writer) = tokio::io::split(stream);

    let write_task = tokio::spawn(write(writer, nick.clone()));
    let read_task = tokio::spawn(read(reader, nick));

    let _ = tokio::try_join!(write_task, read_task)?;
    Ok(())
}
