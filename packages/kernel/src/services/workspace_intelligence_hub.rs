//! Workspace Intelligence Hub — Programme III Batch 12.
//!
//! Aggregate intelligence. Never replace the intelligence that produced it.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceIntelligenceHubRepository};
use workspace_domain::{
    ActorContext, IntelligenceHubFrame, WorkspaceIntelligenceHubExplanation,
    WorkspaceIntelligenceHubProjection, WorkspaceIntelligenceHubSnapshot, IntelligenceHubStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceCrossIntelligenceService, WorkspaceDecisionSupportService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceInsightCoordinationService, WorkspaceKnowledgeIntegrationService,
    WorkspaceKnowledgeSynthesisService, WorkspaceStateCompositionService,
    WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceIntelligenceHubService;

impl WorkspaceIntelligenceHubService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: IntelligenceHubFrame,
    ) -> Result<WorkspaceIntelligenceHubProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceIntelligenceHubRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.intelligence_hub.generated",
            &snap.hub_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "package_count": snap.packages.len(),
                "gap_count": snap.gaps.len(),
                "conflict_count": snap.conflicts.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceIntelligenceHubProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceIntelligenceHubRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == IntelligenceHubStatus::Current);
        Ok(WorkspaceIntelligenceHubProjection::assemble(
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
    ) -> Result<WorkspaceIntelligenceHubExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(WorkspaceIntelligenceHubExplanation::from_snapshot(&artefact)),
            None => Ok(WorkspaceIntelligenceHubExplanation {
                explanation_id: "intelligence_hub_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                narrative_summary: None,
                package_summaries: vec![],
                gap_summaries: vec![
                    "No intelligence hub present — Missing source = Missing".into(),
                ],
                conflict_summaries: vec![],
                lineage_summaries: vec![],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing intelligence hub evidence — never invent packages or lineage".into(),
                ],
                narrative: "No intelligence hub artefact is available for this workspace.".into(),
                limitations: vec![
                    "Workspace Intelligence Hub aggregates intelligence only — it never replaces upstream authority"
                        .into(),
                ],
                authority_effect: WorkspaceIntelligenceHubExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot execute or apply actions".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_become_decision_maker() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot become the decision-maker".into(),
        })
    }

    pub(crate) fn attempt_invent_intelligence() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot invent missing intelligence packages".into(),
        })
    }

    pub(crate) fn attempt_resolve_conflicts() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot resolve conflicts — conflict record ≠ resolution".into(),
        })
    }

    pub(crate) fn attempt_fabricate_lineage() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot fabricate lineage — lineage ≠ inferred provenance".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot emit commands".into(),
        })
    }

    pub(crate) fn attempt_mutate_upstream() -> Result<()> {
        Err(KernelError::IntelligenceHubValidation {
            message: "Intelligence hub cannot mutate upstream intelligence sources".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: IntelligenceHubFrame,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceIntelligenceHubRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced intelligence hub transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::IntelligenceHubValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        frame: IntelligenceHubFrame,
    ) -> Result<WorkspaceIntelligenceHubSnapshot> {
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
        let decision_support = WorkspaceDecisionSupportService::load_snapshot(db, workspace_id)?;
        let cross_workspace = if frame.include_cross_workspace {
            Some(WorkspaceCrossIntelligenceService::load_snapshot(db)?)
        } else {
            None
        };
        let snap = WorkspaceIntelligenceHubSnapshot::compose(
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
            Some(&decision_support),
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
