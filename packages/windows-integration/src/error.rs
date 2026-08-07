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

    #[error("Clipboard failed: {0}")]
    ClipboardFailed(String),

    #[error("Notification failed: {0}")]
    NotificationFailed(String),

    #[error("Browser failed: {0}")]
    BrowserFailed(String),

    #[error("Screenshot failed: {0}")]
    ScreenshotFailed(String),
}
