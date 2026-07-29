//! Desktop Arrangement — durable “what belongs together” for observed OS windows (DAF-1c).
//!
//! Why: Workspace must remember named window membership before it can restore or apply it.
//! Owner: domain contracts here; SQLite via `DesktopArrangementRepository`.
//! Separate from: Canvas `Layout` (zone board), `WindowController` (mutation), Assistant/AI.
//!
//! Entries reference observation identities / hwnds. They do **not** store geometry or
//! execute moves. Missing windows are diagnosed, never auto-repaired.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{DesktopArrangementId, WorkspaceId};
use crate::workspace_observation::{
    ObservedWindowAvailability, WorkspaceObservationSnapshot,
};

/// Desktop arrangement validation / ownership errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DesktopArrangementError {
    #[error("desktop arrangements require a workspace id")]
    MissingWorkspace,

    #[error("desktop arrangement not found: {0}")]
    NotFound(String),

    #[error("desktop arrangements cannot move, restore, or authorize windows")]
    CannotControl,

    #[error("desktop arrangement validation failed: {0}")]
    Invalid(String),

    #[error(transparent)]
    Domain(#[from] DomainError),
}

pub type Result<T> = std::result::Result<T, DesktopArrangementError>;

/// Lifecycle of a persisted arrangement (descriptive; not an apply state machine).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopArrangementStatus {
    Draft,
    Active,
    Archived,
}

impl DesktopArrangementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "draft" => Ok(Self::Draft),
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            other => Err(DesktopArrangementError::Invalid(format!(
                "unknown arrangement status: {other}"
            ))),
        }
    }
}

/// One membership row — soft reference to an observed window identity.
///
/// Geometry is intentionally omitted in DAF-1c (“belongs together”, not “move here”).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementEntry {
    pub id: String,
    pub arrangement_id: String,
    /// Preferred durable ref — `ObservationWindowIdentity.id`.
    pub stable_window_id: Option<String>,
    /// Last-known hwnd string from capture (may churn).
    pub hwnd: Option<String>,
    pub process_id: Option<i32>,
    pub process_name: Option<String>,
    pub title_fingerprint: Option<String>,
    pub label: String,
    pub sort_order: i32,
    pub authority_effect: String,
}

impl DesktopArrangementEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn has_identity_reference(&self) -> bool {
        self.stable_window_id
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
            || self.hwnd.as_deref().is_some_and(|value| !value.trim().is_empty())
    }
}

/// Named desktop arrangement for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangement {
    pub id: DesktopArrangementId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub description: String,
    pub status: DesktopArrangementStatus,
    pub entries: Vec<DesktopArrangementEntry>,
    pub created_at: String,
    pub updated_at: String,
    pub authority_effect: String,
}

impl DesktopArrangement {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    /// Architecture guard — persistence must not move windows.
    pub fn attempt_apply() -> Result<()> {
        Err(DesktopArrangementError::CannotControl)
    }

    /// Architecture guard — persistence must not call OS control.
    pub fn attempt_control_window() -> Result<()> {
        Err(DesktopArrangementError::CannotControl)
    }
}

/// Input used when creating/replacing membership rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementEntryInput {
    pub stable_window_id: Option<String>,
    pub hwnd: Option<String>,
    pub process_id: Option<i32>,
    pub process_name: Option<String>,
    pub title_fingerprint: Option<String>,
    pub label: String,
    pub sort_order: i32,
}

/// Per-entry availability diagnosis against a factual observation snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementEntryDiagnostic {
    pub entry_id: String,
    pub label: String,
    pub stable_window_id: Option<String>,
    pub hwnd: Option<String>,
    pub availability: ObservedWindowAvailability,
    pub detail: String,
}

pub fn arrangement_now_rfc3339() -> String {
    Utc::now().to_rfc3339()
}

/// Validates arrangement invariants (authority, identity refs, naming).
pub fn validate_desktop_arrangement(arrangement: &DesktopArrangement) -> Result<()> {
    if arrangement.authority_effect != DesktopArrangement::AUTHORITY_EFFECT_NONE {
        return Err(DesktopArrangementError::Invalid(
            "authority_effect must be none".into(),
        ));
    }
    if arrangement.workspace_id.as_str().trim().is_empty() {
        return Err(DesktopArrangementError::MissingWorkspace);
    }
    if arrangement.name.trim().is_empty() {
        return Err(DesktopArrangementError::Invalid(
            "arrangement name is required".into(),
        ));
    }
    if arrangement.created_at.trim().is_empty() || arrangement.updated_at.trim().is_empty() {
        return Err(DesktopArrangementError::Invalid(
            "created_at and updated_at are required".into(),
        ));
    }

    for entry in &arrangement.entries {
        if entry.arrangement_id != arrangement.id.as_str() {
            return Err(DesktopArrangementError::Invalid(format!(
                "entry {} arrangement_id mismatch",
                entry.id
            )));
        }
        if entry.authority_effect != DesktopArrangementEntry::AUTHORITY_EFFECT_NONE {
            return Err(DesktopArrangementError::Invalid(format!(
                "entry {} authority_effect must be none",
                entry.id
            )));
        }
        if entry.id.trim().is_empty() {
            return Err(DesktopArrangementError::Invalid(
                "entry id is required".into(),
            ));
        }
        if !entry.has_identity_reference() {
            return Err(DesktopArrangementError::Invalid(format!(
                "entry {} requires stable_window_id and/or hwnd",
                entry.id
            )));
        }
    }
    Ok(())
}

