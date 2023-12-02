use env_logger;
use flume::{bounded, Receiver, Sender};
use tokio::{net::TcpListener, try_join};

mod accepting_task;
use accepting_task::accepting_task;
mod broadcasting_task;
use broadcasting_task::accomodate_and_broadcast;
mod listening_task;
use listening_task::listen;
pub mod task;
use task::Task;

// TODO LOGGING
// TODO PARSE HOST & PORT

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6666").await?;
    let (tx_accomodate, rx_accomodate): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_listen, rx_listen): (Sender<Task>, Receiver<Task>) = bounded(10);
    let (tx_send, rx_send): (Sender<Task>, Receiver<Task>) = bounded(10);
    println!("starting a new thing");

    let accepting_task = tokio::task::spawn(accepting_task(listener, tx_accomodate, tx_listen));
    let broadcasting_task = tokio::task::spawn(accomodate_and_broadcast(rx_accomodate, rx_send));
    let listening_task = tokio::task::spawn(listen(rx_listen, tx_send));

    match try_join!(accepting_task, broadcasting_task, listening_task) {
        Ok(i) => {
            if let Err(e) = i.0 {
                eprintln!("It failed: {}", e)
            };
        }
        Err(e) => eprintln!("It failed: {}", e),
    }

    Ok(())
}
