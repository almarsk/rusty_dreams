use std::collections::HashMap;
use std::sync::Arc;

//use flume::Sender;

use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::response::{content, status};
use rocket::State;
mod handlebars;
mod logger;
use clap::Parser;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use message::logging_in::LoginForm;
mod auth;

//use auth::login_backend::backend_login;
mod args;
mod connect_to_server;
//use connect_to_server::connect_to_server;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> RawHtml<String> {
    let mut data = HashMap::new();
    data.insert(String::from("invalid_login"), String::from("false"));
    handlebars::get_template(String::from("login"), Some(data))
}

#[get("/chat")]
fn chat() -> RawHtml<String> {
    let mut data = HashMap::new();
    data.insert(String::from("nickname"), String::from("Blel"));
    handlebars::get_template(String::from("chat"), Some(data))
}

#[post("/login", data = "<form>")]
async fn login(
    //ws: ws::WebSocket,
    form: Form<LoginForm>,
    //_stream: &State<Stream>, // to communicate with db server
    message_receivers: &State<Arc<Mutex<Vec<ws::WebSocket>>>>,
) -> RawHtml<String> {
    log::info!("login in");
    log::info!("nick {} pass {}", form.nick, form.pass);
    //  let user = backend_login(state, form).await;
    //  log::info!("{:?}", user);
    //message_receivers.lock().await.push(ws);
    let mut data = HashMap::new();
    data.insert(String::from("nickname"), String::from("Blel"));
    handlebars::get_template(String::from("chat"), Some(data))
}

#[get("/register_listener")]
async fn echo_stream(_ws: ws::WebSocket) -> status::Custom<content::RawHtml<&'static str>> {
    log::info!("listener registered; todo send over to clients");

    let response = content::RawHtml("<h1>WebSocket listener registered</h1>");
    status::Custom(Status::Ok, response)
}

type Stream = Arc<Mutex<TcpStream>>;

#[launch]
async fn rocket() -> _ {
    logger::build_logger();
    let (_host, _port) = args::Args::parse().deconstruct();
    /*
    let tcp_stream = if let Ok(tcp_stream) = connect_to_server(host, port).await {
        tcp_stream
    } else {
        log::error!("couldnt connect to server");
        std::process::exit(1);
    };
    let shared_stream = Arc::new(Mutex::new(tcp_stream));
    */

    let message_receivers: Arc<Mutex<Vec<ws::WebSocket>>> = Arc::new(Mutex::new(vec![]));

    rocket::build()
        //.manage(shared_stream)
        .manage(message_receivers)
        .mount("/", routes![index, echo_stream, chat, login])
}