/// Diagnose each entry against a current observation snapshot (facts only).
pub fn diagnose_arrangement_entries(
    arrangement: &DesktopArrangement,
    snapshot: &WorkspaceObservationSnapshot,
) -> Vec<DesktopArrangementEntryDiagnostic> {
    arrangement
        .entries
        .iter()
        .map(|entry| {
            let availability = diagnose_entry_availability(entry, snapshot);
            let detail = match availability {
                ObservedWindowAvailability::Available => {
                    "Referenced window is present in the observation snapshot.".into()
                }
                ObservedWindowAvailability::IdentityKnownWindowMissing => {
                    "Identity is known to observation history but missing from current windows."
                        .into()
                }
                ObservedWindowAvailability::Unavailable => {
                    "No matching window or identity found in the observation snapshot.".into()
                }
            };
            DesktopArrangementEntryDiagnostic {
                entry_id: entry.id.clone(),
                label: entry.label.clone(),
                stable_window_id: entry.stable_window_id.clone(),
                hwnd: entry.hwnd.clone(),
                availability,
                detail,
            }
        })
        .collect()
}

fn diagnose_entry_availability(
    entry: &DesktopArrangementEntry,
    snapshot: &WorkspaceObservationSnapshot,
) -> ObservedWindowAvailability {
    if let Some(stable_id) = entry
        .stable_window_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let by_stable = snapshot.availability_for_stable_id(stable_id);
        if by_stable != ObservedWindowAvailability::Unavailable {
            return by_stable;
        }
    }
    if let Some(hwnd) = entry
        .hwnd
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return snapshot.availability_for_hwnd(hwnd);
    }
    ObservedWindowAvailability::Unavailable
}

/// Build entry rows from inputs (caller supplies ids / timestamps separately for persistence).
pub fn entries_from_inputs(
    arrangement_id: &DesktopArrangementId,
    inputs: &[DesktopArrangementEntryInput],
    id_factory: impl Fn(usize) -> String,
) -> Result<Vec<DesktopArrangementEntry>> {
    let mut entries = Vec::with_capacity(inputs.len());
    for (index, input) in inputs.iter().enumerate() {
        let entry = DesktopArrangementEntry {
            id: id_factory(index),
            arrangement_id: arrangement_id.as_str().into(),
            stable_window_id: normalize_optional(&input.stable_window_id),
            hwnd: normalize_optional(&input.hwnd),
            process_id: input.process_id,
            process_name: normalize_optional(&input.process_name),
            title_fingerprint: normalize_optional(&input.title_fingerprint),
            label: input.label.trim().to_string(),
            sort_order: input.sort_order,
            authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
        };
        if !entry.has_identity_reference() {
            return Err(DesktopArrangementError::Invalid(format!(
                "entry input {index} requires stable_window_id and/or hwnd"
            )));
        }
        entries.push(entry);
    }
    Ok(entries)
}

