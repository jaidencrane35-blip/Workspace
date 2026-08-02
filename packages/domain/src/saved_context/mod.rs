//! Bounded, user-authored workspace contexts (Product Proof PP-M1-01 / PP-M1-02).
//!
//! A saved context exists only because a user named it and confirmed a capture
//! scope. Nothing here observes anything: this module declares what a capture
//! may contain, and holds the result once the user has agreed to it.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::{validate_resource_name, DomainError};
use crate::ids::{SavedContextId, WorkspaceId};

/// Identifier for the capture scope this build of Workspace can honour.
///
/// Consent is recorded against this value. If the capture scope ever changes,
/// previously displayed consent no longer describes what would be captured, and
/// the save is refused rather than silently widened.
///
/// v2 adds restore identity fields required for deterministic Resume (PP-M1-02).
pub const SAVED_CONTEXT_SCOPE_ID: &str = "saved-context-scope-v2";

/// Identity schema version for exact-session restore matching.
pub const RESTORE_IDENTITY_SCHEMA_VERSION: &str = "1";

/// Reason recorded for contexts saved before restore identity existed.
pub const RESTORE_IDENTITY_LEGACY_REASON: &str = "saved before restore identity existed";

/// Deterministic v1 title fingerprint (Saved Context Restore Identity Spec §3).
pub fn title_fingerprint(title: &str) -> String {
    title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// One line of the capture scope, written for the person deciding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedContextScopeItem {
    pub key: String,
    pub summary: String,
}

impl SavedContextScopeItem {
    fn new(key: &str, summary: &str) -> Self {
        Self {
            key: key.to_string(),
            summary: summary.to_string(),
        }
    }
}

/// The complete, reviewable description of what saving a context does.
///
/// This is the single source of truth for the pre-capture preview. It is
/// declared next to the type that carries the captured result so the two cannot
/// drift: every `captured` line below corresponds to a field of
/// [`SavedContextWindow`] or [`SavedContextMonitor`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedContextCaptureScope {
    pub id: String,
    pub purpose: String,
    pub captured: Vec<SavedContextScopeItem>,
    pub excluded: Vec<SavedContextScopeItem>,
}

impl SavedContextCaptureScope {
    pub fn current() -> Self {
        Self {
            id: SAVED_CONTEXT_SCOPE_ID.to_string(),
            purpose: "So you can pick up where you left off after an interruption. \
                      This is written to a database file on this computer and nowhere else."
                .to_string(),
            captured: vec![
                SavedContextScopeItem::new(
                    "window_titles",
                    "The title of every open window, exactly as Windows reports it. \
                     A browser or document title often names the page or file you had open, \
                     so that name is saved too.",
                ),
                SavedContextScopeItem::new(
                    "window_process_ids",
                    "The number Windows uses to identify the program each window belongs to.",
                ),
                SavedContextScopeItem::new(
                    "window_placement",
                    "Where each window sat on screen, and how big it was.",
                ),
                SavedContextScopeItem::new(
                    "window_state",
                    "Which windows were minimised, which one you were working in, \
                     and the order they were stacked in.",
                ),
                SavedContextScopeItem::new(
                    "monitor_layout",
                    "How many monitors you have, how they are arranged, and their resolution.",
                ),
                SavedContextScopeItem::new(
                    "restore_identity",
                    "The Windows desktop-session, window, and process identifiers needed to \
                     recognise the same still-open window later on this computer. Restore is \
                     limited to the same continuing Windows desktop session.",
                ),
            ],
            excluded: vec![
                SavedContextScopeItem::new(
                    "screen_contents",
                    "No screenshots. No pixels are read from your screen.",
                ),
                SavedContextScopeItem::new(
                    "document_contents",
                    "Nothing from inside a window — no documents, messages, or page contents.",
                ),
                SavedContextScopeItem::new(
                    "input",
                    "No keystrokes, no mouse movement, no clipboard.",
                ),
                SavedContextScopeItem::new(
                    "credentials",
                    "No passwords, tokens, or account details.",
                ),
                SavedContextScopeItem::new(
                    "program_identity",
                    "No program names and no file paths. Windows gives only the number above.",
                ),
                SavedContextScopeItem::new(
                    "off_device",
                    "Nothing is sent anywhere. No cloud, no account, no telemetry.",
                ),
            ],
        }
    }
}

