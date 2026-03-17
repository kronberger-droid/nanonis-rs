use thiserror::Error;

/// Error types for Nanonis communication.
///
/// This enum represents the four categories of errors that can occur:
/// - [`Io`](NanonisError::Io) - Network and I/O errors
/// - [`Timeout`](NanonisError::Timeout) - Connection or operation timeouts
/// - [`Protocol`](NanonisError::Protocol) - Binary protocol parsing/validation errors
/// - [`Server`](NanonisError::Server) - Errors returned by the Nanonis server
#[derive(Error, Debug)]
pub enum NanonisError {
    /// IO error with context describing what operation failed.
    ///
    /// # Example
    /// ```
    /// use nanonis_rs::NanonisError;
    ///
    /// let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused");
    /// let err = NanonisError::Io {
    ///     source: io_err,
    ///     context: "connecting to server".to_string(),
    /// };
    /// assert!(err.to_string().contains("connecting to server"));
    /// ```
    #[error("IO error: {context}")]
    Io {
        #[source]
        source: std::io::Error,
        context: String,
    },

    /// Connection or operation timeout.
    ///
    /// Contains an optional description of what timed out.
    ///
    /// # Example
    /// ```
    /// use nanonis_rs::NanonisError;
    ///
    /// let err = NanonisError::Timeout("waiting for scan to complete".to_string());
    /// assert!(err.to_string().contains("scan"));
    /// ```
    #[error("Timeout{}", if .0.is_empty() { String::new() } else { format!(": {}", .0) })]
    Timeout(String),

    /// Protocol error during parsing, validation, or type conversion.
    ///
    /// This covers all errors related to the binary protocol:
    /// - Unexpected response formats
    /// - Type mismatches in received data
    /// - Invalid command parameters
    /// - Serialization failures
    ///
    /// # Example
    /// ```
    /// use nanonis_rs::NanonisError;
    ///
    /// let err = NanonisError::Protocol("Expected f32, got i32".to_string());
    /// assert!(err.to_string().contains("Expected f32"));
    /// ```
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Error returned by the Nanonis server.
    ///
    /// The server returns an error code and message when a command fails.
    ///
    /// # Example
    /// ```
    /// use nanonis_rs::NanonisError;
    ///
    /// let err = NanonisError::Server {
    ///     code: -1,
    ///     message: "Invalid parameter".to_string(),
    /// };
    /// assert!(err.is_server_error());
    /// assert_eq!(err.error_code(), Some(-1));
    /// ```
    #[error("Server error: {message} (code: {code})")]
    Server { code: i32, message: String },
}

impl NanonisError {
    /// Check if this is a server-side error.
    pub fn is_server_error(&self) -> bool {
        matches!(self, NanonisError::Server { .. })
    }

    /// Get error code if this is a server error.
    pub fn error_code(&self) -> Option<i32> {
        match self {
            NanonisError::Server { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Check if this is a timeout error.
    pub fn is_timeout(&self) -> bool {
        matches!(self, NanonisError::Timeout(_))
    }

    /// Check if this is an I/O error.
    pub fn is_io(&self) -> bool {
        matches!(self, NanonisError::Io { .. })
    }

    /// Check if this is a protocol error.
    pub fn is_protocol(&self) -> bool {
        matches!(self, NanonisError::Protocol(_))
    }
}

// Allow conversion from std::io::Error
impl From<std::io::Error> for NanonisError {
    fn from(error: std::io::Error) -> Self {
        NanonisError::Io {
            source: error,
            context: "IO operation failed".to_string(),
        }
    }
}

// Allow conversion from serde_json::Error
impl From<serde_json::Error> for NanonisError {
    fn from(error: serde_json::Error) -> Self {
        NanonisError::Protocol(format!("JSON serialization error: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_io_error() -> NanonisError {
        NanonisError::Io {
            source: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused"),
            context: "connecting".to_string(),
        }
    }

    #[test]
    fn predicates() {
        let io = make_io_error();
        assert!(io.is_io());
        assert!(!io.is_timeout());
        assert!(!io.is_protocol());
        assert!(!io.is_server_error());

        let timeout = NanonisError::Timeout("scan".into());
        assert!(timeout.is_timeout());
        assert!(!timeout.is_io());

        let proto = NanonisError::Protocol("bad".into());
        assert!(proto.is_protocol());
        assert!(!proto.is_server_error());

        let server = NanonisError::Server { code: -1, message: "fail".into() };
        assert!(server.is_server_error());
        assert!(!server.is_protocol());
    }

    #[test]
    fn error_code() {
        assert_eq!(NanonisError::Server { code: 42, message: "".into() }.error_code(), Some(42));
        assert_eq!(NanonisError::Server { code: -1, message: "".into() }.error_code(), Some(-1));
        assert_eq!(NanonisError::Protocol("x".into()).error_code(), None);
        assert_eq!(NanonisError::Timeout("".into()).error_code(), None);
        assert_eq!(make_io_error().error_code(), None);
    }

    #[test]
    fn display_formats() {
        assert!(NanonisError::Timeout("scan".into()).to_string().contains("scan"));
        assert_eq!(NanonisError::Timeout("".into()).to_string(), "Timeout");
        assert!(NanonisError::Protocol("bad parse".into()).to_string().contains("bad parse"));
        let s = NanonisError::Server { code: -1, message: "Invalid".into() }.to_string();
        assert!(s.contains("Invalid") && s.contains("-1"));
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe");
        let err: NanonisError = io_err.into();
        assert!(err.is_io());
        assert!(err.to_string().contains("IO operation failed"));
    }
}
