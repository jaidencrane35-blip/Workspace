//! Workspace Assistant Interaction Intelligence - Programme IV Batch 15.
//!
//! Interaction flow packaging over durable snapshots only; never foreign generate;
//! never act for the user; never create memory, Intent, or agency.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceAssistantInteractionRepository};
use workspace_domain::{
    ActorContext, AssistantInteractionStatus, AssistantSurfaceScope,
    WorkspaceAssistantInteractionExplanation, WorkspaceAssistantInteractionProjection,
    WorkspaceAssistantInteractionSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceAssistantContextService, WorkspaceAssistantExplanationService,
    WorkspaceAssistantRetrievalService, WorkspaceAssistantSurfaceService,
};

pub(crate) struct WorkspaceAssistantInteractionService;

impl WorkspaceAssistantInteractionService {
    pub(crate) fn package_interaction(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantInteractionProjection> {
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
                    let repo = WorkspaceAssistantInteractionRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.assistant.interaction.packaged",
            &snap.interaction_id,
            json!({
                "workspace_id": workspace_id,
                "routed_packages": snap.route.routed_packages,
                "step_count": snap.steps.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceAssistantInteractionProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceAssistantInteractionRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == AssistantInteractionStatus::Current);
        Ok(WorkspaceAssistantInteractionProjection::assemble(
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
    ) -> Result<WorkspaceAssistantInteractionExplanation> {
        let projection = Self::load_snapshot(db, workspace_id)?;
        match projection.current {
            Some(snap) => Ok(WorkspaceAssistantInteractionExplanation::from_snapshot(&snap)),
            None => Ok(WorkspaceAssistantInteractionExplanation {
                explanation_id: "assistant_interaction_meta:missing".into(),
                workspace_id: projection.workspace_id,
                narrative_summary: None,
                step_summaries: vec![],
                route_summaries: vec![],
                gap_summaries: vec![
                    "No assistant interaction package is available for this workspace".into(),
                ],
                uncertainty: vec!["Missing interaction package remains missing".into()],
                narrative: "No assistant interaction artefact is available for this workspace.".into(),
                limitations: vec![
                    "Assistant interaction coordinates conversation flow packaging only".into(),
                    "It never acts for the user, invents memory, decides outcomes, or creates commitments".into(),
                    "Flow packaging never advises actions or grants permission".into(),
                ],
                authority_effect: "none".into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_act_for_user() -> Result<()> {
        validation_error("Assistant interaction cannot act for the user")
    }

    pub(crate) fn attempt_infer_intent() -> Result<()> {
        validation_error("Assistant interaction cannot infer user Intent")
    }

    pub(crate) fn attempt_create_hidden_memory() -> Result<()> {
        validation_error("Assistant interaction cannot create hidden memory")
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        validation_error("Assistant interaction cannot execute actions")
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        validation_error("Assistant interaction cannot approve")
    }

    pub(crate) fn attempt_decide() -> Result<()> {
        validation_error("Assistant interaction cannot decide outcomes")
    }

    pub(crate) fn attempt_bypass_permission_gateway() -> Result<()> {
        validation_error("Assistant interaction cannot bypass permission governance")
    }

    pub(crate) fn attempt_autonomous_loop() -> Result<()> {
        validation_error("Assistant interaction cannot run self-directed agent loops")
    }

    pub(crate) fn attempt_create_commitment() -> Result<()> {
        validation_error("Assistant interaction cannot create commitments")
    }

    pub(crate) fn package_interaction_with_forced_rollback(
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
            let repo = WorkspaceAssistantInteractionRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced assistant interaction transaction rollback".into(),
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
    ) -> Result<WorkspaceAssistantInteractionSnapshot> {
        // Required composition via load_snapshot only — Batches 11–14.
        // Do NOT re-load all Programme IV evidence engines; prefer composing
        // the four assistant packages. Missing packages become gaps.
        let surface = WorkspaceAssistantSurfaceService::load_snapshot(db, workspace_id)?;
        let context = WorkspaceAssistantContextService::load_snapshot(db, workspace_id)?;
        let retrieval = WorkspaceAssistantRetrievalService::load_snapshot(db, workspace_id)?;
        let explanation = WorkspaceAssistantExplanationService::load_snapshot(db, workspace_id)?;

        let snap = WorkspaceAssistantInteractionSnapshot::package(
            workspace_id,
            now,
            human_ask,
            scope,
            Some(&surface),
            Some(&context),
            Some(&retrieval),
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

fn validation_error<T>(message: impl Into<String>) -> Result<T> {
    Err(KernelError::AssistantInteractionValidation {
        message: message.into(),
    })
}
