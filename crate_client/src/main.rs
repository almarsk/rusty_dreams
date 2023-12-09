use clap::Parser;
use env_logger::Builder;
use message::{get_buffer, send_message, Message, MessageType};
use tokio::io::AsyncReadExt;
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
    #[arg(long)]
    pass: String,
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String, String, String) {
        (self.host, self.port, self.nick, self.pass)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port, nick, pass) = Args::parse().deconstruct();

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
    let (mut reader, mut writer) = tokio::io::split(stream);

    send_message(
        &mut writer,
        message::MaybeSerializedMessage::ToSerialize(&pass, &nick),
        message::Addressee::Server,
    )
    .await?;

    let mut buffer = get_buffer(&mut reader).await?;
    match reader.read(&mut buffer).await? {
        n if n > 0 => {}
        _ => {
            log::error!("issue reading login");
            std::process::exit(1)
        }
    };

    let m_deser = Message::deserialize(&buffer)?;

    match m_deser.content {
        MessageType::Welcome(Ok(())) => {
            let write_task = tokio::spawn(write(writer, nick.clone()));
            let read_task = tokio::spawn(read(reader, nick));

            let _ = tokio::try_join!(write_task, read_task)?;
        }
        MessageType::Welcome(Err(e)) => log::error!("{}", e),
        _ => log::error!("Something fishy"),
    };

    Ok(())
}
