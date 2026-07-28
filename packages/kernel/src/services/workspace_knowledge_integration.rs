//! Workspace Knowledge Integration — Programme III Batch 8.
//!
//! Integrate evidence. Do not become the authority behind the evidence.
//! Retrieval relevance ≠ factual authority. Confidence ≠ permission.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceKnowledgeIntegrationRepository};
use workspace_domain::{
    ActorContext, KnowledgeIntegrationExplanation, KnowledgeIntegrationProjection,
    KnowledgeIntegrationResult, KnowledgeIntegrationStatus, KnowledgeRetrievalFrame,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceKnowledgeSynthesisService, WorkspaceStateCompositionService,
    WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceKnowledgeIntegrationService;

impl WorkspaceKnowledgeIntegrationService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: KnowledgeRetrievalFrame,
    ) -> Result<KnowledgeIntegrationProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceKnowledgeIntegrationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.knowledge_integration.generated",
            &snap.integration_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "link_count": snap.links.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<KnowledgeIntegrationProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceKnowledgeIntegrationRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == KnowledgeIntegrationStatus::Current);
        Ok(KnowledgeIntegrationProjection::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    /// Frame-scoped retrieval without requiring a new persist — load last artefact
    /// or compose ephemerally via load_snapshot only (never invents evidence).
    pub(crate) fn retrieve(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
        frame: KnowledgeRetrievalFrame,
    ) -> Result<KnowledgeIntegrationResult> {
        let workspace_id = workspace_id.into();
        let existing = Self::load_snapshot(db, &workspace_id)?;
        if let Some(current) = existing.current {
            if frames_compatible(&current.frame, &frame) {
                return Ok(current);
            }
        }
        let now = Utc::now().to_rfc3339();
        Self::compose_view(db, &workspace_id, &now, frame)
    }

    pub(crate) fn explain(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<KnowledgeIntegrationExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(KnowledgeIntegrationExplanation::from_result(&artefact)),
            None => Ok(KnowledgeIntegrationExplanation {
                explanation_id: "knowledge_integration_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                summary: None,
                link_summaries: vec![],
                gaps: vec![
                    "No knowledge integration present — Missing source = Missing".into(),
                ],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing integration evidence — never invent hits or relationships".into(),
                ],
                narrative:
                    "No knowledge integration artefact is available for this workspace.".into(),
                limitations: vec![
                    "Knowledge integration retrieves evidence — it does not become the authority"
                        .into(),
                ],
                authority_effect: KnowledgeIntegrationExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_approve() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot approve policies".into(),
        })
    }

    pub(crate) fn attempt_mutate_intent() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot mutate Intent".into(),
        })
    }

    pub(crate) fn attempt_modify_cognitive_model() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot modify Cognitive Model".into(),
        })
    }

    pub(crate) fn attempt_alter_memory() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot alter Memory".into(),
        })
    }

    pub(crate) fn attempt_mutate_knowledge_synthesis() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot mutate Knowledge Synthesis".into(),
        })
    }

    pub(crate) fn attempt_repair_contradictions() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot repair contradictions".into(),
        })
    }

    pub(crate) fn attempt_convert_confidence_to_authority() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot convert confidence into authority".into(),
        })
    }

    pub(crate) fn attempt_fabricate_hits_without_evidence() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot fabricate hits without evidence".into(),
        })
    }

    pub(crate) fn attempt_treat_correlation_as_causation() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot treat correlation as causation".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::KnowledgeIntegrationValidation {
            message: "Knowledge integration cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        frame: KnowledgeRetrievalFrame,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, frame)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceKnowledgeIntegrationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced knowledge integration transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::KnowledgeIntegrationValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        frame: KnowledgeRetrievalFrame,
    ) -> Result<KnowledgeIntegrationResult> {
        // Evidence: load_snapshot only — never generate.
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
        let snap = KnowledgeIntegrationResult::compose(
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

fn frames_compatible(a: &KnowledgeRetrievalFrame, b: &KnowledgeRetrievalFrame) -> bool {
    a.include_state == b.include_state
        && a.include_policy == b.include_policy
        && a.include_reconstruction == b.include_reconstruction
        && a.include_temporal == b.include_temporal
        && a.include_explanation == b.include_explanation
        && a.include_contextual == b.include_contextual
        && a.include_knowledge_synthesis == b.include_knowledge_synthesis
        && a.focus == b.focus
}
