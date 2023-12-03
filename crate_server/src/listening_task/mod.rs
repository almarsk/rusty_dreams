use flume::{Receiver, Sender};

use crate::task::Task;
mod read_from_socket;
use read_from_socket::read_from_socket;

pub async fn listen(rx_accomodate: Receiver<Task>, tx: Sender<Task>) {
    // this is where we listen like in the example
    // clients hashmap with readers will be iterated

    // accomodation future should work with the arc and mutex
    tokio::task::spawn(async move {
        println!("accomodating task init");

        loop {
            while let Ok(t) = rx_accomodate.recv() {
                match t {
                    Task::ConnRead(a, mut c) => {
                        let tx_clone = tx.clone();
                        tokio::task::spawn(async move {
                            if read_from_socket(&mut c, tx_clone, a).await.is_err() {
                                eprintln!("Issue reading from {}", a)
                            }
                        });
                        tokio::task::yield_now().await;
                    }
                    _ => eprintln!("Something else than Writehal coming in accomodating task"),
                }
            }
        }
    });
}
