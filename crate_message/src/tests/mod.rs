use crate::build_message::build_message_w_path;

use super::*;

impl MessageType {
    fn text(&self) -> &str {
        match self {
            Self::Text(t) => t.as_str(),
            _ => "lulzwut",
        }
    }
}

#[test]
fn make_message() {
    let message = Message::new("lol", "nick".to_string()).unwrap();
    let message2 = Message {
        content: MessageType::Text("lol".to_string()),
        nick: "nick".to_string(),
    };

    assert_eq!(message.content.text(), message2.content.text());
    assert_eq!(message.nick, message2.nick)
}

#[test]
#[should_panic]
fn invalid_path() {
    Message::new(".file qwerty", "nick".to_string()).unwrap();
}

#[test]
fn not_welcome() {
    assert_eq!(MessageType::Welcome(Ok(())).into_db(), None)
}

#[test]
#[should_panic]
fn file_incoming() {
    assert_eq!(
        MessageType::Image(vec![0]).into_db(),
        Some("incoming_image".to_string())
    )
}

#[test]
fn err() {
    assert_eq!(
        format!("{}", ChatError::DatabaseIssue),
        "Database error".to_string()
    )
}

#[test]
fn welcome() {
    let welcome = build_message_w_path(
        "nick".to_string(),
        ".accept rn",
        MessageType::Welcome(Ok(())),
    );

    assert!(welcome.is_err())
}
