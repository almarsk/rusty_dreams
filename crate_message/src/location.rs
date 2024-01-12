/// user struct should be capable of holding
///         the location (index or chat)
///         the username including when there isnt one
///         the correctness of password
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub nick: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Location {
    #[default]
    Login,
    WrongPassword,
    Chat(User),
}
