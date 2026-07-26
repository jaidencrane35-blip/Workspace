//! Workspace Observation Layer — durable desktop perception (Phase 6 / Sprint 105).
//!
//! Captures OS desktop state, reconciles window identities, persists snapshots.
//! Never executes, matches applications, or grants authority.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use workspace_database::{Database, ObservationPassRepository, ObservationWindowIdentityRepository};
use workspace_domain::{
    observation_now_rfc3339, IntentContext, ObservationWindowIdentity, ObservedMonitor,
    ObservedWindow,     WindowIdentityConfidence, WorkspaceObservationPass,
    WorkspaceObservationSnapshot,
};
use workspace_windows_integration::{
    platform_desktop_capturer, CapturedDesktopMonitor, CapturedDesktopWindow,
    DesktopCapturer, DesktopObservationCapture,
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
        let capture = capturer.capture_desktop().map_err(|error| KernelError::WindowsIntegration {
            message: error.to_string(),
        })?;
        Self::persist_capture(db, actor, intent, capture)
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
        WorkspaceObservationSnapshot::attempt_execute()
            .map_err(|error| KernelError::Config(error.to_string()))
    }

    fn persist_capture(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intent: &IntentContext,
        capture: DesktopObservationCapture,
    ) -> Result<WorkspaceObservationCaptureResult> {
        let captured_at = observation_now_rfc3339();
        let existing = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            ObservationWindowIdentityRepository::new(&guard).list_all()?
        };
        let snapshot = build_snapshot(&capture, &existing, &captured_at)?;
        {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let repo = ObservationPassRepository::new(&guard);
            repo.insert_snapshot(&snapshot)?;
            repo.purge_older_than_keep(DEFAULT_PASS_RETENTION)?;
        }
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

fn build_snapshot(
    capture: &DesktopObservationCapture,
    existing: &[ObservationWindowIdentity],
    captured_at: &str,
) -> Result<WorkspaceObservationSnapshot> {
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
    );

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
            window_count: windows.len() as i32,
            monitor_count: monitors.len() as i32,
            duration_ms: capture.metadata.duration_ms.map(|value| value as i32),
            metadata_json,
            authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
        },
        windows,
        monitors,
        identities,
        authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
    };
    snapshot.validate()
        .map_err(|error| KernelError::Config(error.to_string()))?;
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
) -> (Vec<ObservationWindowIdentity>, Vec<ObservedWindow>) {
    let mut identities: Vec<ObservationWindowIdentity> = Vec::new();
    let mut observed: Vec<ObservedWindow> = Vec::new();

    for window in windows {
        let (identity, stable_id) =
            reconcile_identity(window, existing, captured_at, &mut identities);
        if let Some(identity) = identity {
            upsert_identity(&mut identities, identity);
        }
        observed.push(map_window(pass_id, window, stable_id, monitor_ids));
    }

    (identities, observed)
}

fn reconcile_identity(
    window: &CapturedDesktopWindow,
    existing: &[ObservationWindowIdentity],
    captured_at: &str,
    pending: &mut Vec<ObservationWindowIdentity>,
) -> (Option<ObservationWindowIdentity>, Option<String>) {
    let fingerprint = title_fingerprint(&window.title);
    let process_id = window.process_id as i32;

    if let Some(identity) = find_high_confidence(existing, pending, process_id, &fingerprint) {
        let updated = refresh_identity(identity, window, captured_at, WindowIdentityConfidence::High);
        return (Some(updated.clone()), Some(updated.id));
    }

    if let Some(identity) =
        find_medium_confidence(existing, pending, process_id, &window.title, &fingerprint)
    {
        let updated =
            refresh_identity(identity, window, captured_at, WindowIdentityConfidence::Medium);
        return (Some(updated.clone()), Some(updated.id));
    }

    if let Some(identity) = find_low_confidence(existing, pending, process_id) {
        let updated = refresh_identity(identity, window, captured_at, WindowIdentityConfidence::Low);
        return (Some(updated.clone()), Some(updated.id));
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
    (Some(identity.clone()), Some(identity.id))
}

fn find_high_confidence<'a>(
    existing: &'a [ObservationWindowIdentity],
    pending: &'a [ObservationWindowIdentity],
    process_id: i32,
    fingerprint: &str,
) -> Option<&'a ObservationWindowIdentity> {
    existing
        .iter()
        .chain(pending.iter())
        .find(|identity| {
            identity.process_id == process_id && identity.title_fingerprint == fingerprint
        })
}

