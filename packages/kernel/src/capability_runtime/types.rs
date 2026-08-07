use serde::{Deserialize, Serialize};

/// Stable capability domain identifier (one provider per domain).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityDomainId(String);

impl CapabilityDomainId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn clipboard() -> Self {
        Self::new("clipboard")
    }

    pub fn application() -> Self {
        Self::new("application")
    }

    pub fn window() -> Self {
        Self::new("window")
    }

    pub fn notifications() -> Self {
        Self::new("notifications")
    }

    pub fn browser() -> Self {
        Self::new("browser")
    }

    pub fn screenshots() -> Self {
        Self::new("screenshots")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Provider-owned operation (not a standalone feature).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityOperation {
    Read,
    Write,
    Launch,
    Enumerate,
    Focus,
    Close,
    Minimize,
    Restore,
    Find,
    Maximize,
    Move,
    Resize,
    Center,
    Snap,
    Bounds,
    Active,
    Monitors,
    /// Notifications Provider — Level 1 capability query.
    Status,
    /// Notifications Provider — Level 2 show toast.
    Show,
    /// Notifications Provider — Level 2 dismiss (best-effort).
    Dismiss,
    /// Browser Provider — Level 2 open URL / site.
    Open,
    /// Screenshot Provider — capture primary / desktop.
    CaptureDesktop,
    /// Screenshot Provider — capture named / active window.
    CaptureWindow,
    /// Screenshot Provider — capture monitor by 1-based index.
    CaptureMonitor,
    /// Screenshot Provider — ensure PNG save (alias of capture when path empty).
    SavePng,
    /// Screenshot Provider — copy last / path PNG to clipboard.
    CopyClipboard,
}

impl CapabilityOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Launch => "launch",
            Self::Enumerate => "enumerate",
            Self::Focus => "focus",
            Self::Close => "close",
            Self::Minimize => "minimize",
            Self::Restore => "restore",
            Self::Find => "find",
            Self::Maximize => "maximize",
            Self::Move => "move",
            Self::Resize => "resize",
            Self::Center => "center",
            Self::Snap => "snap",
            Self::Bounds => "bounds",
            Self::Active => "active",
            Self::Monitors => "monitors",
            Self::Status => "status",
            Self::Show => "show",
            Self::Dismiss => "dismiss",
            Self::Open => "open",
            Self::CaptureDesktop => "capture_desktop",
            Self::CaptureWindow => "capture_window",
            Self::CaptureMonitor => "capture_monitor",
            Self::SavePng => "save_png",
            Self::CopyClipboard => "copy_clipboard",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "read" => Some(Self::Read),
            "write" => Some(Self::Write),
            "launch" => Some(Self::Launch),
            "enumerate" | "list" => Some(Self::Enumerate),
            "focus" | "switch" | "activate" => Some(Self::Focus),
            "close" | "quit" => Some(Self::Close),
            "minimize" => Some(Self::Minimize),
            "restore" => Some(Self::Restore),
            "find" | "locate" => Some(Self::Find),
            "maximize" => Some(Self::Maximize),
            "move" => Some(Self::Move),
            "resize" => Some(Self::Resize),
            "center" => Some(Self::Center),
            "snap" => Some(Self::Snap),
            "bounds" | "info" => Some(Self::Bounds),
            "active" | "foreground" => Some(Self::Active),
            "monitors" | "displays" => Some(Self::Monitors),
            "status" | "available" | "capability" => Some(Self::Status),
            "show" | "notify" | "toast" => Some(Self::Show),
            "dismiss" | "clear" | "hide" => Some(Self::Dismiss),
            "open" | "browse" | "visit" => Some(Self::Open),
            "capture_desktop" | "capture_screen" | "screenshot_desktop" | "capture" => {
                Some(Self::CaptureDesktop)
            }
            "capture_window" | "screenshot_window" => Some(Self::CaptureWindow),
            "capture_monitor" | "screenshot_monitor" => Some(Self::CaptureMonitor),
            "save_png" | "save_screenshot" => Some(Self::SavePng),
            "copy_clipboard" | "copy_screenshot" => Some(Self::CopyClipboard),
            _ => None,
        }
    }
}

/// Static provider contract surface (registry listing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDescriptor {
    pub name: String,
    pub domain: CapabilityDomainId,
    pub purpose: String,
    pub operations: Vec<&'static str>,
    pub adoption: &'static str,
}

/// Request into the Capability Router → Provider Registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInvokeRequest {
    pub domain: CapabilityDomainId,
    pub operation: CapabilityOperation,
    pub text: Option<String>,
    pub query: Option<String>,
    pub path: Option<String>,
    pub hwnd: Option<String>,
    pub pid: Option<u32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub monitor_index: Option<i32>,
    /// Snap edge: left | right | top | bottom
    pub snap: Option<String>,
    /// Notification title (Notifications Provider).
    pub title: Option<String>,
    /// Notification category label.
    pub category: Option<String>,
    /// Notification priority: normal | high | low
    pub priority: Option<String>,
    /// Notification duration: short | long
    pub duration: Option<String>,
}

impl Default for CapabilityDomainId {
    fn default() -> Self {
        Self::new("")
    }
}

impl Default for CapabilityOperation {
    fn default() -> Self {
        Self::Read
    }
}

/// One enumerated / found window (Conversation-safe).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationWindowItem {
    pub hwnd: String,
    pub title: String,
    pub process_id: u32,
    pub minimized: bool,
    pub focused: bool,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_index: Option<i32>,
}

/// Monitor descriptor for Window Provider Level 1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorItem {
    pub index: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: i32,
    pub work_height: i32,
    pub is_primary: bool,
}

/// Response from a Capability Provider (Desktop Service effect complete).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInvokeResponse {
    pub domain: CapabilityDomainId,
    pub operation: CapabilityOperation,
    pub ok: bool,
    pub format: Option<String>,
    pub bytes: Option<usize>,
    pub text: Option<String>,
    pub preview: Option<String>,
    pub message: Option<String>,
    pub status: Option<String>,
    pub target: Option<String>,
    pub items: Option<Vec<ApplicationWindowItem>>,
    pub monitors: Option<Vec<MonitorItem>>,
}

impl ProviderInvokeResponse {
    pub fn summary(&self) -> ProviderResultSummary {
        ProviderResultSummary {
            domain: self.domain.clone(),
            operation: self.operation,
            ok: self.ok,
            format: self.format.clone(),
            bytes: self.bytes,
            preview: self.preview.clone(),
            message: self.message.clone(),
            status: self.status.clone(),
            target: self.target.clone(),
            item_count: self.items.as_ref().map(|items| items.len()),
        }
    }
}

/// Audit-safe summary (no full secret payload).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResultSummary {
    pub domain: CapabilityDomainId,
    pub operation: CapabilityOperation,
    pub ok: bool,
    pub format: Option<String>,
    pub bytes: Option<usize>,
    pub preview: Option<String>,
    pub message: Option<String>,
    pub status: Option<String>,
    pub target: Option<String>,
    pub item_count: Option<usize>,
}

pub(crate) fn text_preview(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let preview: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}
