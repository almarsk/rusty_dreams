use std::path::Path;

use chrono::Local;
use message::{ChatError, MessageType};

pub fn receive_and_save(message: MessageType, nick: &str) -> Result<(), ChatError> {
    let path = format!("media/users/{}", nick);
    std::fs::create_dir_all(&path).map_err(|_| ChatError::UserDirectoryIssue)?;

    match message {
        MessageType::File(name, file_content) => {
            let file_path = Path::new(&path).join(name);
            std::fs::write(file_path, file_content).map_err(|_| ChatError::SavingIssue)?;
        }
        MessageType::Image(data) => {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let file_path = Path::new(&path).join(timestamp);
            std::fs::write(format!("{}.png", file_path.to_string_lossy()), data)
                .map_err(|_| ChatError::SavingIssue)?;
        }
        _ => (),
    }

    Ok(())
}
