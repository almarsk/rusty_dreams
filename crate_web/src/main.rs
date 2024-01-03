use std::collections::HashMap;

#[macro_use]
extern crate rocket;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

mod get_template;
use get_template::get_template;

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

#[rocket::post("/submit", data = "<form>")]
fn submit(form: Form<LoginForm>) -> RawHtml<String> {
    println!("pass {}", form.password);
    let mut data = HashMap::new();
    data.insert("nickname".to_string(), form.username.clone());
    get_template("chat", Some(data))
}

#[rocket::post("/receive_message", data = "<message>")]
fn receive_message(message: Json<Message>) -> Json<Message> {
    let response_text = message.0.message.to_uppercase();
    Json(Message {
        message: response_text,
    })
}

#[rocket::get("/")]
fn index() -> RawHtml<String> {
    get_template("login", None)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit, receive_message])
}
