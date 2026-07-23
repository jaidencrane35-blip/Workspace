use thiserror::Error;

pub type Result<T> = std::result::Result<T, WindowsIntegrationError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WindowsIntegrationError {
    #[error("Window enumeration failed: {0}")]
    EnumerationFailed(String),
}
