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
    #[error("Error reading from other end")]
    OtherEndIssue,
    #[error("Error writing to the other end")]
    WritingIssue,
    #[error("Error deserializing")]
    DeserializingIssue,
    #[error("Error saving file")]
    SavingIssue,
    #[error("Error creating user dir")]
    UserDirectoryIssue,
    #[error("Error finding path - provide a valid one")]
    PathIssue,
    #[error("Error finding path - provide one")]
    NoPathIssue,
    #[error("Error reading from path")]
    ReadingIssue,
}
