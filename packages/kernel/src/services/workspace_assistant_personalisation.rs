//! Workspace Assistant Personalisation Boundary — Programme IV Batch 16.
//!
//! Preference presentation packaging over durable snapshots only; never foreign
//! generate; never invent who the user is; never write preference SoT;
//! never create hidden profiles or autonomously adapt.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceAssistantPersonalisationRepository};
use workspace_domain::{
    ActorContext, AssistantPersonalisationStatus, AssistantSurfaceScope,
    ExplicitPreferenceSummary, WorkspaceAssistantPersonalisationExplanation,
    WorkspaceAssistantPersonalisationProjection, WorkspaceAssistantPersonalisationSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AiPersonalizationService, AuditService, WorkspaceAssistantContextService,
    WorkspaceAssistantExplanationService, WorkspaceAssistantInteractionService,
    WorkspaceAssistantRetrievalService, WorkspaceAssistantSurfaceService,
};

pub(crate) struct WorkspaceAssistantPersonalisationService;

impl WorkspaceAssistantPersonalisationService {
    pub(crate) fn package_personalisation(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantPersonalisationProjection> {
        let workspace_id = workspace_id.into();
        let human_ask = human_ask.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::package_view(db, actor, &workspace_id, &now, &human_ask, scope)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceAssistantPersonalisationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.assistant.personalisation.packaged",
            &snap.personalisation_id,
            json!({
                "workspace_id": workspace_id,
                "applied_preference_refs": snap.adaptation.applied_preference_refs,
                "personalization_enabled": snap.adaptation.personalization_enabled,
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
    ) -> Result<WorkspaceAssistantPersonalisationProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceAssistantPersonalisationRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == AssistantPersonalisationStatus::Current);
        Ok(WorkspaceAssistantPersonalisationProjection::assemble(
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
    ) -> Result<WorkspaceAssistantPersonalisationExplanation> {
        let projection = Self::load_snapshot(db, workspace_id)?;
        match projection.current {
            Some(snap) => Ok(WorkspaceAssistantPersonalisationExplanation::from_snapshot(
                &snap,
            )),
            None => Ok(WorkspaceAssistantPersonalisationExplanation {
                explanation_id: "assistant_personalisation_meta:missing".into(),
                workspace_id: projection.workspace_id,
                narrative_summary: None,
                item_summaries: vec![],
                adaptation_summaries: vec![],
                gap_summaries: vec![
                    "No assistant personalisation package is available for this workspace".into(),
                ],
                uncertainty: vec!["Missing personalisation package remains missing".into()],
                narrative: "No assistant personalisation artefact is available for this workspace."
                    .into(),
                limitations: vec![
                    "Assistant personalisation packages explicit presentation preferences only"
                        .into(),
                    "It never invents who the user is, never creates hidden profiles, and never writes preferences"
                        .into(),
                    "Preference packaging never grants permission or decides outcomes".into(),
                ],
                authority_effect: "none".into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_infer_identity() -> Result<()> {
        validation_error("Assistant personalisation cannot infer identity")
    }

    pub(crate) fn attempt_infer_personality() -> Result<()> {
        validation_error("Assistant personalisation cannot infer personality")
    }

    pub(crate) fn attempt_infer_preference() -> Result<()> {
        validation_error("Assistant personalisation cannot infer preferences")
    }

    pub(crate) fn attempt_write_preference() -> Result<()> {
        validation_error("Assistant personalisation cannot write preference SoT")
    }

    pub(crate) fn attempt_create_hidden_profile() -> Result<()> {
        validation_error("Assistant personalisation cannot create hidden profiles")
    }

    pub(crate) fn attempt_autonomous_adapt() -> Result<()> {
        validation_error("Assistant personalisation cannot autonomously adapt")
    }

    pub(crate) fn attempt_bypass_permission_gateway() -> Result<()> {
        validation_error("Assistant personalisation cannot bypass permission governance")
    }

    pub(crate) fn attempt_influence_permission() -> Result<()> {
        validation_error("Assistant personalisation cannot influence permissions")
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        validation_error("Assistant personalisation cannot execute actions")
    }

    pub(crate) fn package_personalisation_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        human_ask: impl Into<String>,
        scope: AssistantSurfaceScope,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let human_ask = human_ask.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::package_view(db, actor, &workspace_id, &now, &human_ask, scope)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceAssistantPersonalisationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced assistant personalisation transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => validation_error("expected forced rollback"),
        }
    }

    fn package_view(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: &str,
        now: &str,
        human_ask: &str,
        scope: AssistantSurfaceScope,
    ) -> Result<WorkspaceAssistantPersonalisationSnapshot> {
        // READ ONLY preference access via AiPersonalizationService.
        // Never create/update/delete/set_enabled from this service.
        let personalization_enabled = AiPersonalizationService::is_enabled(db)?;
        let profile =
            AiPersonalizationService::get_profile(db, actor, Some(workspace_id), 200)?;
        let preferences = ExplicitPreferenceSummary::from_profile(&profile);

        // Required composition via load_snapshot only — Batches 11–15.
        // Do NOT call WorkspaceWorkingStyle generate paths or foreign ::generate.
        let surface = WorkspaceAssistantSurfaceService::load_snapshot(db, workspace_id)?;
        let context = WorkspaceAssistantContextService::load_snapshot(db, workspace_id)?;
        let retrieval = WorkspaceAssistantRetrievalService::load_snapshot(db, workspace_id)?;
        let explanation = WorkspaceAssistantExplanationService::load_snapshot(db, workspace_id)?;
        let interaction = WorkspaceAssistantInteractionService::load_snapshot(db, workspace_id)?;

        let snap = WorkspaceAssistantPersonalisationSnapshot::package(
            workspace_id,
            now,
            human_ask,
            scope,
            personalization_enabled,
            &preferences,
            Some(&surface),
            Some(&context),
            Some(&retrieval),
            Some(&explanation),
            Some(&interaction),
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
    Err(KernelError::AssistantPersonalisationValidation {
        message: message.into(),
    })
}
