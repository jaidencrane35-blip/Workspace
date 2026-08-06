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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Provider-level operation (mapped from Intent / Command).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityOperation {
    Read,
    Write,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderInvokeRequest {
    pub domain: CapabilityDomainId,
    pub operation: CapabilityOperation,
    pub text: Option<String>,
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
