//! Notification port — Workspace-owned interface; WRAP WinRT toast APIs (P13).
//!
//! Product identity never leaks WinRT / crate names into Conversation.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use crate::error::{Result, WindowsIntegrationError};

/// Request to show a desktop notification.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NotificationShowRequest {
    pub title: String,
    pub body: String,
    pub category: Option<String>,
    pub priority: Option<String>,
    pub duration: Option<String>,
}

/// Result of showing a notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationShowOutcome {
    pub id: String,
    pub shown: bool,
    pub message: String,
}

/// Provider / OS notification capability snapshot (Level 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationCapabilityStatus {
    pub available: bool,
    pub platform: String,
    pub permission: String,
    pub message: String,
}

/// Desktop notification access behind a Workspace-owned port.
pub trait NotificationPort: Send + Sync {
    fn status(&self) -> Result<NotificationCapabilityStatus>;
    fn show(&self, request: &NotificationShowRequest) -> Result<NotificationShowOutcome>;
    fn dismiss(&self, id: &str) -> Result<NotificationShowOutcome>;
}

/// In-process notifications for tests and non-Windows bootstrap.
#[derive(Debug, Default)]
pub struct MemoryNotificationPort {
    next_id: AtomicU64,
    active: Mutex<Vec<String>>,
}

impl MemoryNotificationPort {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NotificationPort for MemoryNotificationPort {
    fn status(&self) -> Result<NotificationCapabilityStatus> {
        Ok(NotificationCapabilityStatus {
            available: true,
            platform: "memory".into(),
            permission: "granted".into(),
            message: "Desktop notifications are available.".into(),
        })
    }

    fn show(&self, request: &NotificationShowRequest) -> Result<NotificationShowOutcome> {
        if request.title.trim().is_empty() && request.body.trim().is_empty() {
            return Err(WindowsIntegrationError::NotificationFailed(
                "notification requires a title or message".into(),
            ));
        }
        let id = format!("mem-{}", self.next_id.fetch_add(1, Ordering::Relaxed));
        self.active
            .lock()
            .map_err(|_| {
                WindowsIntegrationError::NotificationFailed("notification lock poisoned".into())
            })?
            .push(id.clone());
        Ok(NotificationShowOutcome {
            id,
            shown: true,
            message: "Notification shown.".into(),
        })
    }

    fn dismiss(&self, id: &str) -> Result<NotificationShowOutcome> {
        let mut active = self.active.lock().map_err(|_| {
            WindowsIntegrationError::NotificationFailed("notification lock poisoned".into())
        })?;
        let before = active.len();
        active.retain(|entry| entry != id);
        if active.len() == before {
            return Ok(NotificationShowOutcome {
                id: id.into(),
                shown: false,
                message: "No matching notification to dismiss.".into(),
            });
        }
        Ok(NotificationShowOutcome {
            id: id.into(),
            shown: false,
            message: "Notification dismissed.".into(),
        })
    }
}

/// Production WRAP of WinRT toast notifications (Windows only).
#[cfg(windows)]
pub struct WinRtNotificationPort;

#[cfg(windows)]
impl WinRtNotificationPort {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(windows)]
impl Default for WinRtNotificationPort {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl NotificationPort for WinRtNotificationPort {
    fn status(&self) -> Result<NotificationCapabilityStatus> {
        Ok(NotificationCapabilityStatus {
            available: true,
            platform: "winrt-toast".into(),
            permission: "granted".into(),
            message: "Desktop notifications are available on this PC.".into(),
        })
    }

    fn show(&self, request: &NotificationShowRequest) -> Result<NotificationShowOutcome> {
        use tauri_winrt_notification::{Duration, Toast};

        let title = if request.title.trim().is_empty() {
            "Workspace"
        } else {
            request.title.trim()
        };
        let body = request.body.trim();
        if body.is_empty() && request.title.trim().is_empty() {
            return Err(WindowsIntegrationError::NotificationFailed(
                "notification requires a title or message".into(),
            ));
        }

        let duration = match request
            .duration
            .as_deref()
            .unwrap_or("short")
            .trim()
            .to_ascii_lowercase()
            .as_str()
        {
            "long" => Duration::Long,
            _ => Duration::Short,
        };

        // Unpackaged Tauri builds lack a registered AUMID; PowerShell AppId is the
        // reliable WRAP path. Toast may attribute to PowerShell until packaging lands.
        let toast = Toast::new(Toast::POWERSHELL_APP_ID)
            .title(title)
            .text1(if body.is_empty() { title } else { body })
            .duration(duration);

        toast.show().map_err(|error| {
            WindowsIntegrationError::NotificationFailed(format!("show notification: {error}"))
        })?;

        let id = format!(
            "toast-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0)
        );

        Ok(NotificationShowOutcome {
            id,
            shown: true,
            message: "Desktop notification shown.".into(),
        })
    }

    fn dismiss(&self, id: &str) -> Result<NotificationShowOutcome> {
        // WinRT toast history dismissal is not reliably available for unpackaged apps.
        Ok(NotificationShowOutcome {
            id: id.into(),
            shown: false,
            message: "I can’t dismiss that toast from here on this PC.".into(),
        })
    }
}

/// Stub when notifications are unavailable (non-Windows).
#[derive(Debug, Default)]
pub struct UnavailableNotificationPort;

impl NotificationPort for UnavailableNotificationPort {
    fn status(&self) -> Result<NotificationCapabilityStatus> {
        Ok(NotificationCapabilityStatus {
            available: false,
            platform: "unavailable".into(),
            permission: "unavailable".into(),
            message: "Desktop notifications aren’t available on this system.".into(),
        })
    }

    fn show(&self, _request: &NotificationShowRequest) -> Result<NotificationShowOutcome> {
        Err(WindowsIntegrationError::NotificationFailed(
            "desktop notifications aren’t available on this system".into(),
        ))
    }

    fn dismiss(&self, id: &str) -> Result<NotificationShowOutcome> {
        Ok(NotificationShowOutcome {
            id: id.into(),
            shown: false,
            message: "Desktop notifications aren’t available on this system.".into(),
        })
    }
}

/// Production notification port for the current platform.
pub fn platform_notification() -> std::sync::Arc<dyn NotificationPort> {
    #[cfg(windows)]
    {
        std::sync::Arc::new(WinRtNotificationPort::new())
    }
    #[cfg(not(windows))]
    {
        std::sync::Arc::new(UnavailableNotificationPort)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_notification_show_and_dismiss() {
        let port = MemoryNotificationPort::new();
        assert!(port.status().unwrap().available);
        let shown = port
            .show(&NotificationShowRequest {
                title: "Workspace".into(),
                body: "Done.".into(),
                ..Default::default()
            })
            .unwrap();
        assert!(shown.shown);
        let dismissed = port.dismiss(&shown.id).unwrap();
        assert!(dismissed.message.to_lowercase().contains("dismiss"));
    }

    #[cfg(windows)]
    #[test]
    fn winrt_show_desktop_toast_product_proof() {
        let port = WinRtNotificationPort::new();
        assert!(port.status().unwrap().available);
        let shown = port
            .show(&NotificationShowRequest {
                title: "Workspace".into(),
                body: "P13 Notifications Product Proof.".into(),
                duration: Some("short".into()),
                ..Default::default()
            })
            .expect("WinRT toast should show on Windows");
        assert!(shown.shown);
    }
}
