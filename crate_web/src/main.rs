#![allow(clippy::blocks_in_conditions)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use clap::Parser;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::http::{Cookie, CookieJar, SameSite};
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
async fn login(form: Form<LoginForm>, stream: &State<Stream>, jar: &CookieJar<'_>) -> Redirect {
    log::info!("logging in");
    let login_result = auth::login_backend::backend_login(stream, form).await;

    jar.add(Cookie::build(("user_state", "LoggedIn")).same_site(SameSite::Strict));

    match login_result {
        WrongPassword => jar.add(
            Cookie::build(("user_state", Location::WrongPassword.to_string()))
                .same_site(SameSite::Strict),
        ),
        InternalError => jar.add(
            Cookie::build(("user_state", Location::Login.to_string())).same_site(SameSite::Strict),
        ),

        NewUser(user) | ReturningUser(user) => jar.add(
            Cookie::build(("user_state", Location::Chat(user).to_string()))
                .same_site(SameSite::Strict),
        ),
    }

    Redirect::to(uri!("/"))
}

#[get("/")]
async fn dispatcher(jar: &CookieJar<'_>) -> RawHtml<String> {
    let mut dat = HashMap::new();

    let user_state = if let Ok(us) = Location::from_str(
        jar.get("user_state")
            .map(|cookie| cookie.value())
            .unwrap_or("Login"),
    ) {
        us
    } else {
        Location::Login
    };

    match user_state {
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

    let shared_stream = Arc::new(Mutex::new(tcp_stream));

    rocket::build()
        .manage(channel::<Message>(1024).0)
        .manage(shared_stream)
        .mount("/", routes![post, events, login, dispatcher])
        .mount("/", FileServer::from(relative!("static")))
}
