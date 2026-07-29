//! Workspace Evidence Trace Engine — Programme IV Batch 3.
//!
//! Trace provenance. Never infer provenance.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceEvidenceTraceRepository};
use workspace_domain::{
    ActorContext, EvidenceTraceRequest, EvidenceTraceScope, EvidenceTraceStatus,
    WorkspaceEvidenceTraceExplanation, WorkspaceEvidenceTraceProjection,
    WorkspaceEvidenceTraceSnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceContextualUnderstandingService, WorkspaceEvidenceNavigationService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceIntelligenceHubService, WorkspaceKnowledgeIntegrationService,
    WorkspaceKnowledgeSynthesisService, WorkspaceSemanticQueryService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceEvidenceTraceService;

impl WorkspaceEvidenceTraceService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        request: EvidenceTraceRequest,
    ) -> Result<WorkspaceEvidenceTraceProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, request)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceEvidenceTraceRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.evidence_trace.generated",
            &snap.trace_id,
            json!({
                "workspace_id": workspace_id,
                "completeness": snap.completeness.as_str(),
                "segment_count": snap.chain.as_ref().map(|c| c.segments.len()).unwrap_or(0),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceEvidenceTraceProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceEvidenceTraceRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == EvidenceTraceStatus::Current);
        Ok(WorkspaceEvidenceTraceProjection::assemble(
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
    ) -> Result<WorkspaceEvidenceTraceExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => Ok(WorkspaceEvidenceTraceExplanation::from_snapshot(&artefact)),
            None => Ok(WorkspaceEvidenceTraceExplanation {
                explanation_id: "evidence_trace_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                completeness: None,
                narrative_summary: None,
                segment_summaries: vec![],
                gap_summaries: vec![
                    "No evidence trace present — Missing source = Missing".into(),
                ],
                lineage_summaries: vec![],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing evidence trace — never invent lineage or bridge history".into(),
                ],
                narrative: "No evidence trace artefact is available for this workspace.".into(),
                limitations: vec![
                    "Workspace Evidence Trace Engine reconstructs recorded provenance only"
                        .into(),
                    "Missing provenance remains missing".into(),
                ],
                authority_effect: WorkspaceEvidenceTraceExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot execute or apply actions".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_infer_provenance() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot infer provenance".into(),
        })
    }

    pub(crate) fn attempt_reason() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot perform reasoning".into(),
        })
    }

    pub(crate) fn attempt_invent_hops() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot invent hops — recorded provenance only".into(),
        })
    }

    pub(crate) fn attempt_bridge_lineage() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot bridge missing lineage".into(),
        })
    }

    pub(crate) fn attempt_repair_history() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot repair truncated or missing history".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::EvidenceTraceValidation {
            message: "Evidence trace cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        request: EvidenceTraceRequest,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, request)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceEvidenceTraceRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced evidence trace transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::EvidenceTraceValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        request: EvidenceTraceRequest,
    ) -> Result<WorkspaceEvidenceTraceSnapshot> {
        let evidence_navigation = if request.trace_scope.include_evidence_navigation {
            Some(WorkspaceEvidenceNavigationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let semantic_query = if request.trace_scope.include_semantic_query {
            Some(WorkspaceSemanticQueryService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let intelligence_hub = if request.trace_scope.include_intelligence_hub {
            Some(WorkspaceIntelligenceHubService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let knowledge_integration = if request.trace_scope.include_knowledge_integration {
            Some(WorkspaceKnowledgeIntegrationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let knowledge_synthesis = if request.trace_scope.include_knowledge_synthesis {
            Some(WorkspaceKnowledgeSynthesisService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let contextual = if request.trace_scope.include_contextual {
            Some(WorkspaceContextualUnderstandingService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let explanation = if request.trace_scope.include_explanation {
            Some(WorkspaceExplanationService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };
        let temporal = if request.trace_scope.include_temporal {
            Some(WorkspaceTemporalIntelligenceService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let reconstruction = if request.trace_scope.include_reconstruction {
            Some(WorkspaceHistoricalReconstructionService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let state = if request.trace_scope.include_state {
            Some(WorkspaceStateCompositionService::load_snapshot(db, workspace_id)?)
        } else {
            None
        };

        let snap = WorkspaceEvidenceTraceSnapshot::compose(
            workspace_id,
            now,
            request,
            evidence_navigation.as_ref(),
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
        let _ = EvidenceTraceScope::all_surfaces();
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
