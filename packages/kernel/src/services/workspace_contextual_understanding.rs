//! Contextual Workspace Understanding — Programme III Batch 6.
//!
//! Transform durable evidence into situational context.
//! Organises situational meaning — does not decide, predict, simulate, or change reality.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceContextualUnderstandingRepository};
use workspace_domain::{
    ActorContext, ContextFrame, ContextualUnderstandingProjection, ContextualUnderstandingStatus,
    ContextualWorkspaceSnapshot, WorkspaceContextExplanation,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceExplanationService,
    WorkspaceHistoricalReconstructionService, WorkspaceStateCompositionService,
    WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceContextualUnderstandingService;

impl WorkspaceContextualUnderstandingService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: ContextFrame,
    ) -> Result<ContextualUnderstandingProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceContextualUnderstandingRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.contextual_understanding.generated",
            &snap.understanding_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "theme_count": snap.themes.len(),
                "gap_count": snap.gaps.len(),
                "source_revision_count": snap.source_revisions.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<ContextualUnderstandingProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceContextualUnderstandingRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == ContextualUnderstandingStatus::Current);
        Ok(ContextualUnderstandingProjection::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn explain(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceContextExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(WorkspaceContextExplanation::from_snapshot(&artefact)),
            None => Ok(WorkspaceContextExplanation {
                explanation_id: "context_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                situation_summary: None,
                theme_summaries: vec![],
                gaps: vec![
                    "No contextual understanding present — Missing context = Unknown".into(),
                ],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing contextual evidence — never invent themes or resolve conflicts"
                        .into(),
                ],
                narrative: "No contextual understanding artefact is available for this workspace."
                    .into(),
                limitations: vec![
                    "Contextual understanding organises situational meaning — it does not change reality"
                        .into(),
                ],
                authority_effect: WorkspaceContextExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot approve policies or authorize actions".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_mutate_intent() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot mutate Intent".into(),
        })
    }

    pub(crate) fn attempt_mutate_task_graph() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot mutate Task Graph".into(),
        })
    }

    pub(crate) fn attempt_alter_workspace_state_envelope() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot alter Workspace State Envelope".into(),
        })
    }

    pub(crate) fn attempt_convert_insight_to_recommendation() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot convert insight into recommendation".into(),
        })
    }

    pub(crate) fn attempt_invent_causal_explanations() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot invent causal explanations".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::ContextualUnderstandingValidation {
            message: "Contextual understanding cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: ContextFrame,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceContextualUnderstandingRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced contextual understanding transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::ContextualUnderstandingValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        frame: ContextFrame,
    ) -> Result<ContextualWorkspaceSnapshot> {
        // Evidence: load_snapshot only — never generate.
        let state = WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?;
        let policy = PolicyGovernanceService::load_snapshot(db, workspace_id)?;
        let reconstruction =
            WorkspaceHistoricalReconstructionService::load_snapshot(db, workspace_id)?;
        let temporal = WorkspaceTemporalIntelligenceService::load_snapshot(db, workspace_id)?;
        let explanation = WorkspaceExplanationService::load_snapshot(db, workspace_id)?;
        let snap = ContextualWorkspaceSnapshot::compose(
            workspace_id,
            now,
            frame,
            Some(&state),
            Some(&policy),
            Some(&reconstruction),
            Some(&temporal),
            Some(&explanation),
        )
        .map_err(KernelError::from)?;
        snap.validate().map_err(KernelError::from)?;
        Ok(snap)
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
