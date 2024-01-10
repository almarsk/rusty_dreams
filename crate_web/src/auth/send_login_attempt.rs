use message::logging_in::LoginAttempt;
use tokio::io::{AsyncWrite, AsyncWriteExt, WriteHalf};

pub fn _send_login_attempt<T>(_writer: &mut WriteHalf<T>, _form: LoginAttempt)
where
    T: AsyncWriteExt + AsyncWrite,
{
    log::info!("login_attempt")
}
