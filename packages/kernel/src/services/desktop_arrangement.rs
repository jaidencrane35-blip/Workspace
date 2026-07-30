//! Governed desktop arrangement capture & restore (DAF-1d).
//!
//! Capture persists observed membership + bounds. Restore plans matches, then
//! applies only through [`WindowController`] after Permission Gateway Allow.
//! Never called from Assistant / intelligence paths.

use std::sync::{Arc, Mutex};

use uuid::Uuid;
use workspace_database::{Database, DesktopArrangementRepository};
use workspace_domain::{
    arrangement_now_rfc3339, capture_entry_inputs_from_snapshot_filtered, entries_from_inputs,
    plan_desktop_arrangement_restore, validate_desktop_arrangement, DesktopArrangement,
    DesktopArrangementApplyOutcome, DesktopArrangementApplyStatus, DesktopArrangementError,
    DesktopArrangementId, DesktopArrangementRestoreResult, DesktopArrangementStatus,
    DesktopWindowFocusResult, WorkspaceId,
};
use workspace_windows_integration::{
    platform_window_controller, FocusWindowRequest, SetWindowBoundsRequest, StubWindowController,
    WindowBounds, WindowController,
};

use crate::error::{KernelError, Result};
use crate::services::WorkspaceObservationService;

/// Kernel orchestration for desktop arrangement capture/restore.
pub(crate) struct DesktopArrangementService;

impl DesktopArrangementService {
    pub(crate) fn capture(
        db: &Arc<Mutex<Database>>,
        workspace_id: &WorkspaceId,
        arrangement_id: Option<DesktopArrangementId>,
        name: String,
        description: String,
        refresh_observation: bool,
        member_hwnds: Option<Vec<String>>,
    ) -> Result<DesktopArrangement> {
        if refresh_observation {
            let actor = workspace_domain::ActorContext::local_user();
            let intent = workspace_domain::IntentContext::user_request();
            let _ = WorkspaceObservationService::capture(db, &actor, &intent)?;
        }

        let snapshot = WorkspaceObservationService::get_latest_snapshot(db)?.ok_or(
            KernelError::DesktopArrangementValidation {
                message: DesktopArrangementError::MissingObservation.to_string(),
            },
        )?;

        let hwnd_filter = member_hwnds.as_deref();
        let inputs = capture_entry_inputs_from_snapshot_filtered(&snapshot, hwnd_filter);
        if member_hwnds.as_ref().is_some_and(|hwnds| !hwnds.is_empty()) && inputs.is_empty()
        {
            return Err(KernelError::DesktopArrangementValidation {
                message: "selected windows were not found in the latest observation".into(),
            });
        }
        let now = arrangement_now_rfc3339();
        let id = match arrangement_id {
            Some(id) => id,
            None => DesktopArrangementId::new(format!("arr-{}", Uuid::new_v4()))
                .map_err(KernelError::Domain)?,
        };

        let (created_at, status) = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let repo = DesktopArrangementRepository::new(&guard);
            match repo.get(&id)? {
                Some(existing) => (existing.created_at, existing.status),
                None => (now.clone(), DesktopArrangementStatus::Active),
            }
        };

        let entries = entries_from_inputs(&id, &inputs, |index| {
            format!("entry-{}-{index}", id.as_str())
        })
        .map_err(map_arrangement_error)?;

        let arrangement = DesktopArrangement {
            id: id.clone(),
            workspace_id: workspace_id.clone(),
            name: name.trim().to_string(),
            description,
            status,
            entries: entries.clone(),
            created_at,
            updated_at: now.clone(),
            authority_effect: DesktopArrangement::AUTHORITY_EFFECT_NONE.into(),
        };
        validate_desktop_arrangement(&arrangement).map_err(map_arrangement_error)?;

        {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            let repo = DesktopArrangementRepository::new(&guard);
            repo.upsert_arrangement(&arrangement)?;
            repo.replace_entries(&id, &entries, &now)?;
        }

