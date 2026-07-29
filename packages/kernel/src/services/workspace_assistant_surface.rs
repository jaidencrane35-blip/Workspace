//! Workspace Assistant Surface - Programme IV Batch 11.
//!
//! Conversational composition over durable snapshots only; never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceAssistantSurfaceRepository};
use workspace_domain::{
    ActorContext, AssistantSurfaceScope, AssistantSurfaceStatus,
    WorkspaceAssistantSurfaceExplanation, WorkspaceAssistantSurfaceProjection,
    WorkspaceAssistantSurfaceSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceContextualUnderstandingService, WorkspaceEvidenceCompletenessService,
    WorkspaceEvidenceConsistencyService, WorkspaceEvidenceCoverageService,
    WorkspaceEvidenceDependencyService, WorkspaceEvidenceFreshnessService,
    WorkspaceEvidenceNavigationService, WorkspaceEvidenceReliabilityService,
    WorkspaceEvidenceTraceService, WorkspaceExplanationService, WorkspaceIntelligenceHubService,
    WorkspaceKnowledgeIntegrationService, WorkspaceSemanticQueryService,
    WorkspaceStateCompositionService,
};

pub(crate) struct WorkspaceAssistantSurfaceService;

impl WorkspaceAssistantSurfaceService {
    pub(crate) fn compose_turn(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantSurfaceProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, human_ask.into(), scope)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceAssistantSurfaceRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.assistant.turn.composed",
            &snap.surface_id,
            json!({
                "workspace_id": workspace_id,
                "citation_count": snap.utterance.citations.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceAssistantSurfaceProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceAssistantSurfaceRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == AssistantSurfaceStatus::Current);
        Ok(WorkspaceAssistantSurfaceProjection::assemble(
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
    ) -> Result<WorkspaceAssistantSurfaceExplanation> {
        let projection = Self::load_snapshot(db, workspace_id)?;
        match projection.current {
            Some(snap) => Ok(WorkspaceAssistantSurfaceExplanation::from_snapshot(&snap)),
            None => Ok(WorkspaceAssistantSurfaceExplanation {
                explanation_id: "assistant_surface_explanation:missing".into(),
                workspace_id: projection.workspace_id,
                narrative_summary: None,
                citation_summaries: vec![],
                gap_summaries: vec![
                    "No assistant surface turn is available for this workspace".into()
                ],
                lineage_summaries: vec![],
                uncertainty: vec!["Missing assistant turn remains missing".into()],
                narrative: "No assistant surface artefact is available for this workspace.".into(),
                limitations: vec![
                    "Assistant surface presents recorded evidence only".into(),
                    "It never creates hidden memory, permission, execution, or decisions".into(),
                ],
                authority_effect: "none".into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        validation_error("Assistant surface cannot execute actions")
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        validation_error("Assistant surface cannot approve anything")
    }

    pub(crate) fn attempt_decide() -> Result<()> {
        validation_error("Assistant surface cannot decide outcomes")
    }

    pub(crate) fn attempt_bypass_permission_gateway() -> Result<()> {
        validation_error("Assistant surface cannot bypass permission governance")
    }

    pub(crate) fn attempt_create_hidden_memory() -> Result<()> {
        validation_error("Assistant surface cannot create hidden memory")
    }

    pub(crate) fn attempt_silent_mutate() -> Result<()> {
        validation_error("Assistant surface cannot silently mutate state")
    }

    pub(crate) fn attempt_replace_decision_engine() -> Result<()> {
        validation_error("Assistant surface cannot replace decision ownership")
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        validation_error("Assistant surface cannot emit commands")
    }

    pub(crate) fn compose_turn_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, human_ask.into(), scope)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceAssistantSurfaceRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced assistant surface transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => validation_error("expected forced rollback"),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        human_ask: String,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantSurfaceSnapshot> {
        let semantic_query = if scope.include_semantic_query {
            Some(WorkspaceSemanticQueryService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_navigation = if scope.include_evidence_navigation {
            Some(WorkspaceEvidenceNavigationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_trace = if scope.include_evidence_trace {
            Some(WorkspaceEvidenceTraceService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_coverage = if scope.include_evidence_coverage {
            Some(WorkspaceEvidenceCoverageService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_consistency = if scope.include_evidence_consistency {
            Some(WorkspaceEvidenceConsistencyService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_dependency = if scope.include_evidence_dependency {
            Some(WorkspaceEvidenceDependencyService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_freshness = if scope.include_evidence_freshness {
            Some(WorkspaceEvidenceFreshnessService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_completeness = if scope.include_evidence_completeness {
            Some(WorkspaceEvidenceCompletenessService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let evidence_reliability = if scope.include_evidence_reliability {
            Some(WorkspaceEvidenceReliabilityService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let explanation = if scope.include_explanation {
            Some(WorkspaceExplanationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let contextual = if scope.include_contextual {
            Some(WorkspaceContextualUnderstandingService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let knowledge_integration = if scope.include_knowledge_integration {
            Some(WorkspaceKnowledgeIntegrationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let intelligence_hub = if scope.include_intelligence_hub {
            Some(WorkspaceIntelligenceHubService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let state = if scope.include_state {
            Some(WorkspaceStateCompositionService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };

        let snap = WorkspaceAssistantSurfaceSnapshot::compose(
            workspace_id,
            now,
            human_ask,
            scope,
            semantic_query.as_ref(),
            evidence_navigation.as_ref(),
            evidence_trace.as_ref(),
            evidence_coverage.as_ref(),
            evidence_consistency.as_ref(),
            evidence_dependency.as_ref(),
            evidence_freshness.as_ref(),
            evidence_completeness.as_ref(),
            evidence_reliability.as_ref(),
            explanation.as_ref(),
            contextual.as_ref(),
            knowledge_integration.as_ref(),
            intelligence_hub.as_ref(),
            state.as_ref(),
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

fn validation_error<T>(message: impl Into<String>) -> Result<T> {
    Err(KernelError::AssistantSurfaceValidation {
        message: message.into(),
    })
}
