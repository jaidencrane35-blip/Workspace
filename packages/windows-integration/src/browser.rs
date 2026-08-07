//! Browser port — Workspace-owned interface; WRAP URL open (P14).
//!
//! Product identity never leaks `webbrowser` into Conversation.

use std::path::Path;
use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Level 1 browser capability snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserCapabilityStatus {
    pub available: bool,
    pub default_handler: String,
    pub browsers: Vec<String>,
    pub message: String,
}

/// Outcome of open / focus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserOperationOutcome {
    pub ok: bool,
    pub target: Option<String>,
    pub message: String,
}

/// Desktop browser access behind a Workspace-owned port.
pub trait BrowserPort: Send + Sync {
    fn status(&self) -> Result<BrowserCapabilityStatus>;
    fn open_url(&self, url: &str) -> Result<BrowserOperationOutcome>;
    fn focus(&self, query: &str) -> Result<BrowserOperationOutcome>;
}

fn detect_installed_browsers() -> Vec<String> {
    let candidates: &[(&str, &str)] = &[
        (
            "Chrome",
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        ),
        (
            "Chrome",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ),
        (
            "Edge",
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        ),
        (
            "Edge",
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        ),
        (
            "Firefox",
            r"C:\Program Files\Mozilla Firefox\firefox.exe",
        ),
        (
            "Firefox",
            r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe",
        ),
        (
            "Brave",
            r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
        ),
    ];
    let mut found = Vec::new();
    for (name, path) in candidates {
        if Path::new(path).is_file() && !found.iter().any(|n| n == name) {
            found.push((*name).to_string());
        }
    }
    found
}

/// In-process browser port for tests.
#[derive(Debug, Default)]
pub struct MemoryBrowserPort {
    opened: Mutex<Vec<String>>,
    focused: Mutex<Option<String>>,
}

impl MemoryBrowserPort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BrowserPort for MemoryBrowserPort {
    fn status(&self) -> Result<BrowserCapabilityStatus> {
        Ok(BrowserCapabilityStatus {
            available: true,
            default_handler: "memory".into(),
            browsers: vec!["Chrome".into(), "Edge".into()],
            message: "Browser support is available.".into(),
        })
    }

    fn open_url(&self, url: &str) -> Result<BrowserOperationOutcome> {
        let url = url.trim();
        if url.is_empty() {
            return Err(WindowsIntegrationError::BrowserFailed(
                "URL is required".into(),
            ));
        }
        self.opened
            .lock()
            .map_err(|_| WindowsIntegrationError::BrowserFailed("browser lock poisoned".into()))?
            .push(url.to_string());
        Ok(BrowserOperationOutcome {
            ok: true,
            target: Some(url.to_string()),
            message: format!("Opened {url}."),
        })
    }

    fn focus(&self, query: &str) -> Result<BrowserOperationOutcome> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(BrowserOperationOutcome {
                ok: false,
                target: None,
                message: "Which browser should I bring forward?".into(),
            });
        }
        *self
            .focused
            .lock()
            .map_err(|_| WindowsIntegrationError::BrowserFailed("browser lock poisoned".into()))? =
            Some(q.to_string());
        Ok(BrowserOperationOutcome {
            ok: true,
            target: Some(q.to_string()),
            message: format!("Focused “{q}”."),
        })
    }
}

/// Production WRAP of the `webbrowser` crate + path detection.
pub struct SystemBrowserPort;

impl SystemBrowserPort {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SystemBrowserPort {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserPort for SystemBrowserPort {
    fn status(&self) -> Result<BrowserCapabilityStatus> {
        let browsers = detect_installed_browsers();
        let available = true;
        let default_handler = if browsers.is_empty() {
            "system-default".into()
        } else {
            browsers[0].clone()
        };
        Ok(BrowserCapabilityStatus {
            available,
            default_handler,
            browsers,
            message: "Browser support is available on this PC.".into(),
        })
    }

    fn open_url(&self, url: &str) -> Result<BrowserOperationOutcome> {
        let url = url.trim();
        if url.is_empty() {
            return Err(WindowsIntegrationError::BrowserFailed(
                "URL is required".into(),
            ));
        }
        webbrowser::open(url).map_err(|error| {
            WindowsIntegrationError::BrowserFailed(format!("open URL: {error}"))
        })?;
        Ok(BrowserOperationOutcome {
            ok: true,
            target: Some(url.to_string()),
            message: "Opened in your browser.".into(),
        })
    }

    fn focus(&self, query: &str) -> Result<BrowserOperationOutcome> {
        // Focus is fulfilled by Kernel Operator composing Window Provider for L2.
        // Port reports intent for audit; truthful when no window layer is attached.
        let q = query.trim();
        if q.is_empty() {
            return Ok(BrowserOperationOutcome {
                ok: false,
                target: None,
                message: "Which browser should I bring forward?".into(),
            });
        }
        // Actual HWND focus is Operator → Window Provider composition.
        Ok(BrowserOperationOutcome {
            ok: true,
            target: Some(q.to_string()),
            message: format!("Looking for “{q}” to bring forward."),
        })
    }
}

/// Unavailable stub (rare).
#[derive(Debug, Default)]
pub struct UnavailableBrowserPort;

impl BrowserPort for UnavailableBrowserPort {
    fn status(&self) -> Result<BrowserCapabilityStatus> {
        Ok(BrowserCapabilityStatus {
            available: false,
            default_handler: "unavailable".into(),
            browsers: vec![],
            message: "Browser support isn’t available on this system.".into(),
        })
    }

    fn open_url(&self, _url: &str) -> Result<BrowserOperationOutcome> {
        Err(WindowsIntegrationError::BrowserFailed(
            "browser support isn’t available on this system".into(),
        ))
    }

    fn focus(&self, _query: &str) -> Result<BrowserOperationOutcome> {
        Ok(BrowserOperationOutcome {
            ok: false,
            target: None,
            message: "Browser support isn’t available on this system.".into(),
        })
    }
}

pub fn platform_browser() -> std::sync::Arc<dyn BrowserPort> {
    std::sync::Arc::new(SystemBrowserPort::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_browser_open_roundtrip() {
        let port = MemoryBrowserPort::new();
        assert!(port.status().unwrap().available);
        let opened = port.open_url("https://example.com").unwrap();
        assert!(opened.ok);
        assert_eq!(opened.target.as_deref(), Some("https://example.com"));
    }
}
