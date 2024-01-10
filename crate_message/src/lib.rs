mod user;
pub use user::User;
mod error;
pub mod logging_in;
pub use error::ChatError;

#[macro_use]
extern crate rocket;
