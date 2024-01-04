//! Crate to support an async chat server

use std::io;

#[macro_use]
extern crate rocket;

mod message;
pub use crate::message::{Message, MessageType};
mod error;
pub use error::ChatError;
mod buffer;
pub use buffer::get_buffer;
pub mod build_message;
mod send_message;
pub use send_message::{send_message, Addressee, MaybeSerializedMessage};
mod login_form;
pub use login_form::LoginForm;

use crossterm::{cursor, execute, terminal};

#[cfg(test)]
mod tests;

pub fn clear_previous_line() {
    if execute!(
        io::stdout(),
        cursor::MoveUp(1),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )
    .is_err()
    {}
}
