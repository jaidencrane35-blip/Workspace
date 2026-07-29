//! Workspace Observation Layer — durable desktop perception (Phase 6 / Sprint 105–107A).
//!
//! Captures OS desktop state, reconciles window identities, persists snapshots.
//! Never executes, matches applications, or grants authority.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use serde_json::json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use workspace_database::{Database, ObservationPassRepository};
use chrono::Utc;
use workspace_domain::{
    build_observation_status, observation_now_rfc3339, observation_u32_to_i32,
    observation_u64_to_i32, observation_usize_to_i32, IntentContext, ObservationCaptureErrorClass,
    ObservationCaptureFailure, ObservationWindowIdentity, ObservedMonitor, ObservedWindow,
    WindowIdentityConfidence, WorkspaceObservationError, WorkspaceObservationPass,
    WorkspaceObservationSnapshot, WorkspaceObservationStatus,
};
use workspace_windows_integration::{
    platform_desktop_capturer, CapturedDesktopMonitor, CapturedDesktopWindow, DesktopCapturer,
    DesktopObservationCapture,
};

use crate::error::{KernelError, Result};
use crate::services::AuditService;
use workspace_domain::ActorContext;

const DEFAULT_PASS_RETENTION: usize = 50;
const MEDIUM_TITLE_DISTANCE: usize = 3;

pub(crate) struct WorkspaceObservationService;

impl WorkspaceObservationService {
    pub(crate) fn capture(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceObservationCaptureResult> {
        let capturer = platform_desktop_capturer();
        Self::capture_with(db, actor, intent, capturer.as_ref())
    }

    pub(crate) fn capture_with(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        capturer: &dyn DesktopCapturer,
    ) -> Result<WorkspaceObservationCaptureResult> {
        let capture = match capturer.capture_desktop() {
            Ok(capture) => capture,
            Err(error) => {
                let message = error.to_string();
                Self::record_capture_failure(
                    db,
                    ObservationCaptureErrorClass::WindowsIntegration,
                    &message,
                    Some("win32"),
                )?;
                return Err(KernelError::WindowsIntegration { message });
            }
        };
        match Self::persist_capture(db, actor, intent, capture) {
            Ok(result) => {
                Self::clear_capture_failure(db)?;
                Ok(result)
            }
            Err(error) => {
                let (class, source) = classify_capture_error(&error);
                let _ = Self::record_capture_failure(db, class, &error.to_string(), source);
                Err(error)
            }
        }
    }

    /// Lightweight observation pipeline status — metadata only, never a full snapshot.
    pub(crate) fn get_status(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<WorkspaceObservationStatus> {
        Self::get_status_at(db, actor, intent, Utc::now())
    }

    pub(crate) fn get_status_at(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        now: chrono::DateTime<Utc>,
    ) -> Result<WorkspaceObservationStatus> {
        let (metadata, last_failure) = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let repo = ObservationPassRepository::new(&guard);
            (repo.get_latest_metadata()?, repo.get_capture_failure()?)
        };
        let status = build_observation_status(metadata.as_ref(), last_failure, now)
            .map_err(|error| KernelError::ObservationValidation {
                message: error.to_string(),
            })?;
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            "workspace.observation.status_read",
            true,
            json!({
                "has_observation": status.has_observation,
                "freshness": status.freshness.as_str(),
                "pass_id": status.pass_id,
                "age_seconds": status.age_seconds,
                "has_failure": status.last_failure.is_some(),
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(status)
    }

    fn record_capture_failure(
        db: &Arc<Mutex<Database>>,
        error_class: ObservationCaptureErrorClass,
        message: &str,
        source: Option<&str>,
    ) -> Result<()> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        ObservationPassRepository::new(&guard).record_capture_failure(&ObservationCaptureFailure {
            failed_at: observation_now_rfc3339(),
            error_class,
            message: message.into(),
            source: source.map(str::to_string),
            authority_effect: ObservationCaptureFailure::AUTHORITY_EFFECT_NONE.into(),
        })?;
        Ok(())
    }

    fn clear_capture_failure(db: &Arc<Mutex<Database>>) -> Result<()> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        ObservationPassRepository::new(&guard).clear_capture_failure()?;
        Ok(())
    }

    pub(crate) fn get_latest_snapshot(
        db: &Arc<Mutex<Database>>,
    ) -> Result<Option<WorkspaceObservationSnapshot>> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(ObservationPassRepository::new(&guard).load_latest_snapshot()?)
    }

