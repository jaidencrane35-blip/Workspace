//! Workspace Semantic Query Engine — Programme IV Batch 1.
//!
//! Retrieve meaning. Never create meaning.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceSemanticQueryRepository};
use workspace_domain::{
    ActorContext, SemanticQuery, SemanticQueryStatus, WorkspaceSemanticQueryExplanation,
    WorkspaceSemanticQueryProjection, WorkspaceSemanticQuerySnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceCrossIntelligenceService, WorkspaceDecisionSupportService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceInsightCoordinationService, WorkspaceIntelligenceHubService,
    WorkspaceKnowledgeIntegrationService, WorkspaceKnowledgeSynthesisService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceSemanticQueryService;

impl WorkspaceSemanticQueryService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        query: SemanticQuery,
    ) -> Result<WorkspaceSemanticQueryProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, query)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceSemanticQueryRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.semantic_query.generated",
            &snap.query_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "match_count": snap.result.match_count,
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceSemanticQueryProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceSemanticQueryRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == SemanticQueryStatus::Current);
        Ok(WorkspaceSemanticQueryProjection::assemble(
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
    ) -> Result<WorkspaceSemanticQueryExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(WorkspaceSemanticQueryExplanation::from_snapshot(&artefact)),
            None => Ok(WorkspaceSemanticQueryExplanation {
                explanation_id: "semantic_query_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                narrative_summary: None,
                match_summaries: vec![],
                gap_summaries: vec![
                    "No semantic query retrieval present — Missing source = Missing".into(),
                ],
                lineage_summaries: vec![],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing semantic query evidence — never invent matches or lineage".into(),
                ],
                narrative: "No semantic query artefact is available for this workspace.".into(),
                limitations: vec![
                    "Workspace Semantic Query Engine retrieves existing evidence only — it never creates meaning"
                        .into(),
                    "Missing evidence remains missing".into(),
                ],
                authority_effect: WorkspaceSemanticQueryExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot execute or apply actions".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_reason() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot perform reasoning".into(),
        })
    }

    pub(crate) fn attempt_plan() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot plan".into(),
        })
    }

    pub(crate) fn attempt_invent_matches() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot invent matches — retrieve existing evidence only".into(),
        })
    }

    pub(crate) fn attempt_fabricate_relevance() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot fabricate relevance".into(),
        })
    }

    pub(crate) fn attempt_invent_lineage() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot invent lineage — lineage ≠ inferred provenance".into(),
        })
    }

    pub(crate) fn attempt_infer_absent_evidence() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot infer absent evidence".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::SemanticQueryValidation {
            message: "Semantic query cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        query: SemanticQuery,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, query)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceSemanticQueryRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced semantic query transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::SemanticQueryValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        query: SemanticQuery,
    ) -> Result<WorkspaceSemanticQuerySnapshot> {
        let state = WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?;
        let policy = PolicyGovernanceService::load_snapshot(db, workspace_id)?;
        let reconstruction =
            WorkspaceHistoricalReconstructionService::load_snapshot(db, workspace_id)?;
        let temporal = WorkspaceTemporalIntelligenceService::load_snapshot(db, workspace_id)?;
        let explanation = WorkspaceExplanationService::load_snapshot(db, workspace_id)?;
        let contextual =
            WorkspaceContextualUnderstandingService::load_snapshot(db, workspace_id)?;
        let knowledge = WorkspaceKnowledgeSynthesisService::load_snapshot(db, workspace_id)?;
        let integration =
            WorkspaceKnowledgeIntegrationService::load_snapshot(db, workspace_id)?;
        let insight = WorkspaceInsightCoordinationService::load_snapshot(db, workspace_id)?;
        let decision_support = WorkspaceDecisionSupportService::load_snapshot(db, workspace_id)?;
        let intelligence_hub = WorkspaceIntelligenceHubService::load_snapshot(db, workspace_id)?;
        let cross_workspace = if query.scope.include_cross_workspace {
            Some(WorkspaceCrossIntelligenceService::load_snapshot(db)?)
        } else {
            None
        };
        let snap = WorkspaceSemanticQuerySnapshot::compose(
            workspace_id,
            now,
            query,
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
            Some(&intelligence_hub),
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
