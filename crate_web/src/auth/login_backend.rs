use rocket::{form::Form, State};

use crate::Stream;

use super::{receive_login_result::_receive_login_result, send_login_attempt::_send_login_attempt};
use message::logging_in::{LoginAttempt, LoginForm, LoginResult};

pub async fn _backend_login(stream: &State<Stream>, form: Form<LoginForm>) -> LoginResult {
    let freed_tcp_stream = &mut *stream.lock().await;
    let (mut reader, mut writer) = tokio::io::split(freed_tcp_stream);

    _send_login_attempt(&mut writer, LoginAttempt::from(form.into_inner()));
    _receive_login_result(&mut reader)
}