    pub(crate) fn get_latest(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
    ) -> Result<Option<WorkspaceObservationSnapshot>> {
        let snapshot = Self::get_latest_snapshot(db)?;
        if snapshot.is_some() {
            Self::audit_read(db, actor, intent, "latest", snapshot.as_ref())?;
        }
        Ok(snapshot)
    }

    pub(crate) fn get_by_id(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        pass_id: impl Into<String>,
    ) -> Result<Option<WorkspaceObservationSnapshot>> {
        let pass_id = pass_id.into();
        let snapshot = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            ObservationPassRepository::new(&guard).load_snapshot(&pass_id)?
        };
        if snapshot.is_some() {
            Self::audit_read(db, actor, intent, &pass_id, snapshot.as_ref())?;
        }
        Ok(snapshot)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        WorkspaceObservationSnapshot::attempt_execute().map_err(|error| {
            KernelError::integrity_violation(format!(
                "workspace observation cannot execute: {error}"
            ))
        })
    }

    fn persist_capture(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        capture: DesktopObservationCapture,
    ) -> Result<WorkspaceObservationCaptureResult> {
        let captured_at = observation_now_rfc3339();
        let process_ids = capture_process_ids(&capture)?;
        let snapshot = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            ObservationPassRepository::new(&guard).persist_reconciled_snapshot(
                &process_ids,
                |existing| build_snapshot(&capture, existing, &captured_at),
                DEFAULT_PASS_RETENTION,
            )?
        };
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            "workspace.observation.captured",
            true,
            json!({
                "pass_id": snapshot.pass.id,
                "captured_at": snapshot.pass.captured_at,
                "window_count": snapshot.pass.window_count,
                "monitor_count": snapshot.pass.monitor_count,
                "identity_count": snapshot.identities.len(),
                "source": snapshot.pass.source,
                "authority_effect": "none",
            })
            .to_string(),
        )?;
        Ok(WorkspaceObservationCaptureResult {
            snapshot_id: snapshot.pass.id.clone(),
            captured_at: snapshot.pass.captured_at.clone(),
            window_count: snapshot.pass.window_count as usize,
            monitor_count: snapshot.pass.monitor_count as usize,
            identity_count: snapshot.identities.len(),
            snapshot,
        })
    }

    fn audit_read(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        pass_ref: &str,
        snapshot: Option<&WorkspaceObservationSnapshot>,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            intent,
            "workspace.observation.read",
            true,
            json!({
                "pass_ref": pass_ref,
                "pass_id": snapshot.map(|value| value.pass.id.clone()),
                "window_count": snapshot.map(|value| value.pass.window_count),
                "monitor_count": snapshot.map(|value| value.pass.monitor_count),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

/// Summary returned after a governed capture pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceObservationCaptureResult {
    pub snapshot_id: String,
    pub captured_at: String,
    pub window_count: usize,
    pub monitor_count: usize,
    pub identity_count: usize,
    pub snapshot: WorkspaceObservationSnapshot,
}

fn capture_process_ids(capture: &DesktopObservationCapture) -> Result<Vec<i32>> {
    let mut ids = HashSet::new();
    for window in &capture.windows {
        ids.insert(observation_u32_to_i32(window.process_id, "process_id")
            .map_err(map_observation_error)?);
    }
    let mut ordered: Vec<i32> = ids.into_iter().collect();
    ordered.sort_unstable();
    Ok(ordered)
}

fn build_snapshot(
    capture: &DesktopObservationCapture,
    existing: &[ObservationWindowIdentity],
    captured_at: &str,
) -> std::result::Result<WorkspaceObservationSnapshot, WorkspaceObservationError> {
    let pass_id = Uuid::new_v4().to_string();
    let mut monitor_ids: HashMap<i32, String> = HashMap::new();
    let monitors: Vec<ObservedMonitor> = capture
        .monitors
        .iter()
        .map(|monitor| map_monitor(&pass_id, monitor, &mut monitor_ids))
        .collect();

    let (identities, windows) = reconcile_windows(
        &pass_id,
        &capture.windows,
        existing,
        captured_at,
        &monitor_ids,
    )?;

    let metadata_json = json!({
        "capture_source": capture.metadata.source,
        "capture_duration_ms": capture.metadata.duration_ms,
    })
    .to_string();

    let snapshot = WorkspaceObservationSnapshot {
        pass: WorkspaceObservationPass {
            id: pass_id,
            captured_at: captured_at.into(),
            schema_version: WorkspaceObservationPass::DEFAULT_SCHEMA_VERSION,
            source: capture.metadata.source.clone(),
            foreground_hwnd: capture.foreground_hwnd.clone(),
            window_count: observation_usize_to_i32(windows.len(), "window_count")?,
            monitor_count: observation_usize_to_i32(monitors.len(), "monitor_count")?,
            duration_ms: capture
                .metadata
                .duration_ms
                .map(|value| observation_u64_to_i32(value, "duration_ms"))
                .transpose()?,
            metadata_json,
            authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
        },
        windows,
        monitors,
        identities,
        authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
    };
    snapshot.validate()?;
    Ok(snapshot)
}

fn map_monitor(
    pass_id: &str,
    monitor: &CapturedDesktopMonitor,
    monitor_ids: &mut HashMap<i32, String>,
) -> ObservedMonitor {
    let id = Uuid::new_v4().to_string();
    monitor_ids.insert(monitor.index, id.clone());
    ObservedMonitor {
        id,
        pass_id: pass_id.into(),
        monitor_index: monitor.index,
        name: monitor.name.clone(),
        x: monitor.x,
        y: monitor.y,
        width: monitor.width,
        height: monitor.height,
        work_x: monitor.work_x,
        work_y: monitor.work_y,
        work_w: monitor.work_width,
        work_h: monitor.work_height,
        is_primary: monitor.is_primary,
        dpi_scale: None,
        authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn reconcile_windows(
    pass_id: &str,
    windows: &[CapturedDesktopWindow],
    existing: &[ObservationWindowIdentity],
    captured_at: &str,
    monitor_ids: &HashMap<i32, String>,
) -> std::result::Result<(Vec<ObservationWindowIdentity>, Vec<ObservedWindow>), WorkspaceObservationError>
{
    let mut identities: Vec<ObservationWindowIdentity> = Vec::new();
    let mut observed: Vec<ObservedWindow> = Vec::new();
    let mut claimed: HashSet<String> = HashSet::new();

    for window in windows {
        let (identity, stable_id) =
            reconcile_identity(window, existing, captured_at, &identities, &claimed)?;
        if let Some(stable_id) = &stable_id {
            claimed.insert(stable_id.clone());
        }
        if let Some(identity) = identity {
            upsert_identity(&mut identities, identity);
        }
        observed.push(map_window(pass_id, window, stable_id, monitor_ids)?);
    }

    Ok((identities, observed))
}

/// Identity matching rules (Sprint 107A):
/// 1. process_id + exact title fingerprint → High (HWND-churn resilient)
/// 2. process_id + close title → Medium
/// 3. process_id + same last_hwnd → Low (title diverged, handle stable)
/// 4. otherwise → Ephemeral (never process_id alone)
fn reconcile_identity(
    window: &CapturedDesktopWindow,
    existing: &[ObservationWindowIdentity],
    captured_at: &str,
    pending: &[ObservationWindowIdentity],
    claimed: &HashSet<String>,
) -> std::result::Result<(Option<ObservationWindowIdentity>, Option<String>), WorkspaceObservationError>
{
    let fingerprint = title_fingerprint(&window.title);
    let process_id = observation_u32_to_i32(window.process_id, "process_id")?;

    if let Some(identity) =
        find_high_confidence(existing, pending, claimed, process_id, &fingerprint)
    {
        let updated =
            refresh_identity(identity, window, captured_at, WindowIdentityConfidence::High);
        return Ok((Some(updated.clone()), Some(updated.id)));
    }

    if let Some(identity) =
        find_medium_confidence(existing, pending, claimed, process_id, &window.title, &fingerprint)
    {
        let updated =
            refresh_identity(identity, window, captured_at, WindowIdentityConfidence::Medium);
        return Ok((Some(updated.clone()), Some(updated.id)));
    }

    if let Some(identity) =
        find_low_confidence(existing, pending, claimed, process_id, &window.hwnd)
    {
        let updated = refresh_identity(identity, window, captured_at, WindowIdentityConfidence::Low);
        return Ok((Some(updated.clone()), Some(updated.id)));
    }

    let identity = ObservationWindowIdentity {
        id: Uuid::new_v4().to_string(),
        process_id,
        title_fingerprint: fingerprint,
        first_seen_at: captured_at.into(),
        last_seen_at: captured_at.into(),
        last_hwnd: window.hwnd.clone(),
        confidence: WindowIdentityConfidence::Ephemeral,
        authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
    };
    Ok((Some(identity.clone()), Some(identity.id)))
}

fn find_high_confidence<'a>(
    existing: &'a [ObservationWindowIdentity],
    pending: &'a [ObservationWindowIdentity],
    claimed: &HashSet<String>,
    process_id: i32,
    fingerprint: &str,
) -> Option<&'a ObservationWindowIdentity> {
    existing
        .iter()
        .chain(pending.iter())
        .find(|identity| {
            !claimed.contains(&identity.id)
                && identity.process_id == process_id
                && identity.title_fingerprint == fingerprint
        })
}

fn find_medium_confidence<'a>(
    existing: &'a [ObservationWindowIdentity],
    pending: &'a [ObservationWindowIdentity],
    claimed: &HashSet<String>,
    process_id: i32,
    title: &str,
    fingerprint: &str,
) -> Option<&'a ObservationWindowIdentity> {
    existing
        .iter()
        .chain(pending.iter())
        .find(|identity| {
            !claimed.contains(&identity.id)
                && identity.process_id == process_id
                && identity.title_fingerprint != fingerprint
                && titles_are_close(title, &identity.title_fingerprint)
        })
}

