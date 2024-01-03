use env_logger::Builder;

use message::ChatError;

use std::io::Write;
use std::net::{SocketAddr, TcpStream};

pub async fn connect_to_server(host: String, port: String) -> Result<(), ChatError> {
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
    log::info!("yo les goo");

    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        log::error!("cant use {}:{}, going default", host, port);
        "127.0.0.1:11111"
            .parse()
            .map_err(|_| ChatError::OtherEndIssue)?
    };
    if TcpStream::connect(address).is_ok() {
        log::info!("we on")
    } else {
        log::error!("we not on :((((")
    };
    Ok(())
}
