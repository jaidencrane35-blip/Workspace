use thiserror::Error;

pub type Result<T> = std::result::Result<T, WindowsIntegrationError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WindowsIntegrationError {
    #[error("Window enumeration failed: {0}")]
    EnumerationFailed(String),

    #[error("Process launch failed: {0}")]
    LaunchFailed(String),

    #[error("Invalid launch target: {0}")]
    InvalidLaunchTarget(String),

    #[error("Invalid window handle: {0}")]
    InvalidWindowHandle(String),

    #[error("Invalid window bounds: {0}")]
    InvalidWindowBounds(String),

    #[error("Window control failed: {0}")]
    ControlFailed(String),
}
