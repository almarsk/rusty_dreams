use super::User;
use rocket::serde::{Deserialize, Serialize};

#[derive(FromForm)]
pub struct LoginForm {
    pub nick: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum LoginResult {
    #[default]
    WrongPassword,
    NewUser(User),
    ReturningUser(User),
}

#[derive(Serialize, Deserialize)]
pub struct LoginAttempt {
    nick: String,
    pass: String,
}

impl From<LoginForm> for LoginAttempt {
    fn from(form: LoginForm) -> Self {
        LoginAttempt {
            nick: form.nick,
            pass: form.pass,
        }
    }
}