        Ok(arrangement)
    }

    pub(crate) fn get(
        db: &Arc<Mutex<Database>>,
        arrangement_id: &DesktopArrangementId,
    ) -> Result<Option<DesktopArrangement>> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(DesktopArrangementRepository::new(&guard).get(arrangement_id)?)
    }

    pub(crate) fn list_by_workspace(
        db: &Arc<Mutex<Database>>,
        workspace_id: &WorkspaceId,
        limit: usize,
    ) -> Result<Vec<DesktopArrangement>> {
        let guard = db.lock().map_err(|_| KernelError::NotReady)?;
        Ok(DesktopArrangementRepository::new(&guard).list_by_workspace(workspace_id, limit)?)
    }

    pub(crate) fn restore(
        db: &Arc<Mutex<Database>>,
        arrangement_id: &DesktopArrangementId,
        focus_first: bool,
    ) -> Result<DesktopArrangementRestoreResult> {
        let controller = platform_window_controller();
        Self::restore_with(db, arrangement_id, focus_first, controller.as_ref())
    }

    /// Test / stub path — no OS mutation.
    pub(crate) fn restore_simulated(
        db: &Arc<Mutex<Database>>,
        arrangement_id: &DesktopArrangementId,
        focus_first: bool,
    ) -> Result<DesktopArrangementRestoreResult> {
        let controller = StubWindowController::new();
        Self::restore_with(db, arrangement_id, focus_first, &controller)
    }

    pub(crate) fn restore_with(
        db: &Arc<Mutex<Database>>,
        arrangement_id: &DesktopArrangementId,
        focus_first: bool,
        controller: &dyn WindowController,
    ) -> Result<DesktopArrangementRestoreResult> {
        let arrangement = {
            let guard = db.lock().map_err(|_| KernelError::NotReady)?;
            DesktopArrangementRepository::new(&guard)
                .get(arrangement_id)?
                .ok_or(KernelError::DesktopArrangementNotFound)?
        };

        let snapshot = WorkspaceObservationService::get_latest_snapshot(db)?.ok_or(
            KernelError::DesktopArrangementValidation {
                message: DesktopArrangementError::MissingObservation.to_string(),
            },
        )?;

        let plan = plan_desktop_arrangement_restore(&arrangement, &snapshot, focus_first);
        let gap_count = plan.diagnostics.iter().filter(|d| d.restore_gap).count();
        let mut outcomes = Vec::new();
        let mut applied_count = 0usize;
        let mut failed_count = 0usize;
        let mut halt = false;

        for action in &plan.actions {
            if halt {
                outcomes.push(DesktopArrangementApplyOutcome {
                    entry_id: action.entry_id.clone(),
                    label: action.label.clone(),
                    hwnd: action.hwnd.clone(),
                    status: DesktopArrangementApplyStatus::Skipped,
                    detail: "Skipped after prior restore failure (no automatic repair).".into(),
                    simulated: false,
                });
                continue;
            }

            let bounds_result = controller.set_bounds(&SetWindowBoundsRequest {
                hwnd: action.hwnd.clone(),
                bounds: WindowBounds {
                    x: action.bounds.x,
                    y: action.bounds.y,
                    width: action.bounds.width,
                    height: action.bounds.height,
                },
            });

            match bounds_result {
                Ok(outcome) => {
                    let mut detail = "set_bounds applied".to_string();
                    let mut simulated = outcome.simulated;
                    if action.focus {
                        match controller.focus(&FocusWindowRequest {
                            hwnd: action.hwnd.clone(),
                        }) {
                            Ok(focus_outcome) => {
                                detail.push_str("; focus applied");
                                simulated = simulated || focus_outcome.simulated;
                            }
                            Err(error) => {
                                detail.push_str(&format!("; focus failed: {error}"));
                            }
                        }
                    }
                    applied_count += 1;
                    outcomes.push(DesktopArrangementApplyOutcome {
                        entry_id: action.entry_id.clone(),
                        label: action.label.clone(),
                        hwnd: action.hwnd.clone(),
                        status: DesktopArrangementApplyStatus::Applied,
                        detail,
                        simulated,
                    });
                }
                Err(error) => {
                    failed_count += 1;
                    halt = true;
                    outcomes.push(DesktopArrangementApplyOutcome {
                        entry_id: action.entry_id.clone(),
                        label: action.label.clone(),
                        hwnd: action.hwnd.clone(),
                        status: DesktopArrangementApplyStatus::Failed,
                        detail: format!("set_bounds failed: {error}"),
                        simulated: false,
                    });
                }
            }
        }

        Ok(DesktopArrangementRestoreResult {
            arrangement_id: arrangement.id.as_str().into(),
            diagnostics: plan.diagnostics,
            outcomes,
            applied_count,
            gap_count,
            failed_count,
        })
    }

    /// Focus one observed window through WindowController (local user / restore capability).
    pub(crate) fn focus_window(hwnd: &str, simulate: bool) -> Result<DesktopWindowFocusResult> {
        let hwnd = hwnd.trim();
        if hwnd.is_empty() {
            return Err(KernelError::DesktopArrangementValidation {
                message: "hwnd is required to focus a desktop window".into(),
            });
        }
        let controller: Box<dyn WindowController> = if simulate {
            Box::new(StubWindowController::new())
        } else {
            platform_window_controller()
        };
        match controller.focus(&FocusWindowRequest {
            hwnd: hwnd.to_string(),
        }) {
            Ok(outcome) => Ok(DesktopWindowFocusResult {
                hwnd: outcome.hwnd,
                simulated: outcome.simulated,
            }),
            Err(error) => Err(KernelError::DesktopArrangementValidation {
                message: format!("focus failed: {error}"),
            }),
        }
    }

    /// Architecture seal — Assistant must not own restore execution.
    pub(crate) fn attempt_assistant_restore() -> Result<()> {
        Err(KernelError::IntegrityViolation {
            message: "desktop arrangement restore cannot execute via Assistant path".into(),
        })
    }
}

fn map_arrangement_error(error: DesktopArrangementError) -> KernelError {
    match error {
        DesktopArrangementError::NotFound(_) => KernelError::DesktopArrangementNotFound,
        DesktopArrangementError::CannotControl => KernelError::IntegrityViolation {
            message: error.to_string(),
        },
        DesktopArrangementError::Domain(domain) => KernelError::Domain(domain),
        other => KernelError::DesktopArrangementValidation {
            message: other.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assistant_restore_path_is_sealed() {
        assert!(matches!(
            DesktopArrangementService::attempt_assistant_restore(),
            Err(KernelError::IntegrityViolation { .. })
        ));
    }
}
