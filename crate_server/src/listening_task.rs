use flume::{Receiver, Sender};

use crate::task::Task;

#[allow(clippy::needless_lifetimes)]
pub async fn listen<'a>(_rx: Receiver<Task>, _tx: Sender<Task>) {}
