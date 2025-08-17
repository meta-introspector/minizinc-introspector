use thiserror::Error;

pub type Result<T> = std::result::Result<T, ZosError>;

#[derive(Error, Debug)]
pub enum ZosError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Command execution failed: {command} (exit code: {exit_code:?})")]
    CommandFailed {
        command: String,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
    },

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("MiniZinc FFI error: {0}")]
    MiniZincFfiError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<glob::PatternError> for ZosError {
    fn from(err: glob::PatternError) -> Self {
        ZosError::Unknown(format!("Glob pattern error: {}", err))
    }
}

impl From<std::path::StripPrefixError> for ZosError {
    fn from(err: std::path::StripPrefixError) -> Self {
        ZosError::Unknown(format!("Path strip prefix error: {}", err))
    }
}
