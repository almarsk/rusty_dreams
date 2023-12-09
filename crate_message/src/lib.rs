use std::io;

mod message;
pub use message::{Message, MessageType};
mod error;
pub use error::ChatError;
mod buffer;
pub use buffer::get_buffer;
mod build_message;
mod send_message;
pub use send_message::{send_message, Addressee, MaybeSerializedMessage};

use crossterm::{cursor, execute, terminal};

pub fn clear_previous_line() {
    if execute!(
        io::stdout(),
        cursor::MoveUp(1),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )
    .is_err()
    {}
}
