//! Workspace Insight Coordination — Programme III Batch 9.
//!
//! Coordinate understanding. Never create authority.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, InsightCoordinationRepository};
use workspace_domain::{
    ActorContext, InsightCoordinationExplanation, InsightCoordinationFrame,
    InsightCoordinationProjection, InsightCoordinationSnapshot, InsightCoordinationStatus,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceKnowledgeIntegrationService, WorkspaceKnowledgeSynthesisService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceInsightCoordinationService;

impl WorkspaceInsightCoordinationService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: InsightCoordinationFrame,
    ) -> Result<InsightCoordinationProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = InsightCoordinationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.insight_coordination.generated",
            &snap.coordination_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "cluster_count": snap.clusters.len(),
                "intersection_count": snap.intersections.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<InsightCoordinationProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = InsightCoordinationRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == InsightCoordinationStatus::Current);
        Ok(InsightCoordinationProjection::assemble(
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
    ) -> Result<InsightCoordinationExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(InsightCoordinationExplanation::from_snapshot(&artefact)),
            None => Ok(InsightCoordinationExplanation {
                explanation_id: "insight_coordination_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                summary: None,
                cluster_summaries: vec![],
                intersection_summaries: vec![],
                attention_summaries: vec![],
                gaps: vec![
                    "No insight coordination present — Missing source = Missing".into(),
                ],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing coordination evidence — never invent clusters or intersections".into(),
                ],
                narrative:
                    "No insight coordination artefact is available for this workspace.".into(),
                limitations: vec![
                    "Insight Coordination coordinates understanding only — it does not create authority"
                        .into(),
                ],
                authority_effect: InsightCoordinationExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot mutate lifecycle".into(),
        })
    }

    pub(crate) fn attempt_grant_permissions() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot grant permissions".into(),
        })
    }

    pub(crate) fn attempt_approve_policy() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot approve policies".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_become_memory_or_cognitive_model() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot become Memory or Cognitive Model".into(),
        })
    }

    pub(crate) fn attempt_invent_missing_evidence() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot invent missing evidence".into(),
        })
    }

    pub(crate) fn attempt_convert_correlation_to_causation() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot convert correlation into causation".into(),
        })
    }

    pub(crate) fn attempt_convert_prioritisation_to_action() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot convert prioritisation into action".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::InsightCoordinationValidation {
            message: "Insight coordination cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: InsightCoordinationFrame,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = InsightCoordinationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced insight coordination transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::InsightCoordinationValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        frame: InsightCoordinationFrame,
    ) -> Result<InsightCoordinationSnapshot> {
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
        let snap = InsightCoordinationSnapshot::compose(
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
