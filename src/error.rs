use thiserror::Error;

#[derive(Error, Debug)]
pub enum NanonisError {
    /// Network or I/O operation failed
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Connection or operation timed out
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Binary protocol parsing or validation error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Type conversion or mismatch error (internal)
    #[error("Type error: {0}")]
    Type(String),

    /// Invalid user input or command parameters
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Nanonis server returned an error
    #[error("Nanonis error (code {code}): {message}")]
    Server { code: i32, message: String },
}

impl NanonisError {
    /// Helper to add context to I/O errors (for internal use)
    pub(crate) fn io_context(err: std::io::Error, context: impl Into<String>) -> Self {
        NanonisError::Io(std::io::Error::new(
            err.kind(),
            format!("{}: {}", context.into(), err),
        ))
    }
}

// Handle serde_json errors by converting to Protocol errors
impl From<serde_json::Error> for NanonisError {
    fn from(error: serde_json::Error) -> Self {
        NanonisError::Protocol(format!("JSON serialization error: {}", error))
    }
}
