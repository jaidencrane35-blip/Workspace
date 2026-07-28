//! Workspace Knowledge Synthesis — Programme III Batch 7.
//!
//! Synthesize understanding from evidence. Do not create reality.
//! Derived knowledge ≠ truth. Relationships ≠ causation. Confidence ≠ authority.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceKnowledgeSynthesisRepository};
use workspace_domain::{
    ActorContext, KnowledgeSynthesisExplanation, KnowledgeSynthesisFrame,
    KnowledgeSynthesisProjection, KnowledgeSynthesisStatus, WorkspaceKnowledgeSynthesis,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceKnowledgeSynthesisService;

impl WorkspaceKnowledgeSynthesisService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: KnowledgeSynthesisFrame,
    ) -> Result<KnowledgeSynthesisProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceKnowledgeSynthesisRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.knowledge_synthesis.generated",
            &snap.synthesis_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "concept_count": snap.concepts.len(),
                "cluster_count": snap.clusters.len(),
                "relationship_count": snap.relationships.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<KnowledgeSynthesisProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceKnowledgeSynthesisRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == KnowledgeSynthesisStatus::Current);
        Ok(KnowledgeSynthesisProjection::assemble(
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
    ) -> Result<KnowledgeSynthesisExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(KnowledgeSynthesisExplanation::from_synthesis(&artefact)),
            None => Ok(KnowledgeSynthesisExplanation {
                explanation_id: "knowledge_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                summary: None,
                concept_summaries: vec![],
                cluster_summaries: vec![],
                relationship_summaries: vec![],
                gaps: vec![
                    "No knowledge synthesis present — Missing evidence = Gap".into(),
                ],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing synthesis evidence — never invent concepts or relationships".into(),
                ],
                narrative: "No knowledge synthesis artefact is available for this workspace."
                    .into(),
                limitations: vec![
                    "Knowledge synthesis organises derived concepts — it does not create reality"
                        .into(),
                ],
                authority_effect: KnowledgeSynthesisExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot approve policies or authorize actions".into(),
        })
    }

    pub(crate) fn attempt_mutate_intent() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot mutate Intent".into(),
        })
    }

    pub(crate) fn attempt_modify_cognitive_model() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot modify Cognitive Model".into(),
        })
    }

    pub(crate) fn attempt_alter_memory() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot alter Memory".into(),
        })
    }

    pub(crate) fn attempt_repair_contradictions() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot repair contradictions — gaps/conflicts preserved"
                .into(),
        })
    }

    pub(crate) fn attempt_convert_confidence_to_authority() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot convert confidence into authority".into(),
        })
    }

    pub(crate) fn attempt_fabricate_concepts_without_evidence() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot fabricate concepts without evidence".into(),
        })
    }

    pub(crate) fn attempt_treat_correlation_as_causation() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot treat correlation as causation".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::KnowledgeSynthesisValidation {
            message: "Knowledge synthesis cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: KnowledgeSynthesisFrame,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceKnowledgeSynthesisRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced knowledge synthesis transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::KnowledgeSynthesisValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        frame: KnowledgeSynthesisFrame,
    ) -> Result<WorkspaceKnowledgeSynthesis> {
        // Evidence: load_snapshot only — never generate.
        let state = WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?;
        let policy = PolicyGovernanceService::load_snapshot(db, workspace_id)?;
        let reconstruction =
            WorkspaceHistoricalReconstructionService::load_snapshot(db, workspace_id)?;
        let temporal = WorkspaceTemporalIntelligenceService::load_snapshot(db, workspace_id)?;
        let explanation = WorkspaceExplanationService::load_snapshot(db, workspace_id)?;
        let contextual =
            WorkspaceContextualUnderstandingService::load_snapshot(db, workspace_id)?;
        let snap = WorkspaceKnowledgeSynthesis::compose(
            workspace_id,
            now,
            frame,
            Some(&state),
            Some(&policy),
            Some(&reconstruction),
            Some(&temporal),
            Some(&explanation),
            Some(&contextual),
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
