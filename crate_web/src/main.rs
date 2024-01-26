#![allow(clippy::blocks_in_conditions)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use clap::Parser;
use message::{get_buffer, Task};
use once_cell::sync::Lazy;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::content::RawHtml;
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Shutdown, State};
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics,
};

mod args;
mod handlebars;
mod logger;
use message::logging_in::{LoginForm, LoginResult::*};
mod connect_to_server;
use connect_to_server::connect_to_server;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
mod auth;

use message::{send_message, Location, Message, ServerTask, TaskDirection::*};

type Stream = Arc<Mutex<TcpStream>>;

static MESSAGE_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("message_counter", "Count of messages"), &["message"])
        .expect("Could not create NAME_COUNTER")
});

#[get("/history")]
async fn history(socket: &State<Stream>) -> Json<Vec<Message>> {
    // send message to be written into database
    log::info!("H I S T O R Y T I M E");
    let socket = &mut *socket.lock().await;
    if send_message(socket, Task::History(Request)).await.is_err() {
        log::error!("couldnt send message to db server")
    };
    log::info!("message sent; waiting for response from db");
    let db_response = get_buffer(socket).await;
    log::info!("db response arrived");

    if let Ok(mut buffer) = db_response {
        match socket.read(&mut buffer).await {
            Ok(0) => Json(vec![]),
            Ok(_) => {
                if let Ok(Task::History(Response(h))) = bincode::deserialize::<Task>(&buffer) {
                    Json(h)
                } else {
                    Json(vec![])
                }
            }
            _ => Json(vec![]),
        }
    } else {
        Json(vec![])
    }
}

#[get("/mannschaft")]
async fn mannschaft(socket: &State<Stream>) -> Json<Vec<String>> {
    log::info!("mschft rockt");
    // send message to be written into database
    let socket = &mut *socket.lock().await;
    if send_message(socket, Task::Mannschaft(Request))
        .await
        .is_err()
    {
        log::error!("couldnt send message to db server")
    };
    log::info!("message sent; waiting for response from db");
    let db_response = get_buffer(socket).await;
    log::info!("db response arrived");

    if let Ok(mut buffer) = db_response {
        match socket.read(&mut buffer).await {
            Ok(0) => Json(vec![]),
            Ok(_) => {
                if let Ok(Task::Mannschaft(Response(m))) = bincode::deserialize::<Task>(&buffer) {
                    log::info!("users: {:?}", m);
                    Json(m)
                } else {
                    Json(vec![])
                }
            }
            _ => Json(vec![]),
        }
    } else {
        Json(vec![])
    }
}

#[post("/message", data = "<form>")]
async fn post(form: Form<Message>, queue: &State<Sender<ServerTask>>, writer: &State<Stream>) {
    log::info!("new message {}", form.clone().message);

    // send message to be written into database
    let writer = &mut *writer.lock().await;
    if send_message(writer, Task::Message(form.clone()))
        .await
        .is_err()
    {
        log::error!("couldnt send message to db server")
    };
    MESSAGE_COUNTER.with_label_values(&["message"]).inc();
    let _res = queue.send(ServerTask::Message(form.into_inner()));
}

#[delete("/delete?<user>")]
async fn delete(user: String, queue: &State<Sender<ServerTask>>, writer: &State<Stream>) {
    log::info!("deleting user {}", user);

    // send message to be written into database
    let writer = &mut *writer.lock().await;
    if send_message(writer, Task::Delete(user)).await.is_err() {
        log::error!("couldnt send message to db server")
    };

    let _res = queue.send(ServerTask::Deletion);
}

#[get("/events")]
async fn events(queue: &State<Sender<ServerTask>>, mut end: Shutdown) -> EventStream![] {
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
    jar: &CookieJar<'_>,
    queue: &State<Sender<ServerTask>>,
) -> Redirect {
    log::info!(
        "login request from {} with password {}",
        form.nick,
        form.pass
    );

    let login_result = auth::login_backend::backend_login(stream, form).await;

    jar.add(Cookie::build(("user_state", "LoggedIn")).same_site(SameSite::Strict));

    log::info!("{:?}", login_result);

    match login_result {
        WrongPassword => jar.add(
            Cookie::build(("user_state", Location::WrongPassword.to_string()))
                .same_site(SameSite::Strict),
        ),
        InternalError => jar.add(
            Cookie::build(("user_state", Location::Login.to_string())).same_site(SameSite::Strict),
        ),
        NewUser(user) => {
            // push to users event stream
            let _res = queue.send(ServerTask::NewUser(user.nick.clone()));
            jar.add(
                Cookie::build(("user_state", Location::Chat(user).to_string()))
                    .same_site(SameSite::Strict),
            );
            jar.add(Cookie::build(("new_user", "true")).same_site(SameSite::Strict))
        }
        ReturningUser(user) => jar.add(
            Cookie::build(("user_state", Location::Chat(user).to_string()))
                .same_site(SameSite::Strict),
        ),
        DeletedUser => jar.add(
            Cookie::build(("user_state", Location::Deleted.to_string()))
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

    log::info!("{:?}", user_state);

    match user_state {
        Location::Login => {
            log::info!("reading login html");
            handlebars::get_template(String::from("index"), Some(dat))
        }
        Location::WrongPassword => {
            log::info!("wron pass");
            dat.insert(String::from("issue"), String::from("wrong password"));
            handlebars::get_template(String::from("index"), Some(dat))
        }
        Location::Deleted => {
            log::info!("del user");
            dat.insert(String::from("issue"), String::from("deleted user"));
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

    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(MESSAGE_COUNTER.clone()))
        .unwrap();

    let tcp_stream = if let Ok(tcp_stream) = connect_to_server(host, port).await {
        tcp_stream
    } else {
        log::error!("couldnt connect to server");
        std::process::exit(1);
    };
    let shared_stream = Arc::new(Mutex::new(tcp_stream));

    rocket::build()
        .manage(channel::<ServerTask>(1024).0)
        .manage(shared_stream)
        .attach(prometheus.clone())
        .mount("/metrics", prometheus)
        .mount(
            "/",
            routes![post, events, login, dispatcher, history, mannschaft, delete],
        )
        .mount("/", FileServer::from(relative!("static")))
}
