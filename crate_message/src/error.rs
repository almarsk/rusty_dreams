use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Error accomodating new contact")]
    AccomodationIssue,
    #[error("Error accepting new contact")]
    AcceptanceIssue,
}
