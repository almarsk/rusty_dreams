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
    History(TaskDirection<Message>),
    Mannschaft(TaskDirection<String>),
    Delete(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TaskDirection<T> {
    Request,
    Response(Vec<T>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum LoginDirection {
    Request(LoginAttempt),
    Response(LoginResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum ServerTask {
    Message(Message),
    Deletion,
}
