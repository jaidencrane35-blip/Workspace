//! Workspace Explanation Layer — Programme III Batch 5.
//!
//! Explain evidence. Do not become the authority that changes reality.
//! Consumer of state / policy / reconstruction / temporal surfaces — never a controller.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceExplanationRepository};
use workspace_domain::{
    ActorContext, ExplanationPackage, ExplanationScope, ExplanationStatus,
    WorkspaceExplanationSnapshot, WorkspaceSituationExplanation,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceHistoricalReconstructionService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceExplanationService;

impl WorkspaceExplanationService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        scope: ExplanationScope,
    ) -> Result<WorkspaceExplanationSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let package = Self::compose_view(db, &workspace_id, &now, scope)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceExplanationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&package)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.explanation.package_generated",
            &package.explanation_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": package.completeness.as_str(),
                "section_count": package.sections.len(),
                "gap_count": package.gaps.len(),
                "conflict_count": package.conflicts.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceExplanationSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceExplanationRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == ExplanationStatus::Current);
        Ok(WorkspaceExplanationSnapshot::assemble(
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
    ) -> Result<WorkspaceSituationExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(package) => Ok(WorkspaceSituationExplanation::from_package(&package)),
            None => Ok(WorkspaceSituationExplanation {
                explanation_id: "situation_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                section_summaries: vec![],
                gaps: vec!["No explanation package present — ExplanationIncomplete".into()],
                conflicts: vec![],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing explanation evidence — never invent sections or resolve conflicts"
                        .into(),
                ],
                narrative: "No explanation artefact is available for this workspace.".into(),
                limitations: vec![
                    "Explanation Layer synthesises evidence — it does not change reality".into(),
                ],
                authority_effect: WorkspaceSituationExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot approve or authorize actions".into(),
        })
    }

    pub(crate) fn attempt_mutate_policy() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot mutate policy".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot mutate lifecycle state".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_resolve_conflict() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot resolve conflicts — only explain them".into(),
        })
    }

    pub(crate) fn attempt_fabricate_evidence() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot fabricate missing evidence".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::WorkspaceExplanationValidation {
            message: "Workspace explanation cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        scope: ExplanationScope,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let package = Self::compose_view(db, &workspace_id, &now, scope)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceExplanationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&package)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced workspace explanation transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceExplanationValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        scope: ExplanationScope,
    ) -> Result<ExplanationPackage> {
        // Evidence: load_snapshot only — never generate.
        let state = WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?;
        let policy = PolicyGovernanceService::load_snapshot(db, workspace_id)?;
        let reconstruction =
            WorkspaceHistoricalReconstructionService::load_snapshot(db, workspace_id)?;
        let temporal = WorkspaceTemporalIntelligenceService::load_snapshot(db, workspace_id)?;
        let package = ExplanationPackage::compose(
            workspace_id,
            now,
            scope,
            Some(&state),
            Some(&policy),
            Some(&reconstruction),
            Some(&temporal),
        )
        .map_err(KernelError::from)?;
        package.validate().map_err(KernelError::from)?;
        Ok(package)
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
