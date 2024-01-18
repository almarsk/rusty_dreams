use crate::ChatError;
use std::str::FromStr;

/// user struct should be capable of holding
///         the location (index or chat)
///         the username including when there isnt one
///         the correctness of password
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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

impl ToString for Location {
    fn to_string(&self) -> String {
        match self {
            Location::Login => "Login".to_string(),
            Location::WrongPassword => "WrongPassword".to_string(),
            Location::Chat(user) => format!("Chat {}", user.nick),
        }
    }
}

impl FromStr for Location {
    type Err = ChatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Login" => Ok(Location::Login),
            "WrongPassword" => Ok(Location::WrongPassword),
            _ if s.starts_with("Chat") => {
                let nick = &s[4..]; // Extracting the substring after "Chat"
                Ok(Location::Chat(User {
                    nick: nick.to_string(),
                }))
            }
            _ => Err(ChatError::CookieIssue),
        }
    }
}
