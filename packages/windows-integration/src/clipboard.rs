//! Clipboard port — Workspace-owned interface; WRAP commodity backends (P10).
//!
//! Product identity never leaks `arboard` into Conversation or Intent.

use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Desktop clipboard access behind a Workspace-owned port.
pub trait ClipboardPort: Send + Sync {
    fn read_text(&self) -> Result<String>;
    fn write_text(&self, text: &str) -> Result<()>;
}

/// In-process clipboard for tests and non-production bootstrap.
#[derive(Debug, Default)]
pub struct MemoryClipboard {
    text: Mutex<String>,
}

impl MemoryClipboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            text: Mutex::new(text.into()),
        }
    }
}

impl ClipboardPort for MemoryClipboard {
    fn read_text(&self) -> Result<String> {
        self.text
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| WindowsIntegrationError::ClipboardFailed("clipboard lock poisoned".into()))
    }

    fn write_text(&self, text: &str) -> Result<()> {
        let mut guard = self
            .text
            .lock()
            .map_err(|_| WindowsIntegrationError::ClipboardFailed("clipboard lock poisoned".into()))?;
        *guard = text.to_string();
        Ok(())
    }
}

/// Production WRAP of the `arboard` crate.
pub struct ArboardClipboard;

impl ArboardClipboard {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ArboardClipboard {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardPort for ArboardClipboard {
    fn read_text(&self) -> Result<String> {
        let mut clipboard = arboard::Clipboard::new().map_err(|error| {
            WindowsIntegrationError::ClipboardFailed(format!("open clipboard: {error}"))
        })?;
        match clipboard.get_text() {
            Ok(text) => Ok(text),
            Err(arboard::Error::ContentNotAvailable) => Ok(String::new()),
            Err(error) => Err(WindowsIntegrationError::ClipboardFailed(format!(
                "read clipboard: {error}"
            ))),
        }
    }

    fn write_text(&self, text: &str) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new().map_err(|error| {
            WindowsIntegrationError::ClipboardFailed(format!("open clipboard: {error}"))
        })?;
        clipboard.set_text(text.to_string()).map_err(|error| {
            WindowsIntegrationError::ClipboardFailed(format!("write clipboard: {error}"))
        })
    }
}

/// Production OS clipboard (WRAP `arboard`). Tests should prefer [`MemoryClipboard`].
pub fn platform_clipboard() -> std::sync::Arc<dyn ClipboardPort> {
    std::sync::Arc::new(ArboardClipboard::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_clipboard_roundtrip() {
        let port = MemoryClipboard::new();
        port.write_text("hello workspace").unwrap();
        assert_eq!(port.read_text().unwrap(), "hello workspace");
    }
}
