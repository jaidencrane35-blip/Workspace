//! Workspace Assistant Retrieval Intelligence - Programme IV Batch 13.
//!
//! Retrieval packaging over durable snapshots only; never foreign generate;
//! never invent matches; never rank truth or recommend action.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceAssistantRetrievalRepository};
use workspace_domain::{
    ActorContext, AssistantRetrievalStatus, AssistantSurfaceScope,
    WorkspaceAssistantRetrievalExplanation, WorkspaceAssistantRetrievalProjection,
    WorkspaceAssistantRetrievalSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceAssistantContextService, WorkspaceAssistantSurfaceService,
    WorkspaceEvidenceCompletenessService, WorkspaceEvidenceConsistencyService,
    WorkspaceEvidenceCoverageService, WorkspaceEvidenceDependencyService,
    WorkspaceEvidenceFreshnessService, WorkspaceEvidenceNavigationService,
    WorkspaceEvidenceReliabilityService, WorkspaceEvidenceTraceService,
    WorkspaceSemanticQueryService,
};

pub(crate) struct WorkspaceAssistantRetrievalService;

impl WorkspaceAssistantRetrievalService {
    pub(crate) fn package_retrieval(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantRetrievalProjection> {
        let workspace_id = workspace_id.into();
        let human_ask = human_ask.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::package_view(db, &workspace_id, &now, &human_ask, scope)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceAssistantRetrievalRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.assistant.retrieval.packaged",
            &snap.retrieval_id,
            json!({
                "workspace_id": workspace_id,
                "item_count": snap.items.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceAssistantRetrievalProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceAssistantRetrievalRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == AssistantRetrievalStatus::Current);
        Ok(WorkspaceAssistantRetrievalProjection::assemble(
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
    ) -> Result<WorkspaceAssistantRetrievalExplanation> {
        let projection = Self::load_snapshot(db, workspace_id)?;
        match projection.current {
            Some(snap) => Ok(WorkspaceAssistantRetrievalExplanation::from_snapshot(&snap)),
            None => Ok(WorkspaceAssistantRetrievalExplanation {
                explanation_id: "assistant_retrieval_explanation:missing".into(),
                workspace_id: projection.workspace_id,
                narrative_summary: None,
                item_summaries: vec![],
                gap_summaries: vec![
                    "No assistant retrieval package is available for this workspace".into(),
                ],
                lineage_summaries: vec![],
                uncertainty: vec!["Missing retrieval package remains missing".into()],
                narrative: "No assistant retrieval artefact is available for this workspace.".into(),
                limitations: vec![
                    "Assistant retrieval presents recorded evidence only".into(),
                    "It never ranks truth, decides outcomes, runs work, or creates a second search engine".into(),
                ],
                authority_effect: "none".into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_rank_truth() -> Result<()> {
        validation_error("Assistant retrieval cannot rank truth")
    }

    pub(crate) fn attempt_recommend() -> Result<()> {
        validation_error("Assistant retrieval cannot recommend actions")
    }

    pub(crate) fn attempt_decide() -> Result<()> {
        validation_error("Assistant retrieval cannot decide outcomes")
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        validation_error("Assistant retrieval cannot execute actions")
    }

    pub(crate) fn attempt_bypass_permission_gateway() -> Result<()> {
        validation_error("Assistant retrieval cannot bypass permission governance")
    }

    pub(crate) fn attempt_fabricate_results() -> Result<()> {
        validation_error("Assistant retrieval cannot fabricate results")
    }

    pub(crate) fn attempt_create_search_engine() -> Result<()> {
        validation_error("Assistant retrieval cannot create a second search engine")
    }

    pub(crate) fn attempt_hide_uncertainty() -> Result<()> {
        validation_error("Assistant retrieval cannot hide uncertainty")
    }

    pub(crate) fn package_retrieval_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let human_ask = human_ask.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::package_view(db, &workspace_id, &now, &human_ask, scope)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceAssistantRetrievalRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced assistant retrieval transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => validation_error("expected forced rollback"),
        }
    }

    fn package_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        human_ask: &str,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantRetrievalSnapshot> {
        // Batch 12 context via load_snapshot only — continuity/scope input.
        let context = WorkspaceAssistantContextService::load_snapshot(db, workspace_id)?;
        // Batch 11 surface via load_snapshot only — presentation lineage refs.
        let surface = WorkspaceAssistantSurfaceService::load_snapshot(db, workspace_id)?;

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

        let snap = WorkspaceAssistantRetrievalSnapshot::package(
            workspace_id,
            now,
            human_ask,
            scope,
            Some(&context),
            Some(&surface),
            semantic_query.as_ref(),
            evidence_navigation.as_ref(),
            evidence_trace.as_ref(),
            evidence_coverage.as_ref(),
            evidence_consistency.as_ref(),
            evidence_dependency.as_ref(),
            evidence_freshness.as_ref(),
            evidence_completeness.as_ref(),
            evidence_reliability.as_ref(),
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
    Err(KernelError::AssistantRetrievalValidation {
        message: message.into(),
    })
}
