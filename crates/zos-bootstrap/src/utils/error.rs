use thiserror::Error;

pub type Result<T> = std::result::Result<T, ZosError>;

#[derive(Error, Debug)]
#[allow(dead_code)]
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

        #[error("MiniZinc Ffi error: {0}")]
    MiniZincFfiError(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
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

impl From<std::str::Utf8Error> for ZosError {
    fn from(err: std::str::Utf8Error) -> Self {
        ZosError::Unknown(format!("UTF-8 error: {}", err))
    }
}

impl From<syn::Error> for ZosError {
    fn from(err: syn::Error) -> Self {
        ZosError::Unknown(format!("Syn parsing error: {}", err))
    }
}

impl From<Box<dyn std::error::Error>> for ZosError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ZosError::Unknown(format!("External error: {}", err))
    }
}

impl From<walkdir::Error> for ZosError {
    fn from(err: walkdir::Error) -> Self {
        ZosError::Unknown(format!("Walkdir error: {}", err))
    }
}

