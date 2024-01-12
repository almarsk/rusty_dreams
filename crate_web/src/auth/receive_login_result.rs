use message::logging_in::LoginResult;
use tokio::io::{AsyncRead, AsyncReadExt, ReadHalf};

pub fn receive_login_result<T>(_reader: &mut ReadHalf<T>) -> LoginResult
where
    T: AsyncRead + AsyncReadExt,
{
    LoginResult::default()
}
