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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Provider-owned operation (not a standalone feature).
///
/// Providers own operations. Clipboard owns Read/Write; Application owns
/// Launch/Enumerate/Focus/Close/Minimize/Restore/Find.
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
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "read" => Some(Self::Read),
            "write" => Some(Self::Write),
            "launch" => Some(Self::Launch),
            "enumerate" | "list" => Some(Self::Enumerate),
            "focus" | "switch" => Some(Self::Focus),
            "close" | "quit" => Some(Self::Close),
            "minimize" => Some(Self::Minimize),
            "restore" => Some(Self::Restore),
            "find" | "locate" => Some(Self::Find),
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
    /// Clipboard text, or free-form payload when needed.
    pub text: Option<String>,
    /// Application / window query (name, title fragment).
    pub query: Option<String>,
    /// Explicit executable path for launch.
    pub path: Option<String>,
    /// Window handle hex string when already resolved.
    pub hwnd: Option<String>,
    pub pid: Option<u32>,
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

/// One enumerated / found application window (Conversation-safe).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationWindowItem {
    pub hwnd: String,
    pub title: String,
    pub process_id: u32,
    pub minimized: bool,
    pub focused: bool,
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
    /// Full text for Conversation reply construction — never written to audit as-is.
    pub text: Option<String>,
    pub preview: Option<String>,
    pub message: Option<String>,
    /// Operation result status (launched|focused|closed|not_found|…).
    pub status: Option<String>,
    pub target: Option<String>,
    pub items: Option<Vec<ApplicationWindowItem>>,
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
