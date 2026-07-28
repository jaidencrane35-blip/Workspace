//! Workspace Evidence Navigation Engine — Programme IV Batch 2.
//!
//! Navigate evidence. Never interpret evidence.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceEvidenceNavigationRepository};
use workspace_domain::{
    ActorContext, EvidenceNavigationSession, EvidenceNavigationStatus, TraversalScope,
    WorkspaceEvidenceNavigationExplanation, WorkspaceEvidenceNavigationProjection,
    WorkspaceEvidenceNavigationSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceContextualUnderstandingService, WorkspaceExplanationService,
    WorkspaceHistoricalReconstructionService, WorkspaceIntelligenceHubService,
    WorkspaceKnowledgeIntegrationService, WorkspaceKnowledgeSynthesisService,
    WorkspaceSemanticQueryService, WorkspaceStateCompositionService,
    WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceEvidenceNavigationService;

impl WorkspaceEvidenceNavigationService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        session: EvidenceNavigationSession,
    ) -> Result<WorkspaceEvidenceNavigationProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, session)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceEvidenceNavigationRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.evidence_navigation.generated",
            &snap.navigation_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "path_count": snap.paths.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceEvidenceNavigationProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceEvidenceNavigationRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == EvidenceNavigationStatus::Current);
        Ok(WorkspaceEvidenceNavigationProjection::assemble(
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
    ) -> Result<WorkspaceEvidenceNavigationExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => {
                Ok(WorkspaceEvidenceNavigationExplanation::from_snapshot(&artefact))
            }
            None => Ok(WorkspaceEvidenceNavigationExplanation {
                explanation_id: "evidence_navigation_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                narrative_summary: None,
                path_summaries: vec![],
                gap_summaries: vec![
                    "No evidence navigation present — Missing source = Missing".into(),
                ],
                lineage_summaries: vec![],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing evidence navigation — never invent paths or continuity".into(),
                ],
                narrative: "No evidence navigation artefact is available for this workspace."
                    .into(),
                limitations: vec![
                    "Workspace Evidence Navigation Engine exposes existing evidence relationships only"
                        .into(),
                    "Broken chains remain broken".into(),
                ],
                authority_effect: WorkspaceEvidenceNavigationExplanation::AUTHORITY_EFFECT_NONE
                    .into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot execute or apply actions".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_interpret() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot interpret evidence".into(),
        })
    }

    pub(crate) fn attempt_reason() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot perform reasoning".into(),
        })
    }

    pub(crate) fn attempt_invent_paths() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot invent paths — existing relationships only"
                .into(),
        })
    }

    pub(crate) fn attempt_bridge_lineage() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot bridge missing lineage".into(),
        })
    }

    pub(crate) fn attempt_fabricate_traversal() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot fabricate traversal continuity".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::EvidenceNavigationValidation {
            message: "Evidence navigation cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        session: EvidenceNavigationSession,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, session)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceEvidenceNavigationRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced evidence navigation transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::EvidenceNavigationValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        session: EvidenceNavigationSession,
    ) -> Result<WorkspaceEvidenceNavigationSnapshot> {
        let semantic_query = if session.traversal_scope.include_semantic_query {
            Some(WorkspaceSemanticQueryService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let intelligence_hub = if session.traversal_scope.include_intelligence_hub {
            Some(WorkspaceIntelligenceHubService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let knowledge_integration = if session.traversal_scope.include_knowledge_integration {
            Some(WorkspaceKnowledgeIntegrationService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let knowledge_synthesis = if session.traversal_scope.include_knowledge_synthesis {
            Some(WorkspaceKnowledgeSynthesisService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let contextual = if session.traversal_scope.include_contextual {
            Some(WorkspaceContextualUnderstandingService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let explanation = if session.traversal_scope.include_explanation {
            Some(WorkspaceExplanationService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let temporal = if session.traversal_scope.include_temporal {
            Some(WorkspaceTemporalIntelligenceService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let reconstruction = if session.traversal_scope.include_reconstruction {
            Some(WorkspaceHistoricalReconstructionService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let state = if session.traversal_scope.include_state {
            Some(WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };

        let snap = WorkspaceEvidenceNavigationSnapshot::compose(
            workspace_id,
            now,
            session,
            semantic_query.as_ref(),
            intelligence_hub.as_ref(),
            knowledge_integration.as_ref(),
            knowledge_synthesis.as_ref(),
            contextual.as_ref(),
            explanation.as_ref(),
            temporal.as_ref(),
            reconstruction.as_ref(),
            state.as_ref(),
        )
        .map_err(KernelError::from)?;
        snap.validate().map_err(KernelError::from)?;
        let _ = TraversalScope::all_surfaces();
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
