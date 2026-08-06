//! Declared desktop mutation types and plan resolution (PP-M1-02 / ADM).
//!
//! Action executes only types declared here. It never reads a saved-context
//! identifier and never retains plan or environment state after a request.

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ts_rs::TS;
use uuid::Uuid;

use crate::saved_context::{
    title_fingerprint, SavedContextRestoreIdentity, RESTORE_IDENTITY_SCHEMA_VERSION,
};

pub const ACTION_TYPE_WINDOW_PLACE: &str = "window.place";
pub const ACTION_TYPE_WINDOW_FOCUS: &str = "window.focus";
pub const ACTION_TYPE_WINDOW_Z_ORDER: &str = "window.z_order";
pub const ACTION_TYPE_APPLICATION_LAUNCH: &str = "application.launch";

pub const SCOPE_PLAN_RESOLVE: &str = "action.plan.resolve";
pub const SCOPE_WINDOW_PLACE: &str = "action.window.place";
pub const SCOPE_WINDOW_FOCUS: &str = "action.window.focus";

pub const MATCH_CLASS_EXACT_SESSION: &str = "exact_session";

/// Bounded plan validity (privacy constraint — not a convenience timeout).
pub const ACTION_PLAN_TTL_SECS: i64 = 120;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ProjectedDisposition {
    WillAttempt,
    WillSkipUnsupported,
    WillSkipUnresolvable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ItemDisposition {
    Completed,
    Failed,
    SkippedUnsupported,
    SkippedUnresolvable,
    RefusedChanged,
    NotAttempted,
    OutcomeUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum OperationOutcome {
    Completed,
    PartiallyCompleted,
    Failed,
    Cancelled,
    Indeterminate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProposedEffect {
    Place {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        monitor_index: Option<i32>,
        minimized: bool,
    },
    Focus,
    Unsupported {
        intent: String,
    },
}

impl ProposedEffect {
    pub fn place(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        monitor_index: Option<i32>,
        minimized: bool,
    ) -> Self {
        Self::Place {
            x,
            y,
            width,
            height,
            monitor_index,
            minimized,
        }
    }
}

/// Declared target for one Action item. Contains identity evidence, never a
/// saved-context identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ActionTargetDescriptor {
    pub item_id: String,
    pub action_type: String,
    pub target_summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_identity: Option<SavedContextRestoreIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_unavailable_reason: Option<String>,
    pub proposed_effect: ProposedEffect,
    /// Callers must not supply a threshold. Presence is contract-invalid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<String>,
    /// Forbidden. Presence is contract-invalid (ADM-AC-27).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_context_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionRequest {
    pub purpose: String,
    pub items: Vec<ActionTargetDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ActionPlanItem {
    pub item_id: String,
    pub action_type: String,
    pub target_summary: String,
    pub proposed_effect: ProposedEffect,
    pub permission_scope: String,
    pub projected_disposition: ProjectedDisposition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Exact target descriptor used for digest binding and re-resolution.
    pub target: ActionTargetDescriptor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ActionPlan {
    pub plan_id: String,
    pub expires_at: String,
    pub plan_digest: String,
    pub purpose: String,
    pub items: Vec<ActionPlanItem>,
}

impl ActionPlan {
    pub fn is_expired_at(&self, now: chrono::DateTime<Utc>) -> bool {
        match chrono::DateTime::parse_from_rfc3339(&self.expires_at) {
            Ok(expires) => now >= expires.with_timezone(&Utc),
            Err(_) => true,
        }
    }
}

/// Canonical restore compatibility / confidence summary derived from a resolved plan.
///
/// Experience synthesizes the same bands from disposition ratios; this is the
/// runtime authority so Save→Continue pipelines and tests share one score.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RestoreCompatibilitySummary {
    pub total_items: u32,
    pub will_attempt: u32,
    pub will_skip_unsupported: u32,
    pub will_skip_unresolvable: u32,
    /// Items skipped because the exact-session window was not found (closed apps).
    pub missing_window_count: u32,
    /// `high` | `steady` | `limited` | `empty` — matches frozen Continue quality copy.
    pub confidence_band: String,
    /// True when at least one place/focus item will be attempted.
    pub restore_eligible: bool,
}

impl RestoreCompatibilitySummary {
    pub fn from_plan(plan: &ActionPlan) -> Self {
        let total_items = plan.items.len() as u32;
        let mut will_attempt = 0u32;
        let mut will_skip_unsupported = 0u32;
        let mut will_skip_unresolvable = 0u32;
        let mut missing_window_count = 0u32;
        for item in &plan.items {
            match item.projected_disposition {
                ProjectedDisposition::WillAttempt => will_attempt += 1,
                ProjectedDisposition::WillSkipUnsupported => will_skip_unsupported += 1,
                ProjectedDisposition::WillSkipUnresolvable => {
                    will_skip_unresolvable += 1;
                    if item.error_code.as_deref() == Some("ACTION_TARGET_NOT_FOUND") {
                        missing_window_count += 1;
                    }
                }
            }
        }
        let ratio = if total_items == 0 {
            0.0
        } else {
            f64::from(will_attempt) / f64::from(total_items)
        };
        let confidence_band = if total_items == 0 {
            "empty"
        } else if ratio >= 0.85 {
            "high"
        } else if ratio >= 0.5 {
            "steady"
        } else {
            "limited"
        }
        .to_string();
        Self {
            total_items,
            will_attempt,
            will_skip_unsupported,
            will_skip_unresolvable,
            missing_window_count,
            confidence_band,
            restore_eligible: will_attempt > 0,
        }
    }
}

/// One effect proof bound to a will_attempt item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemEffectProof {
    pub plan_digest: String,
    pub item_id: String,
    pub action_type: String,
    pub permission_scope: String,
    pub purpose: String,
    pub requester: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ActionItemOutcome {
    pub item_id: String,
    pub action_type: String,
    pub target_summary: String,
    pub disposition: ItemDisposition,
    pub what: String,
    pub why: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    pub user_action_available: String,
}

/// Aggregated restore execution facts for Continue / operators.
///
/// Derived from per-item dispositions so partial success is never discarded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct RestoreExecutionSummary {
    /// Effect items that completed (place and/or focus).
    pub restored_windows: u32,
    /// Skipped unsupported, unresolvable, or not attempted.
    pub skipped_windows: u32,
    /// Closed / absent windows (`ACTION_TARGET_NOT_FOUND`).
    pub missing_applications: u32,
    /// Failed, refused-changed, or outcome-unknown items.
    pub failed_operations: u32,
    /// Wall-clock duration of the execute pass.
    #[ts(type = "number")]
    pub duration_ms: u64,
}

impl RestoreExecutionSummary {
    pub fn from_items(items: &[ActionItemOutcome], duration_ms: u64) -> Self {
        let mut restored_windows = 0u32;
        let mut skipped_windows = 0u32;
        let mut missing_applications = 0u32;
        let mut failed_operations = 0u32;
        for item in items {
            match item.disposition {
                ItemDisposition::Completed => restored_windows += 1,
                ItemDisposition::SkippedUnsupported
                | ItemDisposition::SkippedUnresolvable
                | ItemDisposition::NotAttempted => {
                    skipped_windows += 1;
                    if item.error_code.as_deref() == Some("ACTION_TARGET_NOT_FOUND") {
                        missing_applications += 1;
                    }
                }
                ItemDisposition::Failed
                | ItemDisposition::RefusedChanged
                | ItemDisposition::OutcomeUnknown => failed_operations += 1,
            }
        }
        Self {
            restored_windows,
            skipped_windows,
            missing_applications,
            failed_operations,
            duration_ms,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ActionOperationResult {
    pub operation_id: String,
    pub outcome: OperationOutcome,
    pub items: Vec<ActionItemOutcome>,
    pub summary: RestoreExecutionSummary,
}

/// Preview payload returned to Experience for Continue (PP-M1-02).
///
/// Owned by domain as a Product Proof wire contract. Kernel commands construct
/// this value; the React shell consumes it via IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
pub struct ResumePlanPreview {
    pub saved_context_id: String,
    pub saved_context_name: String,
    /// User-authored intended next action (PP-P01A). Not an Action effect.
    pub handoff_note: String,
    pub plan: ActionPlan,
    /// Compatibility / confidence derived from plan dispositions.
    pub compatibility: RestoreCompatibilitySummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DesktopActionError {
    #[error("ACTION_REQUEST_EMPTY: the action request has no items")]
    RequestEmpty,

    #[error("ACTION_TYPE_NOT_DECLARED: '{0}' is not a declared action type")]
    TypeNotDeclared(String),

    #[error("ACTION_PLAN_UNKNOWN: the plan is expired, malformed, or digest-mismatched")]
    PlanUnknown,

    #[error("ACTION_CONTRACT_INVALID: {0}")]
    ContractInvalid(String),

    #[error("ACTION_PERMISSION_DENIED: {0}")]
    PermissionDenied(String),
}

/// Stable digest over the complete ordered plan (ADM §4).
pub fn compute_plan_digest(
    plan_id: &str,
    expires_at: &str,
    purpose: &str,
    items: &[ActionPlanItem],
) -> String {
    let mut material = String::new();
    material.push_str(plan_id);
    material.push('\n');
    material.push_str(expires_at);
    material.push('\n');
    material.push_str(purpose);
    material.push('\n');
    for item in items {
        material.push_str(&item.item_id);
        material.push('|');
        material.push_str(&item.action_type);
        material.push('|');
        material.push_str(&serde_json::to_string(&item.target).unwrap_or_default());
        material.push('|');
        material.push_str(&serde_json::to_string(&item.proposed_effect).unwrap_or_default());
        material.push('|');
        material.push_str(&item.permission_scope);
        material.push('|');
        material.push_str(match item.projected_disposition {
            ProjectedDisposition::WillAttempt => "will_attempt",
            ProjectedDisposition::WillSkipUnsupported => "will_skip_unsupported",
            ProjectedDisposition::WillSkipUnresolvable => "will_skip_unresolvable",
        });
        material.push('|');
        material.push_str(item.reason.as_deref().unwrap_or(""));
        material.push('|');
        material.push_str(item.error_code.as_deref().unwrap_or(""));
        material.push('\n');
    }
    format!("fnv1a64:{:016x}", fnv1a64(material.as_bytes()))
}

pub fn new_plan_expiry(now: chrono::DateTime<Utc>) -> String {
    (now + Duration::seconds(ACTION_PLAN_TTL_SECS)).to_rfc3339()
}

pub fn new_plan_id() -> String {
    format!("ap-{}", Uuid::new_v4())
}

pub fn new_operation_id() -> String {
    format!("ao-{}", Uuid::new_v4())
}

pub fn permission_scope_for(action_type: &str) -> Option<&'static str> {
    match action_type {
        ACTION_TYPE_WINDOW_PLACE => Some(SCOPE_WINDOW_PLACE),
        ACTION_TYPE_WINDOW_FOCUS => Some(SCOPE_WINDOW_FOCUS),
        _ => None,
    }
}

pub fn is_declared_action_type(action_type: &str) -> bool {
    matches!(
        action_type,
        ACTION_TYPE_WINDOW_PLACE | ACTION_TYPE_WINDOW_FOCUS
    )
}

pub fn is_reserved_action_type(action_type: &str) -> bool {
    matches!(
        action_type,
        ACTION_TYPE_APPLICATION_LAUNCH
            | "application.reuse"
            | "resource.openFile"
            | "resource.openUrl"
            | "workspace.activate"
            | ACTION_TYPE_WINDOW_Z_ORDER
    )
}

/// Live window facts Action may compare during bounded matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveWindowIdentity {
    pub hwnd: String,
    pub process_id: i32,
    pub title: String,
}

impl LiveWindowIdentity {
    pub fn title_fingerprint(&self) -> String {
        title_fingerprint(&self.title)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult {
    ExactSessionMatch,
    NoMatch,
    Ambiguous,
    IdentityUnavailable { reason: String },
    IdentityVersionUnsupported,
    NotPortable,
    ConfidenceInsufficient,
    PlacementUnsatisfiable { reason: String },
}

impl MatchResult {
    pub fn error_code(&self) -> Option<&'static str> {
        match self {
            Self::ExactSessionMatch => None,
            Self::NoMatch => Some("ACTION_TARGET_NOT_FOUND"),
            Self::Ambiguous => Some("ACTION_TARGET_AMBIGUOUS"),
            Self::IdentityUnavailable { .. } => Some("ACTION_TARGET_IDENTITY_UNAVAILABLE"),
            Self::IdentityVersionUnsupported => {
                Some("ACTION_TARGET_IDENTITY_VERSION_UNSUPPORTED")
            }
            Self::NotPortable => Some("ACTION_TARGET_CONFIDENCE_INSUFFICIENT"),
            Self::ConfidenceInsufficient => Some("ACTION_TARGET_CONFIDENCE_INSUFFICIENT"),
            Self::PlacementUnsatisfiable { .. } => Some("ACTION_PLACEMENT_UNSATISFIABLE"),
        }
    }

    pub fn safe_reason(&self) -> String {
        match self {
            Self::ExactSessionMatch => String::new(),
            Self::NoMatch => {
                "That window is no longer open in this desktop session. Nothing was moved for it."
                    .into()
            }
            Self::Ambiguous => {
                "More than one window matched the saved identity, so nothing was moved."
                    .into()
            }
            Self::IdentityUnavailable { reason } => format!(
                "This window cannot be restored ({reason}). Workspace did not retry or search in the background."
            ),
            Self::IdentityVersionUnsupported => {
                "This window uses an unsupported restore identity version. Nothing was moved."
                    .into()
            }
            Self::NotPortable => {
                "Restore is limited to the same continuing Windows desktop session. Nothing was moved."
                    .into()
            }
            Self::ConfidenceInsufficient => {
                "The window could not be matched exactly, so nothing was moved."
                    .into()
            }
            Self::PlacementUnsatisfiable { reason } => format!(
                "The saved placement cannot be honoured on the current monitors ({reason}). Nothing was moved."
            ),
        }
    }
}

/// Exact-session matcher (Saved Context Restore Identity Spec §6).
pub fn match_exact_session(
    identity: &SavedContextRestoreIdentity,
    current_session: &str,
    candidates_with_hwnd: &[LiveWindowIdentity],
    monitor_indices: &[i32],
    proposed: &ProposedEffect,
) -> MatchResult {
    if identity.identity_schema_version != RESTORE_IDENTITY_SCHEMA_VERSION {
        return MatchResult::IdentityVersionUnsupported;
    }
    if !identity.is_complete() {
        return MatchResult::IdentityUnavailable {
            reason: "incomplete restore identity".into(),
        };
    }
    if current_session != identity.desktop_session_id {
        return MatchResult::NotPortable;
    }

    let matches: Vec<_> = candidates_with_hwnd
        .iter()
        .filter(|live| live.hwnd == identity.captured_hwnd)
        .collect();

    match matches.len() {
        0 => MatchResult::NoMatch,
        1 => {
            let live = matches[0];
            if live.process_id != identity.captured_process_id {
                return MatchResult::ConfidenceInsufficient;
            }
            if live.title_fingerprint() != identity.title_fingerprint {
                return MatchResult::ConfidenceInsufficient;
            }
            if let ProposedEffect::Place {
                monitor_index: Some(index),
                ..
            } = proposed
            {
                if !monitor_indices.contains(index) {
                    return MatchResult::PlacementUnsatisfiable {
                        reason: format!("monitor {index} is not attached"),
                    };
                }
            }
            MatchResult::ExactSessionMatch
        }
        _ => MatchResult::Ambiguous,
    }
}

pub fn operation_outcome_from_items(items: &[ActionItemOutcome]) -> OperationOutcome {
    let any_unknown = items
        .iter()
        .any(|item| item.disposition == ItemDisposition::OutcomeUnknown);
    if any_unknown {
        return OperationOutcome::Indeterminate;
    }
    let completed = items
        .iter()
        .filter(|item| item.disposition == ItemDisposition::Completed)
        .count();
    let total = items.len();
    if completed == total && total > 0 {
        OperationOutcome::Completed
    } else if completed > 0 {
        OperationOutcome::PartiallyCompleted
    } else {
        OperationOutcome::Failed
    }
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::saved_context::RESTORE_IDENTITY_SCHEMA_VERSION;

    fn identity() -> SavedContextRestoreIdentity {
        SavedContextRestoreIdentity::new("session-1", "0xAA", 100, "Fixture Focus", "t")
    }

    #[test]
    fn exact_session_match_requires_all_fields() {
        let live = LiveWindowIdentity {
            hwnd: "0xAA".into(),
            process_id: 100,
            title: "Fixture Focus".into(),
        };
        let result = match_exact_session(
            &identity(),
            "session-1",
            &[live],
            &[0],
            &ProposedEffect::place(1, 2, 3, 4, Some(0), false),
        );
        assert_eq!(result, MatchResult::ExactSessionMatch);
    }

    #[test]
    fn handle_reuse_with_different_title_fails_closed() {
        let live = LiveWindowIdentity {
            hwnd: "0xAA".into(),
            process_id: 100,
            title: "Other Title".into(),
        };
        let result = match_exact_session(
            &identity(),
            "session-1",
            &[live],
            &[0],
            &ProposedEffect::Focus,
        );
        assert_eq!(result, MatchResult::ConfidenceInsufficient);
    }

    #[test]
    fn different_session_is_not_portable() {
        let live = LiveWindowIdentity {
            hwnd: "0xAA".into(),
            process_id: 100,
            title: "Fixture Focus".into(),
        };
        let result = match_exact_session(
            &identity(),
            "session-other",
            &[live],
            &[0],
            &ProposedEffect::Focus,
        );
        assert_eq!(result, MatchResult::NotPortable);
    }

    #[test]
    fn unsupported_identity_version_fails_closed() {
        let mut id = identity();
        id.identity_schema_version = "99".into();
        assert_ne!(id.identity_schema_version, RESTORE_IDENTITY_SCHEMA_VERSION);
        let result = match_exact_session(&id, "session-1", &[], &[0], &ProposedEffect::Focus);
        assert_eq!(result, MatchResult::IdentityVersionUnsupported);
    }

    #[test]
    fn digest_changes_when_item_changes() {
        let item = ActionPlanItem {
            item_id: "i1".into(),
            action_type: ACTION_TYPE_WINDOW_FOCUS.into(),
            target_summary: "A".into(),
            proposed_effect: ProposedEffect::Focus,
            permission_scope: SCOPE_WINDOW_FOCUS.into(),
            projected_disposition: ProjectedDisposition::WillAttempt,
            reason: None,
            error_code: None,
            target: ActionTargetDescriptor {
                item_id: "i1".into(),
                action_type: ACTION_TYPE_WINDOW_FOCUS.into(),
                target_summary: "A".into(),
                restore_identity: Some(identity()),
                identity_unavailable_reason: None,
                proposed_effect: ProposedEffect::Focus,
                confidence_threshold: None,
                saved_context_id: None,
            },
        };
        let d1 = compute_plan_digest("p", "e", "why", &[item.clone()]);
        let mut changed = item;
        changed.target.target_summary = "B".into();
        changed.target_summary = "B".into();
        let d2 = compute_plan_digest("p", "e", "why", &[changed]);
        assert_ne!(d1, d2);
    }
}
