use std::io;

mod message;
pub use message::{Message, MessageType};
mod error;
pub use error::ChatError;
mod build_message;

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
