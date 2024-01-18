use rocket::serde::{Deserialize, Serialize};

use crate::{
    logging_in::{LoginAttempt, LoginResult},
    Message,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Task {
    Message(Message),
    User(LoginDirection),
    History(HistoryDirection),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum HistoryDirection {
    Request,
    Response(Vec<Message>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum LoginDirection {
    Request(LoginAttempt),
    Response(LoginResult),
}
