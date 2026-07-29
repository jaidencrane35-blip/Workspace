//! Desktop Arrangement — durable membership + optional restore geometry (DAF-1c/1d).
//!
//! Why: Workspace must remember named window membership and captured bounds before
//! governed restore can apply them through WindowController.
//! Owner: domain contracts here; SQLite via `DesktopArrangementRepository`;
//! apply ownership lives in kernel (`DesktopArrangementService`) after PermissionGateway.
//! Separate from: Canvas `Layout` (zone board), Assistant/AI.
//!
//! The arrangement record itself never moves windows (`attempt_apply` / `attempt_control_window`).

use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;
use crate::ids::{DesktopArrangementId, WorkspaceId};
use crate::workspace_observation::{
    ObservedWindow, ObservedWindowAvailability, WorkspaceObservationSnapshot,
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

    #[error("no observation snapshot available for desktop arrangement capture/restore")]
    MissingObservation,

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

/// Optional screen-space bounds captured with membership (DAF-1d restore geometry).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl DesktopArrangementBounds {
    pub fn validate(&self) -> Result<()> {
        if self.width <= 0 || self.height <= 0 {
            return Err(DesktopArrangementError::Invalid(format!(
                "bounds width and height must be positive (got {}×{})",
                self.width, self.height
            )));
        }
        Ok(())
    }
}

/// One membership row — soft reference to an observed window identity.
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
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
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

    pub fn stored_bounds(&self) -> Option<DesktopArrangementBounds> {
        match (self.x, self.y, self.width, self.height) {
            (Some(x), Some(y), Some(width), Some(height)) => {
                let bounds = DesktopArrangementBounds {
                    x,
                    y,
                    width,
                    height,
                };
                bounds.validate().ok()?;
                Some(bounds)
            }
            _ => None,
        }
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
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
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
    /// True when restore cannot apply a move for this entry.
    pub restore_gap: bool,
    pub gap_reason: Option<String>,
}

/// Planned WindowController action for one available entry with bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementRestoreAction {
    pub entry_id: String,
    pub label: String,
    pub hwnd: String,
    pub bounds: DesktopArrangementBounds,
    pub focus: bool,
}

/// Restore plan: diagnostics for every entry + only authorised-candidate actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementRestorePlan {
    pub arrangement_id: String,
    pub diagnostics: Vec<DesktopArrangementEntryDiagnostic>,
    pub actions: Vec<DesktopArrangementRestoreAction>,
}

/// Outcome of one attempted controller apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopArrangementApplyStatus {
    Applied,
    Failed,
    Skipped,
}

/// Per-action apply result recorded for diagnostics (never silent).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementApplyOutcome {
    pub entry_id: String,
    pub label: String,
    pub hwnd: String,
    pub status: DesktopArrangementApplyStatus,
    pub detail: String,
    pub simulated: bool,
}

/// Full restore result returned to callers after governed apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopArrangementRestoreResult {
    pub arrangement_id: String,
    pub diagnostics: Vec<DesktopArrangementEntryDiagnostic>,
    pub outcomes: Vec<DesktopArrangementApplyOutcome>,
    pub applied_count: usize,
    pub gap_count: usize,
    pub failed_count: usize,
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
        if let Some(bounds) = entry.stored_bounds() {
            bounds.validate()?;
        } else {
            let any_bound = entry.x.is_some()
                || entry.y.is_some()
                || entry.width.is_some()
                || entry.height.is_some();
            if any_bound {
                return Err(DesktopArrangementError::Invalid(format!(
                    "entry {} has incomplete or invalid bounds",
                    entry.id
                )));
            }
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
            let (detail, restore_gap, gap_reason) = match availability {
                ObservedWindowAvailability::Available => {
                    if entry.stored_bounds().is_some() {
                        (
                            "Referenced window is present; bounds available for restore.".into(),
                            false,
                            None,
                        )
                    } else {
                        (
                            "Referenced window is present but no stored bounds; restore will not move it."
                                .into(),
                            true,
                            Some("missing_bounds".into()),
                        )
                    }
                }
                ObservedWindowAvailability::IdentityKnownWindowMissing => (
                    "Identity is known to observation history but missing from current windows."
                        .into(),
                    true,
                    Some("identity_known_window_missing".into()),
                ),
                ObservedWindowAvailability::Unavailable => (
                    "No matching window or identity found in the observation snapshot.".into(),
                    true,
                    Some("unavailable".into()),
                ),
            };
            DesktopArrangementEntryDiagnostic {
                entry_id: entry.id.clone(),
                label: entry.label.clone(),
                stable_window_id: entry.stable_window_id.clone(),
                hwnd: entry.hwnd.clone(),
                availability,
                detail,
                restore_gap,
                gap_reason,
            }
        })
        .collect()
}

