#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use clap::Parser;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::content::RawHtml;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};

mod args;
mod handlebars;
mod logger;
use message::logging_in::LoginForm;

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
async fn login(form: Form<LoginForm>) -> RawHtml<String> {
    log::info!("login in");
    log::info!("nick {} pass {}", form.nick, form.pass);
    //  let user = backend_login(state, form).await;
    //  log::info!("{:?}", user);
    let mut data = HashMap::new();
    data.insert(String::from("nickname"), String::from("Blel"));
    handlebars::get_template(String::from("chat"), Some(data))
}

#[launch]
fn rocket() -> _ {
    logger::build_logger();
    let (_host, _port) = args::Args::parse().deconstruct();

    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events, login])
        .mount("/", FileServer::from(relative!("static")))
}
