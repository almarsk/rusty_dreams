#![allow(clippy::blocks_in_conditions)]
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
    pub deleted: bool,
    pub date: String,
}
