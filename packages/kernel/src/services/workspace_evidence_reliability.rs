//! Workspace Evidence Reliability Engine — Programme IV Batch 9.
//!
//! Observe reliability. Never establish truth.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceEvidenceReliabilityRepository};
use workspace_domain::{
    ActorContext, EvidenceReliabilityStatus, ReliabilityScope,
    WorkspaceEvidenceReliabilityExplanation, WorkspaceEvidenceReliabilityProjection,
    WorkspaceEvidenceReliabilitySnapshot,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceContextualUnderstandingService, WorkspaceEvidenceCompletenessService,
    WorkspaceEvidenceConsistencyService, WorkspaceEvidenceCoverageService,
    WorkspaceEvidenceDependencyService, WorkspaceEvidenceFreshnessService,
    WorkspaceEvidenceNavigationService, WorkspaceEvidenceTraceService, WorkspaceExplanationService,
    WorkspaceHistoricalReconstructionService, WorkspaceIntelligenceHubService,
    WorkspaceKnowledgeIntegrationService, WorkspaceSemanticQueryService,
    WorkspaceStateCompositionService, WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceEvidenceReliabilityService;

impl WorkspaceEvidenceReliabilityService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        scope: ReliabilityScope,
    ) -> Result<WorkspaceEvidenceReliabilityProjection> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, scope)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceEvidenceReliabilityRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.evidence_reliability.generated",
            &snap.reliability_id,
            json!({
                "workspace_id": workspace_id,
                "reliability": snap.reliability.as_str(),
                "observation_count": snap.observations.len(),
                "gap_count": snap.gaps.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceEvidenceReliabilityProjection> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceEvidenceReliabilityRepository::new(&guard);
        let views = repo.list_views(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = views
            .into_iter()
            .find(|v| v.status == EvidenceReliabilityStatus::Current);
        Ok(WorkspaceEvidenceReliabilityProjection::assemble(
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
    ) -> Result<WorkspaceEvidenceReliabilityExplanation> {
        let snap = Self::load_snapshot(db, workspace_id)?;
        match snap.current {
            Some(artefact) => {
                Ok(WorkspaceEvidenceReliabilityExplanation::from_snapshot(&artefact))
            }
            None => Ok(WorkspaceEvidenceReliabilityExplanation {
                explanation_id: "evidence_reliability_explanation:missing".into(),
                workspace_id: snap.workspace_id,
                reliability: None,
                narrative_summary: None,
                observation_summaries: vec![],
                gap_summaries: vec![
                    "No evidence reliability present — Missing source = Missing".into(),
                ],
                lineage_summaries: vec![],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing evidence reliability — never invent or estimate reliability".into(),
                ],
                narrative: "No evidence reliability artefact is available for this workspace."
                    .into(),
                limitations: vec![
                    "Workspace Evidence Reliability Engine observes recorded evidence reliability only"
                        .into(),
                    "Unknown remains unknown".into(),
                    "Partial remains partial — never a completion directive".into(),
                ],
                authority_effect: WorkspaceEvidenceReliabilityExplanation::AUTHORITY_EFFECT_NONE
                    .into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot execute or apply actions".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot create advisory outputs".into(),
        })
    }

    pub(crate) fn attempt_determine_truth() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot establish truth".into(),
        })
    }

    pub(crate) fn attempt_resolve_conflict() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot settle conflicts".into(),
        })
    }

    pub(crate) fn attempt_repair_evidence() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot repair evidence".into(),
        })
    }

    pub(crate) fn attempt_fabricate_reliability() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot fabricate reliability".into(),
        })
    }

    pub(crate) fn attempt_rank_by_opinion() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot rank by opinion".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::EvidenceReliabilityValidation {
            message: "Evidence reliability cannot emit commands".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
        scope: ReliabilityScope,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &workspace_id, &now, scope)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceEvidenceReliabilityRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced evidence reliability transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::EvidenceReliabilityValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
        scope: ReliabilityScope,
    ) -> Result<WorkspaceEvidenceReliabilitySnapshot> {
        let evidence_completeness = if scope.include_evidence_completeness {
            Some(WorkspaceEvidenceCompletenessService::load_snapshot(
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
        let evidence_dependency = if scope.include_evidence_dependency {
            Some(WorkspaceEvidenceDependencyService::load_snapshot(
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
        let evidence_coverage = if scope.include_evidence_coverage {
            Some(WorkspaceEvidenceCoverageService::load_snapshot(
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
        let evidence_navigation = if scope.include_evidence_navigation {
            Some(WorkspaceEvidenceNavigationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let semantic_query = if scope.include_semantic_query {
            Some(WorkspaceSemanticQueryService::load_snapshot(
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
        let knowledge_integration = if scope.include_knowledge_integration {
            Some(WorkspaceKnowledgeIntegrationService::load_snapshot(
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
        let explanation = if scope.include_explanation {
            Some(WorkspaceExplanationService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let temporal = if scope.include_temporal {
            Some(WorkspaceTemporalIntelligenceService::load_snapshot(
                db,
                workspace_id,
            )?)
        } else {
            None
        };
        let reconstruction = if scope.include_reconstruction {
            Some(WorkspaceHistoricalReconstructionService::load_snapshot(
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

        let snap = WorkspaceEvidenceReliabilitySnapshot::compose(
            workspace_id,
            now,
            scope,
            evidence_completeness.as_ref(),
            evidence_freshness.as_ref(),
            evidence_dependency.as_ref(),
            evidence_consistency.as_ref(),
            evidence_coverage.as_ref(),
            evidence_trace.as_ref(),
            evidence_navigation.as_ref(),
            semantic_query.as_ref(),
            intelligence_hub.as_ref(),
            knowledge_integration.as_ref(),
            contextual.as_ref(),
            explanation.as_ref(),
            temporal.as_ref(),
            reconstruction.as_ref(),
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
