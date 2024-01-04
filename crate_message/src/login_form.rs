use rocket::serde::{Deserialize, Serialize};

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct LoginForm {
    username: String,
    password: String,
}