/// Resolve the currently observed window for an entry (stable id first, then hwnd).
pub fn match_observed_window<'a>(
    entry: &DesktopArrangementEntry,
    snapshot: &'a WorkspaceObservationSnapshot,
) -> Option<&'a ObservedWindow> {
    if let Some(stable_id) = entry
        .stable_window_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        if let Some(window) = snapshot.windows.iter().find(|window| {
            window
                .stable_window_id
                .as_deref()
                .is_some_and(|id| id == stable_id)
        }) {
            return Some(window);
        }
    }
    if let Some(hwnd) = entry
        .hwnd
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        return snapshot.windows.iter().find(|window| window.hwnd == hwnd);
    }
    None
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

/// Build a restore plan: diagnostics for all entries; actions only for available+bounds.
///
/// Does **not** launch replacements or invent geometry for missing windows.
pub fn plan_desktop_arrangement_restore(
    arrangement: &DesktopArrangement,
    snapshot: &WorkspaceObservationSnapshot,
    focus_first: bool,
) -> DesktopArrangementRestorePlan {
    let diagnostics = diagnose_arrangement_entries(arrangement, snapshot);
    let mut actions = Vec::new();
    let mut focus_assigned = false;

    for entry in &arrangement.entries {
        let Some(window) = match_observed_window(entry, snapshot) else {
            continue;
        };
        let Some(bounds) = entry.stored_bounds() else {
            continue;
        };
        let focus = focus_first && !focus_assigned;
        if focus {
            focus_assigned = true;
        }
        actions.push(DesktopArrangementRestoreAction {
            entry_id: entry.id.clone(),
            label: entry.label.clone(),
            hwnd: window.hwnd.clone(),
            bounds,
            focus,
        });
    }

    DesktopArrangementRestorePlan {
        arrangement_id: arrangement.id.as_str().into(),
        diagnostics,
        actions,
    }
}

