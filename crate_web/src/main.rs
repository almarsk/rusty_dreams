use std::collections::HashMap;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[macro_use]
extern crate rocket;
use clap::Parser;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

use message::{send_message, Addressee, MaybeSerializedMessage::*};

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
struct MessageFront {
    message: String,
    nick: String,
}

#[rocket::post("/submit", data = "<form>")]
fn submit(form: Form<LoginForm>) -> RawHtml<String> {
    println!("pass {}", form.password);
    let mut data = HashMap::new();
    data.insert("nickname".to_string(), form.username.clone());
    get_template("chat", Some(data))
}

#[rocket::post("/receive_message", data = "<message>")]
async fn receive_message(
    state: &State<SharedState>,
    message: Json<MessageFront>,
) -> Json<MessageFront> {
    let response_text = message.0.message;
    let nick = message.0.nick;

    let freed_tcp_stream = &mut *state.tcp_stream.lock().await;

    let (_, mut writer) = tokio::io::split(freed_tcp_stream);
    if send_message(
        &mut writer,
        ToSerialize(response_text.as_str(), nick.as_str()),
        Addressee::Server,
    )
    .await
    .is_err()
    {
        log::error!("failed sending the message");
    };

    Json(MessageFront {
        message: response_text,
        nick,
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

struct SharedState {
    tcp_stream: Arc<Mutex<TcpStream>>,
}

#[launch]
async fn rocket() -> _ {
    let (host, port) = Args::parse().deconstruct();
    let tcp_stream = if let Ok(tcp_stream) = connect_to_server(host, port).await {
        tcp_stream
    } else {
        log::error!("couldnt connect to server");
        std::process::exit(1);
    };
    let shared_state = SharedState {
        tcp_stream: Arc::new(Mutex::new(tcp_stream)),
    };
    rocket::build()
        .manage(shared_state)
        .mount("/", routes![index, submit, receive_message])
}
