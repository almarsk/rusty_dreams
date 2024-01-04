use std::collections::HashMap;
use std::sync::Arc;

use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[macro_use]
extern crate rocket;
use clap::Parser;
use rocket::form::Form;
use rocket::response::content::RawHtml;
use rocket::response::{Flash, Redirect};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::uri;
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
fn submit(form: Form<LoginForm>) -> Flash<Redirect> {
    println!("pass {}", form.password);
    println!("nick {}", form.username);
    let username = form.username.clone();
    //let is_valid = authenticate_user(&form.username, &form.password);

    if true {
        //is_valid {

        Flash::success(Redirect::to(uri!(chat(username))), "Login succesful")
    } else {
        Flash::error(
            Redirect::to(uri!(index_bool(Some(false)))),
            "Invalid login credentials",
        )
    }
}

#[rocket::get("/chat_page/<username>")]
fn chat(username: String) -> RawHtml<String> {
    let mut data = HashMap::new();
    data.insert("nickname".to_string(), username);
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

#[rocket::get("/?<login>")]
fn index_bool(login: Option<bool>) -> RawHtml<String> {
    let mut login_map = HashMap::new();
    login_map.insert(
        "login_success".to_string(),
        if let Some(false) = login {
            "false".to_string()
        } else {
            "true".to_string()
        },
    );
    get_template("login", Some(login_map))
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
        .mount("/", routes![index_bool, submit, receive_message, chat])
}
