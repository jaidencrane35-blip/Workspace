//! Workspace Decision Support — Programme III Batch 11.
//!
//! Support decisions. Never become the decision-maker.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceDecisionSupportRepository};
use workspace_domain::{
    ActorContext, DecisionSupportFrame, WorkspaceDecisionSupportExplanation,
    WorkspaceDecisionSupportProjection, WorkspaceDecisionSupportSnapshot, DecisionSupportStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceCrossIntelligenceService, WorkspaceExplanationService,
    WorkspaceHistoricalReconstructionService, WorkspaceInsightCoordinationService,
    WorkspaceKnowledgeIntegrationService, WorkspaceKnowledgeSynthesisService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceDecisionSupportService;

impl WorkspaceDecisionSupportService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: DecisionSupportFrame,
    ) -> Result<WorkspaceDecisionSupportProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceDecisionSupportRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.decision_support.generated",
            &snap.support_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "context_count": snap.contexts.len(),
                "bundle_count": snap.evidence_bundles.len(),
                "tradeoff_count": snap.tradeoffs.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceDecisionSupportProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceDecisionSupportRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == DecisionSupportStatus::Current);
        Ok(WorkspaceDecisionSupportProjection::assemble(
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
    ) -> Result<WorkspaceDecisionSupportExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(WorkspaceDecisionSupportExplanation::from_snapshot(&artefact)),
            None => Ok(WorkspaceDecisionSupportExplanation {
                explanation_id: "decision_support_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                summary: None,
                context_summaries: vec![],
                bundle_summaries: vec![],
                tradeoff_summaries: vec![],
                dependency_summaries: vec![],
                gaps: vec![
                    "No decision support present — Missing source = Missing".into(),
                ],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing decision support evidence — never invent contexts or tradeoffs".into(),
                ],
                narrative:
                    "No decision support artefact is available for this workspace.".into(),
                limitations: vec![
                    "Workspace Decision Support organises evidence only — it never becomes the decision-maker"
                        .into(),
                ],
                authority_effect: WorkspaceDecisionSupportExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot execute or apply decisions".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot mutate lifecycle".into(),
        })
    }

    pub(crate) fn attempt_grant_permissions() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot grant permissions".into(),
        })
    }

    pub(crate) fn attempt_approve_policy() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot approve policies".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_make_decision() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot make decisions".into(),
        })
    }

    pub(crate) fn attempt_invent_missing_evidence() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot invent missing evidence".into(),
        })
    }

    pub(crate) fn attempt_convert_tradeoff_to_recommendation() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot convert trade-offs into recommendations".into(),
        })
    }

    pub(crate) fn attempt_convert_comparison_to_ranking() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot convert comparisons into ranking-as-authority".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::DecisionSupportValidation {
            message: "Decision support cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: DecisionSupportFrame,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceDecisionSupportRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced decision support transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::DecisionSupportValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        frame: DecisionSupportFrame,
    ) -> Result<WorkspaceDecisionSupportSnapshot> {
        let state = WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?;
        let policy = PolicyGovernanceService::load_snapshot(db, workspace_id)?;
        let reconstruction =
            WorkspaceHistoricalReconstructionService::load_snapshot(db, workspace_id)?;
        let temporal = WorkspaceTemporalIntelligenceService::load_snapshot(db, workspace_id)?;
        let explanation = WorkspaceExplanationService::load_snapshot(db, workspace_id)?;
        let contextual =
            WorkspaceContextualUnderstandingService::load_snapshot(db, workspace_id)?;
        let knowledge =
            WorkspaceKnowledgeSynthesisService::load_snapshot(db, workspace_id)?;
        let integration =
            WorkspaceKnowledgeIntegrationService::load_snapshot(db, workspace_id)?;
        let insight = WorkspaceInsightCoordinationService::load_snapshot(db, workspace_id)?;
        let cross_workspace = if frame.include_cross_workspace {
            Some(WorkspaceCrossIntelligenceService::load_snapshot(db)?)
        } else {
            None
        };
        let snap = WorkspaceDecisionSupportSnapshot::compose(
            workspace_id,
            now,
            frame,
            Some(&state),
            Some(&policy),
            Some(&reconstruction),
            Some(&temporal),
            Some(&explanation),
            Some(&contextual),
            Some(&knowledge),
            Some(&integration),
            Some(&insight),
            cross_workspace.as_ref(),
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
