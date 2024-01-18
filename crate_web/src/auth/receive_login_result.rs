use message::{get_buffer, logging_in::LoginResult, LoginDirection, Task};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn receive_login_result(socket: &mut TcpStream) -> LoginResult
where
{
    let db_response = get_buffer(socket).await;
    log::info!("db response arrived");

    if let Ok(mut buffer) = db_response {
        match socket.read(&mut buffer).await {
            Ok(0) => LoginResult::InternalError,
            Ok(_) => {
                if let Ok(Task::User(LoginDirection::Response(login_result))) =
                    bincode::deserialize::<Task>(&buffer)
                {
                    login_result
                } else {
                    LoginResult::InternalError
                }
            }
            _ => LoginResult::InternalError,
        }
    } else {
        LoginResult::InternalError
    }
}
