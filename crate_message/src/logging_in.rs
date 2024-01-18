#![allow(clippy::blocks_in_conditions)]

use super::User;
use rocket::serde::{Deserialize, Serialize};

#[derive(FromForm)]
pub struct LoginForm {
    pub nick: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum LoginResult {
    #[default]
    WrongPassword,
    InternalError,
    NewUser(User),
    ReturningUser(User),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginAttempt {
    pub nick: String,
    pub pass: String,
}

impl LoginAttempt {
    pub fn dec(self) -> (String, String) {
        (self.nick, self.pass)
    }
}

impl From<LoginForm> for LoginAttempt {
    fn from(form: LoginForm) -> Self {
        LoginAttempt {
            nick: form.nick,
            pass: form.pass,
        }
    }
}
