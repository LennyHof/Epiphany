use std::{
    error::Error,
    fmt::{Debug, Display},
};

/// ProviderError represents a database error.
#[derive(Debug, PartialEq)]
pub enum ProviderError {
    /// Failure to obtain a lock.
    Lock(String),
    /// A general database failure.
    General(String),
}

impl Error for ProviderError {}

impl Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::Lock(msg) => write!(f, "Lock Error: {}", msg),
            ProviderError::General(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl From<String> for ProviderError {
    fn from(value: String) -> Self {
        ProviderError::General(value)
    }
}
