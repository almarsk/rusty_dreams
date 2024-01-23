mod location;
pub use location::{Location, User};
mod error;
pub mod logging_in;
pub use error::ChatError;
mod send_message;
pub use send_message::send_message;
pub mod message;
mod task;
pub use message::Message;
pub use task::{LoginDirection, ServerTask, Task, TaskDirection};
mod buffer;
pub use buffer::get_buffer;

#[macro_use]
extern crate rocket;
