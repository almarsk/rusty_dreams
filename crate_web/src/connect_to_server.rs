use tokio::net::TcpStream;

use std::net::SocketAddr;

use message::ChatError;

pub async fn _connect_to_server(host: String, port: String) -> Result<TcpStream, ChatError> {
    let address: SocketAddr = if let Ok(a) = format!("{}:{}", host, port).parse() {
        a
    } else {
        log::error!("cant use {}:{}, going default", host, port);
        "127.0.0.1:11111"
            .parse()
            .map_err(|_| ChatError::OtherEndIssue)?
    };
    if let Ok(tcp_stream) = TcpStream::connect(address).await {
        log::info!("we on");
        Ok(tcp_stream)
    } else {
        log::error!("we not on :((((");
        Err(ChatError::AcceptanceIssue)
    }
}
