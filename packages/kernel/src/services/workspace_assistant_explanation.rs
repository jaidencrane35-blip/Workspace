//! Workspace Assistant Explanation Intelligence - Programme IV Batch 14.
//!
//! Explanation packaging over durable snapshots only; never foreign generate;
//! never invent causality; never conclude truth or recommend action.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceAssistantExplanationRepository};
use workspace_domain::{
    ActorContext, AssistantExplanationStatus, AssistantSurfaceScope,
    WorkspaceAssistantExplanationExplanation, WorkspaceAssistantExplanationProjection,
    WorkspaceAssistantExplanationSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceAssistantContextService, WorkspaceAssistantRetrievalService,
    WorkspaceAssistantSurfaceService, WorkspaceEvidenceConsistencyService,
    WorkspaceEvidenceNavigationService, WorkspaceEvidenceTraceService,
    WorkspaceExplanationService,
};

pub(crate) struct WorkspaceAssistantExplanationService;

impl WorkspaceAssistantExplanationService {
    pub(crate) fn package_explanation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantExplanationProjection> {
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
                    let repo = WorkspaceAssistantExplanationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.assistant.explanation.packaged",
            &snap.explanation_id,
            json!({
                "workspace_id": workspace_id,
                "section_count": snap.sections.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceAssistantExplanationProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceAssistantExplanationRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == AssistantExplanationStatus::Current);
        Ok(WorkspaceAssistantExplanationProjection::assemble(
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
    ) -> Result<WorkspaceAssistantExplanationExplanation> {
        let projection = Self::load_snapshot(db, workspace_id)?;
        match projection.current {
            Some(snap) => Ok(WorkspaceAssistantExplanationExplanation::from_snapshot(&snap)),
            None => Ok(WorkspaceAssistantExplanationExplanation {
                explanation_id: "assistant_explanation_meta:missing".into(),
                workspace_id: projection.workspace_id,
                narrative_summary: None,
                section_summaries: vec![],
                citation_summaries: vec![],
                gap_summaries: vec![
                    "No assistant explanation package is available for this workspace".into(),
                ],
                lineage_summaries: vec![],
                uncertainty: vec!["Missing explanation package remains missing".into()],
                narrative: "No assistant explanation artefact is available for this workspace.".into(),
                limitations: vec![
                    "Assistant explanation clarifies recorded evidence only".into(),
                    "It never concludes truth, invents causality, decides outcomes, or replaces the Explanation Layer".into(),
                ],
                authority_effect: "none".into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_conclude_truth() -> Result<()> {
        validation_error("Assistant explanation cannot conclude truth")
    }

    pub(crate) fn attempt_invent_cause() -> Result<()> {
        validation_error("Assistant explanation cannot invent causal narratives")
    }

    pub(crate) fn attempt_recommend() -> Result<()> {
        validation_error("Assistant explanation cannot advise or recommend actions")
    }

    pub(crate) fn attempt_decide() -> Result<()> {
        validation_error("Assistant explanation cannot decide outcomes")
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        validation_error("Assistant explanation cannot execute actions")
    }

    pub(crate) fn attempt_bypass_permission_gateway() -> Result<()> {
        validation_error("Assistant explanation cannot bypass permission governance")
    }

    pub(crate) fn attempt_replace_explanation_layer() -> Result<()> {
        validation_error("Assistant explanation cannot replace the Workspace Explanation Layer")
    }

    pub(crate) fn attempt_hide_uncertainty() -> Result<()> {
        validation_error("Assistant explanation cannot hide uncertainty")
    }

    pub(crate) fn package_explanation_with_forced_rollback(
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
            let repo = WorkspaceAssistantExplanationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced assistant explanation transaction rollback".into(),
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
    ) -> Result<WorkspaceAssistantExplanationSnapshot> {
        // Programme III Explanation Layer — REQUIRED composition via load_snapshot only.
        let explanation = if scope.include_explanation {
            Some(WorkspaceExplanationService::load_snapshot(db, workspace_id)?)
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
        let evidence_navigation = if scope.include_evidence_navigation {
            Some(WorkspaceEvidenceNavigationService::load_snapshot(
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

        // Batch 13 retrieval via load_snapshot only — input packages.
        let retrieval = WorkspaceAssistantRetrievalService::load_snapshot(db, workspace_id)?;
        // Batch 12 context via load_snapshot only — scope/continuity input.
        let context = WorkspaceAssistantContextService::load_snapshot(db, workspace_id)?;
        // Batch 11 surface via load_snapshot only — presentation lineage refs.
        let surface = WorkspaceAssistantSurfaceService::load_snapshot(db, workspace_id)?;

        let snap = WorkspaceAssistantExplanationSnapshot::package(
            workspace_id,
            now,
            human_ask,
            scope,
            Some(&retrieval),
            Some(&context),
            Some(&surface),
            explanation.as_ref(),
            evidence_trace.as_ref(),
            evidence_navigation.as_ref(),
            evidence_consistency.as_ref(),
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
    Err(KernelError::AssistantExplanationValidation {
        message: message.into(),
    })
}
