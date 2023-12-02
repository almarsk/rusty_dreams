use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use flume::{Receiver, Sender};
use futures::executor::block_on;
use tokio::{io::ReadHalf, net::TcpStream, sync::Mutex};

use crate::task::Task;
mod read_from_socket;
use read_from_socket::read_from_socket;

#[allow(clippy::needless_lifetimes)]
pub async fn listen<'a>(rx_accomodate: Receiver<Task>, tx: Sender<Task>) {
    // this is where we listen like in the example
    // clients hashmap with readers will be iterated
    let clients: Arc<Mutex<HashMap<SocketAddr, ReadHalf<TcpStream>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let clients_a = clients.clone();

    // accomodation future should work with the arc and mutex
    tokio::task::spawn(async move {
        loop {
            while let Ok(t) = rx_accomodate.try_recv() {
                match t {
                    Task::ConnRead(a, c) => {
                        if let Ok(mut h) = clients_a.clone().try_lock() {
                            h.insert(a, c);
                        } else {
                            eprintln!("Couldnt accomodate {}", a)
                        }
                    }
                    _ => eprintln!("Something else than Writehal coming in accomodating task"),
                }
            }
        }
    });

    // listening task
    tokio::task::spawn(async move {
        // loop over clients and try to read from readhalfs
        // if there is something send it to the broadcasting task

        let tx_arc = Arc::new(tx);

        loop {
            if let Ok(mut h) = clients.try_lock() {
                h.iter_mut().for_each(|(a, socket)| {
                    if block_on(read_from_socket(socket, tx_arc.clone(), *a)).is_err() {
                        eprintln!("Issue reading from {}", a)
                    };
                });
            }
        }
    });
}

/*

*/