/// Canonical restore identity for a still-open window (exact-session matching).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedContextRestoreIdentity {
    pub identity_schema_version: String,
    pub desktop_session_id: String,
    pub captured_hwnd: String,
    pub captured_process_id: i32,
    pub title_fingerprint: String,
    pub captured_at: String,
}

impl SavedContextRestoreIdentity {
    pub fn new(
        desktop_session_id: impl Into<String>,
        captured_hwnd: impl Into<String>,
        captured_process_id: i32,
        title: &str,
        captured_at: impl Into<String>,
    ) -> Self {
        Self {
            identity_schema_version: RESTORE_IDENTITY_SCHEMA_VERSION.to_string(),
            desktop_session_id: desktop_session_id.into(),
            captured_hwnd: captured_hwnd.into(),
            captured_process_id,
            title_fingerprint: title_fingerprint(title),
            captured_at: captured_at.into(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.identity_schema_version.trim().is_empty()
            && !self.desktop_session_id.trim().is_empty()
            && !self.captured_hwnd.trim().is_empty()
            && !self.title_fingerprint.trim().is_empty()
            && !self.captured_at.trim().is_empty()
    }
}

/// One window as it stood when the user saved the context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedContextWindow {
    pub id: String,
    pub title: String,
    /// Normalised the same way the observation layer normalises it.
    pub process_id: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub monitor_index: Option<i32>,
    pub minimized: bool,
    pub focused: bool,
    pub z_order: Option<i32>,
    /// Present only when complete restore identity was captured under scope v2+.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_identity: Option<SavedContextRestoreIdentity>,
    /// Present when restore identity is unavailable (legacy or incomplete capture).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_identity_unavailable_reason: Option<String>,
}

impl SavedContextWindow {
    pub fn with_restore_identity(mut self, identity: SavedContextRestoreIdentity) -> Self {
        if identity.is_complete() {
            self.restore_identity = Some(identity);
            self.restore_identity_unavailable_reason = None;
        } else {
            self.restore_identity = None;
            self.restore_identity_unavailable_reason =
                Some("incomplete restore identity at capture".into());
        }
        self
    }

    pub fn with_unavailable_reason(mut self, reason: impl Into<String>) -> Self {
        self.restore_identity = None;
        self.restore_identity_unavailable_reason = Some(reason.into());
        self
    }

    pub fn can_attempt_restore(&self) -> bool {
        self.restore_identity
            .as_ref()
            .is_some_and(SavedContextRestoreIdentity::is_complete)
    }
}

/// One monitor as it stood when the user saved the context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedContextMonitor {
    pub id: String,
    pub monitor_index: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_primary: bool,
}

/// A named bounded workspace context the user deliberately saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedContext {
    pub id: SavedContextId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub created_at: String,
    /// The capture scope the user confirmed before anything was captured.
    pub approved_scope: String,
    /// Provenance of the capture this context was built from.
    pub observation_pass_id: String,
    pub captured_at: String,
    pub windows: Vec<SavedContextWindow>,
    pub monitors: Vec<SavedContextMonitor>,
}

impl SavedContext {
    pub fn validate_name(name: &str) -> Result<(), SavedContextError> {
        validate_resource_name(name).map_err(SavedContextError::Domain)
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn monitor_count(&self) -> usize {
        self.monitors.len()
    }
}

/// What the user asked to save, and what they confirmed while asking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveContextRequest {
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub approved_scope: String,
}

