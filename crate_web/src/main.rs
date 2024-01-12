#![allow(clippy::blocks_in_conditions)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;

use clap::Parser;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::content::RawHtml;
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};

mod args;
mod handlebars;
mod logger;
use message::logging_in::{LoginForm, LoginResult::*};
mod connect_to_server;
use connect_to_server::connect_to_server;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
mod auth;

use message::Location;

type Stream = Arc<Mutex<TcpStream>>;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    log::info!("new message {}", form.clone().message);
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[post("/login", data = "<form>")]
async fn login(
    form: Form<LoginForm>,
    stream: &State<Stream>,
    location: &State<Arc<Mutex<Location>>>,
) -> Redirect {
    log::info!("loging in");
    let login_result = auth::login_backend::backend_login(stream, form).await;
    let mut location = location.inner().lock().await;

    match login_result {
        WrongPassword => *location = Location::WrongPassword,
        InternalError => *location = Location::Login,
        NewUser(user) | ReturningUser(user) => *location = Location::Chat(user),
    }

    Redirect::to(uri!("/"))
}

#[get("/")]
async fn dispatcher(location: &State<Arc<Mutex<Location>>>) -> RawHtml<String> {
    let location = &mut location.inner().lock().await;
    let location = location.deref_mut();
    let mut dat = HashMap::new();

    match location {
        Location::Login => {
            log::info!("reading login html");
            dat.insert(String::from("wrongPass"), String::from("false"));
            handlebars::get_template(String::from("index"), Some(dat))
        }
        Location::WrongPassword => {
            dat.insert(String::from("wrongPass"), String::from("true"));
            handlebars::get_template(String::from("index"), Some(dat))
        }
        Location::Chat(user) => {
            dat.insert(String::from("nickname"), user.nick.clone());
            handlebars::get_template(String::from("chat"), Some(dat))
        }
    }
}

#[launch]
async fn rocket() -> _ {
    logger::build_logger();
    let (host, port) = args::Args::parse().deconstruct();

    let tcp_stream = if let Ok(tcp_stream) = connect_to_server(host, port).await {
        tcp_stream
    } else {
        log::error!("couldnt connect to server");
        std::process::exit(1);
    };

    let user = Arc::new(Mutex::new(Location::Login));
    let shared_stream = Arc::new(Mutex::new(tcp_stream));

    rocket::build()
        .manage(channel::<Message>(1024).0)
        .manage(shared_stream)
        .manage(user)
        .mount("/", routes![post, events, login, dispatcher])
        .mount("/", FileServer::from(relative!("static")))
}
