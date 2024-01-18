use rocket::{form::Form, State};

use crate::Stream;

use super::receive_login_result::receive_login_result;
use message::{
    logging_in::{LoginAttempt, LoginForm, LoginResult},
    send_message,
    LoginDirection::*,
    Task,
};

pub async fn backend_login(stream: &State<Stream>, form: Form<LoginForm>) -> LoginResult {
    let freed_tcp_stream = &mut *stream.lock().await;

    if send_message(
        freed_tcp_stream,
        Task::User(Request(LoginAttempt {
            nick: form.nick.clone(),
            pass: form.pass.clone(),
        })),
    )
    .await
    .is_err()
    {
        log::error!("couldnt send message to db server")
    }

    receive_login_result(freed_tcp_stream).await
}