impl SaveContextRequest {
    pub fn new(
        workspace_id: WorkspaceId,
        name: impl Into<String>,
        approved_scope: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id,
            name: name.into(),
            approved_scope: approved_scope.into(),
        }
    }

    /// Rejects anything that was not explicitly and currently consented to.
    pub fn validate(&self) -> Result<(), SavedContextError> {
        if self.name.trim().is_empty() {
            return Err(SavedContextError::NameMissing);
        }
        SavedContext::validate_name(&self.name)?;

        if self.approved_scope.trim().is_empty() {
            return Err(SavedContextError::ConsentMissing);
        }

        if self.approved_scope.trim() != SAVED_CONTEXT_SCOPE_ID {
            return Err(SavedContextError::ConsentStale {
                approved: self.approved_scope.trim().to_string(),
                current: SAVED_CONTEXT_SCOPE_ID.to_string(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum SavedContextError {
    #[error("{0}")]
    Domain(DomainError),

    #[error("give this context a name before saving it")]
    NameMissing,

    #[error("no capture scope was confirmed, so nothing may be captured")]
    ConsentMissing,

    #[error(
        "the confirmed capture scope '{approved}' is not the current scope '{current}'; \
         review what will be saved again before saving"
    )]
    ConsentStale { approved: String, current: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(name: &str, scope: &str) -> SaveContextRequest {
        SaveContextRequest::new(WorkspaceId::new("ws-1").unwrap(), name, scope)
    }

    #[test]
    fn accepts_a_named_context_with_current_consent() {
        assert!(request("Tuesday review", SAVED_CONTEXT_SCOPE_ID)
            .validate()
            .is_ok());
    }

    #[test]
    fn rejects_an_unnamed_context() {
        assert_eq!(
            request("   ", SAVED_CONTEXT_SCOPE_ID)
                .validate()
                .unwrap_err(),
            SavedContextError::NameMissing
        );
    }

    #[test]
    fn rejects_a_save_with_no_confirmed_scope() {
        assert_eq!(
            request("Tuesday review", "").validate().unwrap_err(),
            SavedContextError::ConsentMissing
        );
    }

    #[test]
    fn rejects_consent_given_for_a_different_scope() {
        let error = request("Tuesday review", "saved-context-scope-v1")
            .validate()
            .unwrap_err();
        assert_eq!(
            error,
            SavedContextError::ConsentStale {
                approved: "saved-context-scope-v1".into(),
                current: SAVED_CONTEXT_SCOPE_ID.into(),
            }
        );
    }

    #[test]
    fn every_captured_scope_line_is_explained_and_keyed() {
        let scope = SavedContextCaptureScope::current();
        assert_eq!(scope.id, SAVED_CONTEXT_SCOPE_ID);
        assert!(!scope.purpose.trim().is_empty());
        assert!(!scope.captured.is_empty());
        assert!(!scope.excluded.is_empty());
        for item in scope.captured.iter().chain(scope.excluded.iter()) {
            assert!(!item.key.trim().is_empty());
            assert!(!item.summary.trim().is_empty());
        }
        assert!(scope.captured.iter().any(|item| item.key == "restore_identity"));
    }

    #[test]
    fn the_scope_declares_the_window_title_caveat_it_cannot_avoid() {
        let scope = SavedContextCaptureScope::current();
        let titles = scope
            .captured
            .iter()
            .find(|item| item.key == "window_titles")
            .expect("window titles are captured");
        assert!(titles.summary.contains("page or file"));
    }

    #[test]
    fn title_fingerprint_is_whitespace_normalized_lowercase() {
        assert_eq!(title_fingerprint("  Hello   World\t"), "hello world");
    }

    #[test]
    fn restore_identity_rejects_partial_fields() {
        let incomplete = SavedContextRestoreIdentity {
            identity_schema_version: RESTORE_IDENTITY_SCHEMA_VERSION.into(),
            desktop_session_id: String::new(),
            captured_hwnd: "0x1".into(),
            captured_process_id: 1,
            title_fingerprint: "x".into(),
            captured_at: "t".into(),
        };
        assert!(!incomplete.is_complete());
    }
}
