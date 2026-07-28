//! Historical Workspace Reconstruction — Programme III Batch 3.
//!
//! Reconstruction explains change over time. It does not become the source of truth.
//! Reads durable workspace state envelopes only — never generates, replays, or mutates.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, HistoricalReconstructionRepository};
use workspace_domain::{
    ActorContext, HistoricalChangeExplanation, HistoricalReconstructionSnapshot,
    ReconstructionStatus, RevisionComparison, WorkspaceHistoricalView,
};

use crate::error::{KernelError, Result};
use crate::services::{AuditService, WorkspaceStateCompositionService};

pub(crate) struct WorkspaceHistoricalReconstructionService;

impl WorkspaceHistoricalReconstructionService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<HistoricalReconstructionSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = HistoricalReconstructionRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.historical_reconstruction.generated",
            &view.reconstruction_id,
            json!({
                "workspace_id": workspace_id,
                "from_revision": view.from_revision,
                "to_revision": view.to_revision,
                "completeness": view.completeness.as_str(),
                "change_count": view.timeline.len(),
                "gap_count": view.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<HistoricalReconstructionSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = HistoricalReconstructionRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == ReconstructionStatus::Current);
        Ok(HistoricalReconstructionSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn compare_revisions(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        left_revision: impl Into<String>,
        right_revision: impl Into<String>,
    ) -> Result<RevisionComparison> {
        let workspace_id = workspace_id.into();
        let left_revision = left_revision.into();
        let right_revision = right_revision.into();
        // Read-only durable envelopes — never generate to fill gaps.
        let envelopes =
            WorkspaceStateCompositionService::list_durable_envelopes(db, &workspace_id)?;
        let left = envelopes.iter().find(|e| e.revision == left_revision);
        let right = envelopes.iter().find(|e| e.revision == right_revision);
        match (left, right) {
            (Some(l), Some(r)) => Ok(RevisionComparison::compare_envelopes(l, r)),
            _ => Ok(RevisionComparison::unavailable(
                left_revision,
                right_revision,
                "One or both revisions unavailable in durable envelope evidence — No evidence found ≠ Nothing happened",
            )),
        }
    }

    pub(crate) fn explain(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<HistoricalChangeExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(view) => Ok(HistoricalChangeExplanation::from_view(&view)),
            None => Ok(HistoricalChangeExplanation {
                explanation_id: "historical_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                from_revision: None,
                to_revision: None,
                completeness: None,
                observed_changes: vec![],
                evidence_refs: vec![],
                gaps: vec![
                    "No reconstruction present — ReconstructionIncomplete".into(),
                ],
                uncertainty: vec![
                    "Missing reconstruction evidence — never invent transitions".into(),
                ],
                narrative:
                    "No historical reconstruction artefact is available for this workspace."
                        .into(),
                authority_effect: HistoricalChangeExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::HistoricalReconstructionValidation {
            message: "Historical reconstruction cannot execute or replay".into(),
        })
    }

    pub(crate) fn attempt_replay() -> Result<()> {
        Err(KernelError::HistoricalReconstructionValidation {
            message: "Historical reconstruction has no replay authority".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::HistoricalReconstructionValidation {
            message: "Historical reconstruction cannot mutate lifecycle state".into(),
        })
    }

    pub(crate) fn attempt_fabricate_transition() -> Result<()> {
        Err(KernelError::HistoricalReconstructionValidation {
            message: "Historical reconstruction cannot invent unsupported transitions".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::HistoricalReconstructionValidation {
            message: "Historical reconstruction cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = HistoricalReconstructionRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced historical reconstruction transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::HistoricalReconstructionValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<WorkspaceHistoricalView> {
        // Evidence source: durable envelopes via list_durable_envelopes only — never generate.
        let envelopes =
            WorkspaceStateCompositionService::list_durable_envelopes(db, workspace_id)?;
        let view = WorkspaceHistoricalView::reconstruct(workspace_id, now, &envelopes)
            .map_err(KernelError::from)?;
        view.validate().map_err(KernelError::from)?;
        Ok(view)
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let _ = subject_id;
        AuditService::record_ai_planning_event(
            db,
            actor,
            &workspace_domain::IntentContext::user_request(),
            event,
            true,
            metadata.to_string(),
        )
    }
}