fn find_low_confidence<'a>(
    existing: &'a [ObservationWindowIdentity],
    pending: &'a [ObservationWindowIdentity],
    claimed: &HashSet<String>,
    process_id: i32,
    hwnd: &str,
) -> Option<&'a ObservationWindowIdentity> {
    // Low confidence requires HWND continuity — never process_id alone.
    existing
        .iter()
        .chain(pending.iter())
        .find(|identity| {
            !claimed.contains(&identity.id)
                && identity.process_id == process_id
                && identity.last_hwnd == hwnd
        })
}

fn refresh_identity(
    identity: &ObservationWindowIdentity,
    window: &CapturedDesktopWindow,
    captured_at: &str,
    confidence: WindowIdentityConfidence,
) -> ObservationWindowIdentity {
    ObservationWindowIdentity {
        id: identity.id.clone(),
        process_id: identity.process_id,
        title_fingerprint: title_fingerprint(&window.title),
        first_seen_at: identity.first_seen_at.clone(),
        last_seen_at: captured_at.into(),
        last_hwnd: window.hwnd.clone(),
        confidence,
        authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn upsert_identity(identities: &mut Vec<ObservationWindowIdentity>, identity: ObservationWindowIdentity) {
    if let Some(existing) = identities.iter_mut().find(|item| item.id == identity.id) {
        *existing = identity;
    } else {
        identities.push(identity);
    }
}

fn map_window(
    pass_id: &str,
    window: &CapturedDesktopWindow,
    stable_window_id: Option<String>,
    monitor_ids: &HashMap<i32, String>,
) -> std::result::Result<ObservedWindow, WorkspaceObservationError> {
    Ok(ObservedWindow {
        id: Uuid::new_v4().to_string(),
        pass_id: pass_id.into(),
        hwnd: window.hwnd.clone(),
        stable_window_id,
        title: window.title.clone(),
        process_id: observation_u32_to_i32(window.process_id, "process_id")?,
        process_name: window.process_name.clone(),
        x: window.x,
        y: window.y,
        width: window.width,
        height: window.height,
        monitor_id: window
            .monitor_index
            .and_then(|index| monitor_ids.get(&index).cloned()),
        visible: window.visible,
        minimized: window.minimized,
        focused: window.focused,
        z_order: window.z_order,
        authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn title_fingerprint(title: &str) -> String {
    title.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

fn titles_are_close(left: &str, right: &str) -> bool {
    let left = title_fingerprint(left);
    let right = title_fingerprint(right);
    if left == right {
        return true;
    }
    edit_distance(&left, &right) <= MEDIUM_TITLE_DISTANCE
}

fn edit_distance(left: &str, right: &str) -> usize {
    let left_chars: Vec<char> = left.chars().collect();
    let right_chars: Vec<char> = right.chars().collect();
    if left_chars.is_empty() {
        return right_chars.len();
    }
    if right_chars.is_empty() {
        return left_chars.len();
    }

    let mut previous: Vec<usize> = (0..=right_chars.len()).collect();
    for (i, left_char) in left_chars.iter().enumerate() {
        let mut current = vec![i + 1; right_chars.len() + 1];
        for (j, right_char) in right_chars.iter().enumerate() {
            let cost = usize::from(left_char != right_char);
            current[j + 1] = (current[j] + 1)
                .min(previous[j + 1] + 1)
                .min(previous[j] + cost);
        }
        previous = current;
    }
    previous[right_chars.len()]
}

fn map_observation_error(error: WorkspaceObservationError) -> KernelError {
    KernelError::ObservationValidation {
        message: error.to_string(),
    }
}

fn classify_capture_error(error: &KernelError) -> (ObservationCaptureErrorClass, Option<&'static str>) {
    match error {
        KernelError::WindowsIntegration { .. } => {
            (ObservationCaptureErrorClass::WindowsIntegration, Some("win32"))
        }
        KernelError::Database(_) => (ObservationCaptureErrorClass::Persistence, Some("sqlite")),
        KernelError::ObservationValidation { message }
            if message.contains("validation")
                || message.contains("window_count")
                || message.contains("focused")
                || message.contains("monitor")
                || message.contains("NumericOverflow")
                || message.contains("overflow") =>
        {
            (ObservationCaptureErrorClass::Validation, Some("observation"))
        }
        KernelError::Config(message)
            if message.contains("validation")
                || message.contains("window_count")
                || message.contains("focused")
                || message.contains("monitor")
                || message.contains("NumericOverflow")
                || message.contains("overflow") =>
        {
            // Legacy config-mapped observation failures remain classified as validation.
            (ObservationCaptureErrorClass::Validation, Some("observation"))
        }
        KernelError::Internal { .. } | KernelError::IntegrityViolation { .. } => {
            (ObservationCaptureErrorClass::Internal, None)
        }
        _ => (ObservationCaptureErrorClass::Internal, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_database::ObservationWindowIdentityRepository;
    use workspace_windows_integration::StubDesktopCapturer;

    fn in_memory_db() -> Arc<Mutex<Database>> {
        let db = Database::open_in_memory().unwrap();
        let runner =
            workspace_database::MigrationRunner::load_from_dir(workspace_database::bundled_migrations_dir())
                .unwrap();
        runner.apply_all(&db).unwrap();
        Arc::new(Mutex::new(db))
    }

    fn sample_window(
        hwnd: &str,
        title: &str,
        process_id: u32,
        focused: bool,
    ) -> CapturedDesktopWindow {
        CapturedDesktopWindow {
            hwnd: hwnd.into(),
            title: title.into(),
            process_id,
            process_name: Some(format!("proc-{process_id}.exe")),
            visible: true,
            minimized: false,
            focused,
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            monitor_index: Some(0),
            z_order: Some(0),
        }
    }

    #[test]
    fn high_confidence_matches_exact_title_across_hwnd_change() {
        let (identity, stable_id) = reconcile_identity(
            &sample_window("0xNEW", "Fixture Focus", 100, true),
            &[ObservationWindowIdentity {
                id: "identity-1".into(),
                process_id: 100,
                title_fingerprint: "fixture focus".into(),
                first_seen_at: "2026-07-26T10:00:00Z".into(),
                last_seen_at: "2026-07-26T10:00:00Z".into(),
                last_hwnd: "0xOLD".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            }],
            "2026-07-26T11:00:00Z",
            &[],
            &HashSet::new(),
        )
        .unwrap();
        let identity = identity.expect("matched");
        assert_eq!(stable_id.as_deref(), Some("identity-1"));
        assert_eq!(identity.confidence, WindowIdentityConfidence::High);
        assert_eq!(identity.last_hwnd, "0xNEW");
    }

    #[test]
    fn low_confidence_requires_hwnd_continuity_not_process_alone() {
        let (identity, _) = reconcile_identity(
            &sample_window("0xSAME", "Different Title", 200, false),
            &[ObservationWindowIdentity {
                id: "identity-2".into(),
                process_id: 200,
                title_fingerprint: "other".into(),
                first_seen_at: "2026-07-26T10:00:00Z".into(),
                last_seen_at: "2026-07-26T10:00:00Z".into(),
                last_hwnd: "0xSAME".into(),
                confidence: WindowIdentityConfidence::Low,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            }],
            "2026-07-26T11:00:00Z",
            &[],
            &HashSet::new(),
        )
        .unwrap();
        assert_eq!(
            identity.expect("matched").confidence,
            WindowIdentityConfidence::Low
        );
    }

    #[test]
    fn same_process_different_titles_create_distinct_identities() {
        let existing = Vec::new();
        let monitors = HashMap::from([(0, "mon".into())]);
        let (identities, windows) = reconcile_windows(
            "pass",
            &[
                sample_window("0xA", "Google", 5000, true),
                sample_window("0xB", "Settings", 5000, false),
            ],
            &existing,
            "2026-07-26T11:00:00Z",
            &monitors,
        )
        .unwrap();
        assert_eq!(identities.len(), 2);
        assert_ne!(
            windows[0].stable_window_id.as_deref(),
            windows[1].stable_window_id.as_deref()
        );
        assert_eq!(
            identities[0].confidence,
            WindowIdentityConfidence::Ephemeral
        );
        assert_eq!(
            identities[1].confidence,
            WindowIdentityConfidence::Ephemeral
        );
    }

    #[test]
    fn pid_reuse_with_unrelated_title_is_not_high_confidence() {
        let (identity, stable_id) = reconcile_identity(
            &sample_window("0xNEW", "Completely Different App", 5000, false),
            &[ObservationWindowIdentity {
                id: "stale".into(),
                process_id: 5000,
                title_fingerprint: "old browser tab".into(),
                first_seen_at: "2026-07-01T10:00:00Z".into(),
                last_seen_at: "2026-07-01T10:00:00Z".into(),
                last_hwnd: "0xOLD".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            }],
            "2026-07-26T11:00:00Z",
            &[],
            &HashSet::new(),
        )
        .unwrap();
        let identity = identity.expect("new identity");
        assert_ne!(stable_id.as_deref(), Some("stale"));
        assert_eq!(identity.confidence, WindowIdentityConfidence::Ephemeral);
    }

    #[test]
    fn new_window_creates_ephemeral_identity() {
        let (identity, _) = reconcile_identity(
            &sample_window("0x3", "Brand New", 999, false),
            &[],
            "2026-07-26T11:00:00Z",
            &[],
            &HashSet::new(),
        )
        .unwrap();
        assert_eq!(
            identity.expect("new").confidence,
            WindowIdentityConfidence::Ephemeral
        );
    }

    #[test]
    fn capture_persists_fixture_snapshot() {
        let db = in_memory_db();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let result = WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();

        assert!(!result.snapshot_id.is_empty());
        assert_eq!(result.window_count, 4);
        assert_eq!(result.monitor_count, 2);
        assert!(result.identity_count >= 4);

        let latest = WorkspaceObservationService::get_latest(&db, &actor, &intent)
            .unwrap()
            .expect("latest");
        assert_eq!(latest.pass.id, result.snapshot_id);
        assert_eq!(latest.windows.len(), 4);
        assert_eq!(latest.monitors.len(), 2);
        assert!(latest.identities.is_empty());
        assert!(latest
            .windows
            .iter()
            .all(|window| window.process_name.is_some()));
        assert!(latest
            .windows
            .iter()
            .all(|window| window.stable_window_id.is_some()));
    }

    #[test]
    fn foreground_focus_persists_in_snapshot() {
        let db = in_memory_db();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let result = WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        let focused = result
            .snapshot
            .windows
            .iter()
            .find(|window| window.focused)
            .expect("focused window");
        assert_eq!(
            result.snapshot.pass.foreground_hwnd.as_deref(),
            Some(focused.hwnd.as_str())
        );
    }

    #[test]
    fn historical_snapshot_integrity_after_identity_mutation() {
        let db = in_memory_db();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        let first = WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        let first_id = first.snapshot_id.clone();
        let first_stable = first.snapshot.windows[0]
            .stable_window_id
            .clone()
            .expect("stable id");

        let mut second_fixture = workspace_windows_integration::dual_monitor_fixture();
        second_fixture.windows[0].title = "Fixture Focus Renamed A Lot".into();
        second_fixture.windows[0].hwnd = "0xCHANGEDHWND".into();
        second_fixture.foreground_hwnd = Some("0xCHANGEDHWND".into());
        WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::new(second_fixture),
        )
        .unwrap();

        let historical = WorkspaceObservationService::get_by_id(&db, &actor, &intent, first_id)
            .unwrap()
            .expect("historical");
        assert!(historical.identities.is_empty());
        assert_eq!(
            historical.windows[0].stable_window_id.as_deref(),
            Some(first_stable.as_str())
        );
    }

    #[test]
    fn scoped_lookup_ignores_unrelated_identity_table() {
        let db = in_memory_db();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        {
            let guard = db.lock().unwrap();
            for index in 0..40 {
                let identity = ObservationWindowIdentity {
                    id: format!("noise-{index}"),
                    process_id: 10_000 + index,
                    title_fingerprint: format!("noise title {index}"),
                    first_seen_at: "2026-07-01T00:00:00Z".into(),
                    last_seen_at: "2026-07-01T00:00:00Z".into(),
                    last_hwnd: format!("0xNOISE{index:04}"),
                    confidence: WindowIdentityConfidence::Ephemeral,
                    authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
                };
                ObservationWindowIdentityRepository::new(&guard)
                    .upsert_all(&[identity])
                    .unwrap();
            }
        }

        let result = WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        assert_eq!(result.window_count, 4);
        assert_eq!(result.identity_count, 4);
    }

    #[test]
    fn identity_retention_keeps_referenced_removes_orphans() {
        let db = in_memory_db();
        let actor = ActorContext::local_user();
        let intent = IntentContext::user_request();
        WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        {
            let guard = db.lock().unwrap();
            ObservationWindowIdentityRepository::new(&guard)
                .upsert_all(&[ObservationWindowIdentity {
                    id: "orphan-identity".into(),
                    process_id: 42,
                    title_fingerprint: "orphan".into(),
                    first_seen_at: "2026-01-01T00:00:00Z".into(),
                    last_seen_at: "2026-01-01T00:00:00Z".into(),
                    last_hwnd: "0xORPHAN".into(),
                    confidence: WindowIdentityConfidence::Ephemeral,
                    authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
                }])
                .unwrap();
        }
        WorkspaceObservationService::capture_with(
            &db,
            &actor,
            &intent,
            &StubDesktopCapturer::fixture_dual_monitor(),
        )
        .unwrap();
        let guard = db.lock().unwrap();
        let identities = ObservationWindowIdentityRepository::new(&guard)
            .list_all()
            .unwrap();
        assert!(!identities.iter().any(|identity| identity.id == "orphan-identity"));
        assert!(!identities.is_empty());
    }

    #[test]
    fn numeric_overflow_fails_explicitly() {
        let error = observation_u64_to_i32(u64::from(u32::MAX) + 1, "duration_ms").unwrap_err();
        assert!(matches!(
            error,
            WorkspaceObservationError::NumericOverflow(_)
        ));
    }
}
