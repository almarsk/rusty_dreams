use std::collections::HashMap;

#[macro_use]
extern crate rocket;
use clap::Parser;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

mod get_template;
use get_template::get_template;
mod connect_to_server;
use connect_to_server::connect_to_server;

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
    #[arg(long, default_value_t = String::from("11111"))]
    port: String,
}

impl Args {
    fn deconstruct(self) -> (String, String) {
        (self.host, self.port)
    }
}

#[launch]
async fn rocket() -> _ {
    let (host, port) = Args::parse().deconstruct();
    if connect_to_server(host, port).await.is_err() {
        log::error!("couldnt connect to server");
    };
    rocket::build().mount("/", routes![index, submit, receive_message])
}