/// Build entry inputs from an observation snapshot (all windows, deterministic order).
pub fn capture_entry_inputs_from_snapshot(
    snapshot: &WorkspaceObservationSnapshot,
) -> Vec<DesktopArrangementEntryInput> {
    let mut windows = snapshot.windows.clone();
    windows.sort_by(|a, b| {
        a.z_order
            .cmp(&b.z_order)
            .then_with(|| a.hwnd.cmp(&b.hwnd))
            .then_with(|| a.id.cmp(&b.id))
    });

    windows
        .into_iter()
        .enumerate()
        .map(|(index, window)| DesktopArrangementEntryInput {
            stable_window_id: window.stable_window_id.clone(),
            hwnd: Some(window.hwnd.clone()),
            process_id: Some(window.process_id),
            process_name: window.process_name.clone(),
            title_fingerprint: Some(window.title.clone()),
            label: if window.title.trim().is_empty() {
                window
                    .process_name
                    .clone()
                    .unwrap_or_else(|| format!("window-{}", index + 1))
            } else {
                window.title.clone()
            },
            sort_order: index as i32,
            x: Some(window.x),
            y: Some(window.y),
            width: Some(window.width),
            height: Some(window.height),
        })
        .collect()
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
            x: input.x,
            y: input.y,
            width: input.width,
            height: input.height,
            authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
        };
        if !entry.has_identity_reference() {
            return Err(DesktopArrangementError::Invalid(format!(
                "entry input {index} requires stable_window_id and/or hwnd"
            )));
        }
        if let Some(bounds) = entry.stored_bounds() {
            bounds.validate()?;
        } else {
            let any_bound = entry.x.is_some()
                || entry.y.is_some()
                || entry.width.is_some()
                || entry.height.is_some();
            if any_bound {
                return Err(DesktopArrangementError::Invalid(format!(
                    "entry input {index} has incomplete or invalid bounds"
                )));
            }
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

    fn sample_entry(
        id: &str,
        stable: Option<&str>,
        hwnd: Option<&str>,
        bounds: Option<(i32, i32, i32, i32)>,
    ) -> DesktopArrangementEntry {
        DesktopArrangementEntry {
            id: id.into(),
            arrangement_id: "arr-1".into(),
            stable_window_id: stable.map(str::to_string),
            hwnd: hwnd.map(str::to_string),
            process_id: Some(10),
            process_name: Some("code.exe".into()),
            title_fingerprint: Some("code".into()),
            label: id.into(),
            sort_order: 0,
            x: bounds.map(|b| b.0),
            y: bounds.map(|b| b.1),
            width: bounds.map(|b| b.2),
            height: bounds.map(|b| b.3),
            authority_effect: DesktopArrangementEntry::AUTHORITY_EFFECT_NONE.into(),
        }
    }

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
        let arrangement = sample_arrangement(vec![sample_entry("e1", None, None, None)]);
        assert!(matches!(
            validate_desktop_arrangement(&arrangement),
            Err(DesktopArrangementError::Invalid(_))
        ));
    }

    #[test]
    fn validate_accepts_stable_or_hwnd_refs() {
        let arrangement = sample_arrangement(vec![
            sample_entry("e1", Some("stable-1"), None, Some((0, 0, 100, 100))),
            sample_entry("e2", None, Some("0xAA"), None),
        ]);
        assert!(validate_desktop_arrangement(&arrangement).is_ok());
    }

    #[test]
    fn diagnose_missing_and_available_entries() {
        let arrangement = sample_arrangement(vec![
            sample_entry(
                "e-present",
                Some("stable-1"),
                Some("0xAA"),
                Some((10, 20, 800, 600)),
            ),
            sample_entry(
                "e-gone",
                Some("stable-gone"),
                Some("0xBB"),
                Some((0, 0, 100, 100)),
            ),
            sample_entry("e-unknown", Some("never-seen"), None, None),
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
        assert!(!diagnostics[0].restore_gap);
        assert_eq!(
            diagnostics[1].availability,
            ObservedWindowAvailability::IdentityKnownWindowMissing
        );
        assert!(diagnostics[1].restore_gap);
        assert_eq!(
            diagnostics[1].gap_reason.as_deref(),
            Some("identity_known_window_missing")
        );
        assert_eq!(
            diagnostics[2].availability,
            ObservedWindowAvailability::Unavailable
        );
        assert!(diagnostics[2].restore_gap);
    }

    #[test]
    fn plan_restore_matches_current_hwnd_and_skips_missing() {
        let arrangement = sample_arrangement(vec![
            sample_entry(
                "e-present",
                Some("stable-1"),
                Some("0xOLD"),
                Some((10, 20, 800, 600)),
            ),
            sample_entry(
                "e-gone",
                Some("stable-gone"),
                Some("0xBB"),
                Some((0, 0, 100, 100)),
            ),
            sample_entry("e-no-bounds", Some("stable-2"), Some("0xCC"), None),
        ]);

        let mut snapshot = empty_stub_snapshot("pass-1", "2026-07-29T12:00:00Z");
        snapshot.windows.push(ObservedWindow {
            id: "w1".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xNEW".into(),
            stable_window_id: Some("stable-1".into()),
            title: "Code".into(),
            process_id: 10,
            process_name: Some("code.exe".into()),
            x: 1,
            y: 2,
            width: 3,
            height: 4,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.windows.push(ObservedWindow {
            id: "w2".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xCC".into(),
            stable_window_id: Some("stable-2".into()),
            title: "Other".into(),
            process_id: 11,
            process_name: Some("other.exe".into()),
            x: 0,
            y: 0,
            width: 50,
            height: 50,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(1),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.pass.window_count = 2;

        let plan = plan_desktop_arrangement_restore(&arrangement, &snapshot, true);
        assert_eq!(plan.actions.len(), 1);
        assert_eq!(plan.actions[0].hwnd, "0xNEW");
        assert_eq!(
            plan.actions[0].bounds,
            DesktopArrangementBounds {
                x: 10,
                y: 20,
                width: 800,
                height: 600
            }
        );
        assert!(plan.actions[0].focus);
        assert_eq!(plan.diagnostics.iter().filter(|d| d.restore_gap).count(), 2);
    }

    #[test]
    fn matching_is_deterministic_stable_before_hwnd() {
        let entry = sample_entry("e1", Some("stable-1"), Some("0xOTHER"), Some((1, 2, 3, 4)));
        let mut snapshot = empty_stub_snapshot("pass-1", "2026-07-29T12:00:00Z");
        snapshot.windows.push(ObservedWindow {
            id: "w-other".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xOTHER".into(),
            stable_window_id: Some("stable-other".into()),
            title: "Other".into(),
            process_id: 1,
            process_name: None,
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.windows.push(ObservedWindow {
            id: "w1".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xSTABLE".into(),
            stable_window_id: Some("stable-1".into()),
            title: "Primary".into(),
            process_id: 2,
            process_name: None,
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(1),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        let matched = match_observed_window(&entry, &snapshot).unwrap();
        assert_eq!(matched.hwnd, "0xSTABLE");
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
    fn capture_from_snapshot_is_deterministic_and_includes_bounds() {
        let mut snapshot = empty_stub_snapshot("pass-1", "2026-07-29T12:00:00Z");
        snapshot.windows.push(ObservedWindow {
            id: "w2".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xBB".into(),
            stable_window_id: Some("stable-b".into()),
            title: "B".into(),
            process_id: 2,
            process_name: Some("b.exe".into()),
            x: 5,
            y: 6,
            width: 7,
            height: 8,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(1),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        snapshot.windows.push(ObservedWindow {
            id: "w1".into(),
            pass_id: snapshot.pass.id.clone(),
            hwnd: "0xAA".into(),
            stable_window_id: Some("stable-a".into()),
            title: "A".into(),
            process_id: 1,
            process_name: Some("a.exe".into()),
            x: 1,
            y: 2,
            width: 3,
            height: 4,
            monitor_id: None,
            visible: true,
            minimized: false,
            focused: false,
            z_order: Some(0),
            authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
        });
        let first = capture_entry_inputs_from_snapshot(&snapshot);
        let second = capture_entry_inputs_from_snapshot(&snapshot);
        assert_eq!(first, second);
        assert_eq!(first[0].hwnd.as_deref(), Some("0xAA"));
        assert_eq!(first[0].x, Some(1));
        assert_eq!(first[1].hwnd.as_deref(), Some("0xBB"));
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
            x: Some(1),
            y: Some(2),
            width: Some(3),
            height: Some(4),
        }];
        let first = entries_from_inputs(&id, &inputs, |index| format!("e-{index}")).unwrap();
        let second = entries_from_inputs(&id, &inputs, |index| format!("e-{index}")).unwrap();
        assert_eq!(first, second);
        assert_eq!(first[0].stored_bounds().unwrap().width, 3);
    }

    #[test]
    fn no_automatic_repair_for_missing_windows() {
        let arrangement = sample_arrangement(vec![sample_entry(
            "e-gone",
            Some("stable-gone"),
            Some("0xBB"),
            Some((0, 0, 100, 100)),
        )]);
        let snapshot = empty_stub_snapshot("pass-1", "2026-07-29T12:00:00Z");
        let plan = plan_desktop_arrangement_restore(&arrangement, &snapshot, true);
        assert!(plan.actions.is_empty());
        assert_eq!(plan.diagnostics.len(), 1);
        assert!(plan.diagnostics[0].restore_gap);
    }
}
