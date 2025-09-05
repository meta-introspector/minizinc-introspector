use thiserror::Error;

#[derive(Error, Debug)]
pub enum TmuxControllerError {
    #[error("Tmux command failed: {0}")]
    TmuxCommandError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse output: {0}")]
    ParseError(String),
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    #[error("Unknown error")]
    Unknown,
    #[error("Generic error: {0}")]
    GenericError(String),
}

// Implement From for tmux_interface::Error
impl From<tmux_interface::Error> for TmuxControllerError {
    fn from(err: tmux_interface::Error) -> Self {
        TmuxControllerError::TmuxCommandError(err.to_string())
    }
}