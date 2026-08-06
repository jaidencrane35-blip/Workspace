use serde::{Deserialize, Serialize};

use crate::capability_runtime::{ApplicationWindowItem, MonitorItem};

/// Shared Capability Intent envelope — Conversation/Intent Layer → Kernel Operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityIntent {
    /// clipboard | application | window
    pub domain: String,
    /// Provider op or high-level composition key (e.g. "open").
    pub operation: String,
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
    pub snap: Option<String>,
}

/// Truthful Operator turn result for Conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatorTurnResult {
    pub ok: bool,
    pub message: String,
    pub status: Option<String>,
    pub domain: String,
    pub operation: String,
    pub target: Option<String>,
    pub preview: Option<String>,
    pub text: Option<String>,
    pub items: Option<Vec<ApplicationWindowItem>>,
    pub monitors: Option<Vec<MonitorItem>>,
    pub composition_id: Option<String>,
}