fn find_medium_confidence<'a>(
    existing: &'a [ObservationWindowIdentity],
    pending: &'a [ObservationWindowIdentity],
    process_id: i32,
    title: &str,
    fingerprint: &str,
) -> Option<&'a ObservationWindowIdentity> {
    existing
        .iter()
        .chain(pending.iter())
        .find(|identity| {
            identity.process_id == process_id
                && identity.title_fingerprint != fingerprint
                && titles_are_close(title, &identity.title_fingerprint)
        })
}

fn find_low_confidence<'a>(
    existing: &'a [ObservationWindowIdentity],
    pending: &'a [ObservationWindowIdentity],
    process_id: i32,
) -> Option<&'a ObservationWindowIdentity> {
    existing
        .iter()
        .chain(pending.iter())
        .filter(|identity| identity.process_id == process_id)
        .max_by(|left, right| left.last_seen_at.cmp(&right.last_seen_at))
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
        title_fingerprint: identity.title_fingerprint.clone(),
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
) -> ObservedWindow {
    ObservedWindow {
        id: Uuid::new_v4().to_string(),
        pass_id: pass_id.into(),
        hwnd: window.hwnd.clone(),
        stable_window_id,
        title: window.title.clone(),
        process_id: window.process_id as i32,
        process_name: None,
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
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_windows_integration::StubDesktopCapturer;

    fn in_memory_db() -> Arc<Mutex<Database>> {
        let db = Database::open_in_memory().unwrap();
        let runner =
            workspace_database::MigrationRunner::load_from_dir(workspace_database::bundled_migrations_dir())
                .unwrap();
        runner.apply_all(&db).unwrap();
        Arc::new(Mutex::new(db))
    }

    #[test]
    fn high_confidence_matches_exact_title() {
        let confidence = reconcile_identity(
            &CapturedDesktopWindow {
                hwnd: "0x1".into(),
                title: "Fixture Focus".into(),
                process_id: 100,
                visible: true,
                minimized: false,
                focused: true,
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                monitor_index: Some(0),
                z_order: Some(0),
            },
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
            &mut Vec::new(),
        );
        let (Some(identity), Some(stable_id)) = confidence else {
            panic!("expected matched identity");
        };
        assert_eq!(stable_id, "identity-1");
        assert_eq!(identity.confidence, WindowIdentityConfidence::High);
    }

    #[test]
    fn low_confidence_matches_process_only() {
        let confidence = reconcile_identity(
            &CapturedDesktopWindow {
                hwnd: "0x2".into(),
                title: "Different Title".into(),
                process_id: 200,
                visible: true,
                minimized: false,
                focused: false,
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                monitor_index: None,
                z_order: None,
            },
            &[ObservationWindowIdentity {
                id: "identity-2".into(),
                process_id: 200,
                title_fingerprint: "other".into(),
                first_seen_at: "2026-07-26T10:00:00Z".into(),
                last_seen_at: "2026-07-26T10:00:00Z".into(),
                last_hwnd: "0xOLD".into(),
                confidence: WindowIdentityConfidence::Low,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            }],
            "2026-07-26T11:00:00Z",
            &mut Vec::new(),
        );
        let (Some(identity), _) = confidence else {
            panic!("expected matched identity");
        };
        assert_eq!(identity.confidence, WindowIdentityConfidence::Low);
    }

    #[test]
    fn new_window_creates_ephemeral_identity() {
        let confidence = reconcile_identity(
            &CapturedDesktopWindow {
                hwnd: "0x3".into(),
                title: "Brand New".into(),
                process_id: 999,
                visible: true,
                minimized: false,
                focused: false,
                x: 0,
                y: 0,
                width: 100,
                height: 100,
                monitor_index: None,
                z_order: None,
            },
            &[],
            "2026-07-26T11:00:00Z",
            &mut Vec::new(),
        );
        let (Some(identity), _) = confidence else {
            panic!("expected new identity");
        };
        assert_eq!(identity.confidence, WindowIdentityConfidence::Ephemeral);
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
}