fn normalize_optional(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|item| item.trim().to_string())
        .filter(|item| !item.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace_observation::{
        empty_stub_snapshot, ObservationWindowIdentity, ObservedWindow, WindowIdentityConfidence,
    };

    fn sample_arrangement(entries: Vec<DesktopArrangementEntry>) -> DesktopArrangement {
        DesktopArrangement {
            id: DesktopArrangementId::new("arr-1").unwrap(),
            workspace_id: WorkspaceId::new("ws-1").unwrap(),
            name: "Focus coding".into(),
            description: "Editor + browser".into(),
            status: DesktopArrangementStatus::Active,
            entries,
            created_at: "2026-07-29T12:00:00Z".into(),
            updated_at: "2026-07-29T12:00:00Z".into(),
            authority_effect: DesktopArrangement::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn validate_requires_identity_reference() {
        let arrangement = sample_arrangement(vec![DesktopArrangementEntry {
            id: "e1".into(),
            arrangement_id: "arr-1".into(),
            stable_window_id: None,
            hwnd: None,
            process_id: Some(1),
            process_name: Some("app.exe".into()),
            title_fingerprint: Some("app".into()),
            label: "App".into(),
            sort_order: 0,
            authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
        }]);
        assert!(matches!(
            validate_desktop_arrangement(&arrangement),
            Err(DesktopArrangementError::Invalid(_))
        ));
    }

    #[test]
    fn validate_accepts_stable_or_hwnd_refs() {
        let arrangement = sample_arrangement(vec![
            DesktopArrangementEntry {
                id: "e1".into(),
                arrangement_id: "arr-1".into(),
                stable_window_id: Some("stable-1".into()),
                hwnd: None,
                process_id: Some(10),
                process_name: Some("code.exe".into()),
                title_fingerprint: Some("code".into()),
                label: "Editor".into(),
                sort_order: 0,
                authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
            },
            DesktopArrangementEntry {
                id: "e2".into(),
                arrangement_id: "arr-1".into(),
                stable_window_id: None,
                hwnd: Some("0xAA".into()),
                process_id: None,
                process_name: None,
                title_fingerprint: None,
                label: "Browser".into(),
                sort_order: 1,
                authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
            },
        ]);
        assert!(validate_desktop_arrangement(&arrangement).is_ok());
    }

    #[test]
    fn diagnose_missing_and_available_entries() {
        let arrangement = sample_arrangement(vec![
            DesktopArrangementEntry {
                id: "e-present".into(),
                arrangement_id: "arr-1".into(),
                stable_window_id: Some("stable-1".into()),
                hwnd: Some("0xAA".into()),
                process_id: Some(10),
                process_name: Some("code.exe".into()),
                title_fingerprint: Some("code".into()),
                label: "Editor".into(),
                sort_order: 0,
                authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
            },
            DesktopArrangementEntry {
                id: "e-gone".into(),
                arrangement_id: "arr-1".into(),
                stable_window_id: Some("stable-gone".into()),
                hwnd: Some("0xBB".into()),
                process_id: Some(11),
                process_name: Some("chat.exe".into()),
                title_fingerprint: Some("chat".into()),
                label: "Chat".into(),
                sort_order: 1,
                authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
            },
            DesktopArrangementEntry {
                id: "e-unknown".into(),
                arrangement_id: "arr-1".into(),
                stable_window_id: Some("never-seen".into()),
                hwnd: None,
                process_id: None,
                process_name: None,
                title_fingerprint: None,
                label: "Unknown".into(),
                sort_order: 2,
                authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
            },
        ]);

        let mut snapshot = empty_stub_snapshot("pass-1", "2026-07-29T12:00:00Z");
        snapshot.windows.push(ObservedWindow {
            id: "w1".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xAA".into(),
            stable_window_id: Some("stable-1".into()),
            title: "Code".into(),
            process_id: 10,
            process_name: Some("code.exe".into()),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.pass.window_count = 1;
        snapshot.identities.push(ObservationWindowIdentity {
            id: "stable-gone".into(),
            process_id: 11,
            title_fingerprint: "chat".into(),
            first_seen_at: "2026-07-29T11:00:00Z".into(),
            last_seen_at: "2026-07-29T11:00:00Z".into(),
            last_hwnd: "0xBB".into(),
            confidence: WindowIdentityConfidence::High,
            authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
        });

        let diagnostics = diagnose_arrangement_entries(&arrangement, &snapshot);
        assert_eq!(diagnostics.len(), 3);
        assert_eq!(
            diagnostics[0].availability,
            ObservedWindowAvailability::Available
        );
        assert_eq!(
            diagnostics[1].availability,
            ObservedWindowAvailability::IdentityKnownWindowMissing
        );
        assert_eq!(
            diagnostics[2].availability,
            ObservedWindowAvailability::Unavailable
        );
    }

    #[test]
    fn arrangement_refuses_control_leakage() {
        assert!(matches!(
            DesktopArrangement::attempt_apply(),
            Err(DesktopArrangementError::CannotControl)
        ));
        assert!(matches!(
            DesktopArrangement::attempt_control_window(),
            Err(DesktopArrangementError::CannotControl)
        ));
    }

    #[test]
    fn entries_from_inputs_are_deterministic() {
        let id = DesktopArrangementId::new("arr-1").unwrap();
        let inputs = vec![DesktopArrangementEntryInput {
            stable_window_id: Some("stable-1".into()),
            hwnd: Some("0xAA".into()),
            process_id: Some(1),
            process_name: Some("a.exe".into()),
            title_fingerprint: Some("a".into()),
            label: "A".into(),
            sort_order: 0,
        }];
        let first = entries_from_inputs(&id, &inputs, |index| format!("e-{index}")).unwrap();
        let second = entries_from_inputs(&id, &inputs, |index| format!("e-{index}")).unwrap();
        assert_eq!(first, second);
    }
}
