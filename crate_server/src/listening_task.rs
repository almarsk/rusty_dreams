use flume::{Receiver, Sender};

use crate::task::Task;

#[allow(clippy::needless_lifetimes)]
pub async fn listen<'a>(_rx: Receiver<Task>, _tx: Sender<Task>) {
    // this is where we listen like in the example
    // clients hashmap with readers will be iterated
}
