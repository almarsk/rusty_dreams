use flume::{Receiver, Sender};
use message::ChatError;

use crate::task_type::Task;
mod read_from_socket;
use read_from_socket::read_from_socket;

pub async fn listen(
    rx_accomodate: Receiver<Task>,
    tx: Sender<(Task, i32)>,
) -> Result<(), ChatError> {
    // this is where we listen like in the example
    tokio::task::spawn(async move {
        log::info!("starting accomodation task");

        loop {
            while let Ok(t) = rx_accomodate.recv() {
                match t {
                    Task::ConnRead(a, mut c, client_id) => {
                        let tx_clone = tx.clone();

                        tokio::task::spawn(async move {
                            if read_from_socket(&mut c, tx_clone, a, client_id)
                                .await
                                .is_err()
                            {
                                log::error!("Issue reading from {}", a)
                            }
                        });
                        tokio::task::yield_now().await;
                    }
                    _ => log::error!(
                        "Something else than Writehalf coming from {}",
                        match t {
                            Task::Message(a, _) => a,
                            Task::ConnRead(a, _, _) => a,
                            Task::ConnWrite(a, _) => a,
                        }
                    ),
                }
            }
        }
    });
    Ok(())
}
