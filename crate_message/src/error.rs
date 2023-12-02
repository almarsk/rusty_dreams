use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Error passing message to distribution")]
    PassToSendIssue,
    #[error("Error accomodating new contact")]
    AccomodationIssue,
    #[error("Error accepting new contact")]
    AcceptanceIssue,
    #[error("Error greeting new contact")]
    GreetingIssue,
}
