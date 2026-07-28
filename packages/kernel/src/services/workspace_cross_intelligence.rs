//! Workspace Cross-Workspace Intelligence — Programme III Batch 10.
//!
//! Aggregate understanding. Never centralise authority.
//! Reads durable snapshots only — never foreign generate.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspaceCrossIntelligenceRepository};
use workspace_domain::{
    ActorContext, CROSS_WORKSPACE_SCOPE, CrossWorkspaceEvidenceRef,
    CrossWorkspaceIntelligenceProjection, CrossWorkspaceIntelligenceSnapshot,
    CrossWorkspaceIntelligenceStatus, CrossWorkspacePatternExplanation, WorkspaceContribution,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, PolicyGovernanceService, WorkspaceContextualUnderstandingService,
    WorkspaceExplanationService, WorkspaceHistoricalReconstructionService,
    WorkspaceInsightCoordinationService, WorkspaceKnowledgeIntegrationService,
    WorkspaceKnowledgeSynthesisService, WorkspaceService, WorkspaceStateCompositionService,
    WorkspaceTemporalIntelligenceService,
};

pub(crate) struct WorkspaceCrossIntelligenceService;

impl WorkspaceCrossIntelligenceService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
    ) -> Result<CrossWorkspaceIntelligenceProjection> {
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = WorkspaceCrossIntelligenceRepository::new(database);
                    repo.supersede_current(CROSS_WORKSPACE_SCOPE, &now)?;
                    repo.persist_view(&snap)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.cross_intelligence.generated",
            &snap.intelligence_id,
            json!({
                "scope_id": CROSS_WORKSPACE_SCOPE,
                "completeness": snap.completeness.as_str(),
                "pattern_count": snap.patterns.len(),
                "theme_count": snap.themes.len(),
                "gap_count": snap.gaps.len(),
                "workspace_count": snap.participating_workspaces.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
    ) -> Result<CrossWorkspaceIntelligenceProjection> {
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspaceCrossIntelligenceRepository::new(&guard);
        let views = repo.list_views(CROSS_WORKSPACE_SCOPE)?;
        let history = repo.list_history(CROSS_WORKSPACE_SCOPE)?;
        let history_count = repo.history_count(CROSS_WORKSPACE_SCOPE)?;
        let current = views
            .into_iter()
            .find(|v| v.status == CrossWorkspaceIntelligenceStatus::Current);
        Ok(CrossWorkspaceIntelligenceProjection::assemble(
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn explain(
        db: &Arc<Mutex<Database>>,
    ) -> Result<CrossWorkspacePatternExplanation> {
        let snap = Self::load_snapshot(db)?;
        match snap.current {
            Some(artefact) => Ok(CrossWorkspacePatternExplanation::from_snapshot(&artefact)),
            None => Ok(CrossWorkspacePatternExplanation {
                explanation_id: "cross_workspace_pattern_explanation:missing".into(),
                scope_id: snap.scope_id,
                completeness: None,
                summary: None,
                pattern_summaries: vec![],
                theme_summaries: vec![],
                risk_summaries: vec![],
                gaps: vec![
                    "No cross-workspace intelligence present — Missing source = Missing".into(),
                ],
                evidence_refs: vec![],
                uncertainty: vec![
                    "Missing cross-workspace evidence — never invent global patterns".into(),
                ],
                narrative:
                    "No cross-workspace intelligence artefact is available for this scope.".into(),
                limitations: vec![
                    "Cross-Workspace Intelligence aggregates understanding only — it never centralises authority"
                        .into(),
                ],
                authority_effect: CrossWorkspacePatternExplanation::AUTHORITY_EFFECT_NONE.into(),
                actionable: false,
            }),
        }
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot execute or apply corrections".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot mutate lifecycle".into(),
        })
    }

    pub(crate) fn attempt_grant_permissions() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot grant permissions".into(),
        })
    }

    pub(crate) fn attempt_approve_policy() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot approve policies".into(),
        })
    }

    pub(crate) fn attempt_create_recommendation() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot create recommendations".into(),
        })
    }

    pub(crate) fn attempt_become_memory_or_cognitive_model() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot become Memory or Cognitive Model".into(),
        })
    }

    pub(crate) fn attempt_invent_global_patterns() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot invent global patterns".into(),
        })
    }

    pub(crate) fn attempt_infer_missing_workspaces() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot infer missing workspaces".into(),
        })
    }

    pub(crate) fn attempt_fabricate_statistics() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot fabricate statistics".into(),
        })
    }

    pub(crate) fn attempt_silent_refresh() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot silently refresh foreign sources".into(),
        })
    }

    pub(crate) fn attempt_emit_command() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot emit commands".into(),
        })
    }

    pub(crate) fn attempt_convert_frequency_to_action() -> Result<()> {
        Err(KernelError::CrossWorkspaceIntelligenceValidation {
            message: "Cross-workspace intelligence cannot convert frequency into action".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let snap = Self::compose_view(db, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = WorkspaceCrossIntelligenceRepository::new(database);
            repo.supersede_current(CROSS_WORKSPACE_SCOPE, &now)?;
            repo.persist_view(&snap)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced cross-workspace intelligence transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::CrossWorkspaceIntelligenceValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        now: &str,
    ) -> Result<CrossWorkspaceIntelligenceSnapshot> {
        // (a) Lock briefly to list workspaces only.
        let workspaces = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            WorkspaceService::list(&guard)?
        };

        let mut contributions = Vec::new();
        for ws in &workspaces {
            let workspace_id = ws.id.as_str().to_string();
            let workspace_name = ws.name.clone();
            contributions.push(Self::contribute_workspace(
                db,
                &workspace_id,
                &workspace_name,
            )?);
        }

        let snap = CrossWorkspaceIntelligenceSnapshot::compose(now, contributions)
            .map_err(KernelError::from)?;
        snap.validate().map_err(KernelError::from)?;
        Ok(snap)
    }

    fn contribute_workspace(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        workspace_name: &str,
    ) -> Result<WorkspaceContribution> {
        // (b) load_snapshot ONLY — never ::generate on upstreams.
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
        let integration =
            WorkspaceKnowledgeIntegrationService::load_snapshot(db, workspace_id)?;
        let insight =
            WorkspaceInsightCoordinationService::load_snapshot(db, workspace_id)?;

        let mut surfaces_present = Vec::new();
        let mut surfaces_missing = Vec::new();
        let mut theme_labels = Vec::new();
        let mut risk_labels = Vec::new();
        let mut constraint_labels = Vec::new();
        let mut pattern_labels = Vec::new();
        let mut evidence_refs = Vec::new();
        let mut gap_count = 0usize;
        let mut conflict_indicated = false;

        let mut mark = |name: &str, present: bool, external_ref: Option<String>| {
            if present {
                surfaces_present.push(name.to_string());
                pattern_labels.push(format!("surface:{name}"));
                if let Some(ext) = external_ref {
                    evidence_refs.push(CrossWorkspaceEvidenceRef::link(
                        name,
                        ext,
                        Some(workspace_id.to_string()),
                        None,
                    ));
                }
            } else {
                surfaces_missing.push(name.to_string());
            }
        };

        match state.current.as_ref() {
            Some(envelope) => {
                if !envelope.contradictions.is_empty()
                    || envelope.consistency.as_str() == "contradictory"
                {
                    conflict_indicated = true;
                }
                gap_count += envelope.unknowns.len();
                mark(
                    "state",
                    true,
                    Some(envelope.state_id.clone()),
                );
            }
            None => mark("state", false, None),
        }

        match policy.current.as_ref() {
            Some(view) => {
                constraint_labels.push("policy_evaluation_present".into());
                gap_count += view.unknowns.len();
                mark(
                    "policy",
                    true,
                    Some(view.meta.evaluation_set_id.clone()),
                );
            }
            None => mark("policy", false, None),
        }

        match reconstruction.current.as_ref() {
            Some(view) => {
                gap_count += view.gaps.len();
                mark(
                    "historical",
                    true,
                    Some(view.reconstruction_id.clone()),
                );
            }
            None => mark("historical", false, None),
        }

        match temporal.current.as_ref() {
            Some(view) => {
                if !view.conflict_explanations.is_empty() {
                    conflict_indicated = true;
                }
                gap_count += view.gaps.len();
                mark("temporal", true, Some(view.analysis_id.clone()));
            }
            None => mark("temporal", false, None),
        }

        match explanation.current.as_ref() {
            Some(package) => {
                if !package.conflicts.is_empty() {
                    conflict_indicated = true;
                }
                gap_count += package.gaps.len();
                mark(
                    "explanation",
                    true,
                    Some(package.explanation_id.clone()),
                );
            }
            None => mark("explanation", false, None),
        }

        match contextual.current.as_ref() {
            Some(snap) => {
                theme_labels.extend(snap.themes.iter().map(|t| t.title.clone()));
                if snap.confidence.conflict_count > 0 {
                    conflict_indicated = true;
                }
                gap_count += snap.gaps.len();
                mark(
                    "contextual",
                    true,
                    Some(snap.understanding_id.clone()),
                );
            }
            None => mark("contextual", false, None),
        }

        match knowledge.current.as_ref() {
            Some(synth) => {
                if theme_labels.is_empty() {
                    theme_labels.extend(synth.concepts.iter().map(|c| c.label.clone()));
                }
                if synth.confidence.conflict_count > 0 {
                    conflict_indicated = true;
                }
                gap_count += synth.gaps.len();
                mark(
                    "knowledge_synthesis",
                    true,
                    Some(synth.synthesis_id.clone()),
                );
            }
            None => mark("knowledge_synthesis", false, None),
        }

        match integration.current.as_ref() {
            Some(integ) => {
                gap_count += integ.gaps.len();
                mark(
                    "knowledge_integration",
                    true,
                    Some(integ.integration_id.clone()),
                );
            }
            None => mark("knowledge_integration", false, None),
        }

        match insight.current.as_ref() {
            Some(coord) => {
                if theme_labels.is_empty() {
                    theme_labels.extend(coord.clusters.iter().map(|c| c.title.clone()));
                }
                let cluster_titles: Vec<String> =
                    coord.clusters.iter().map(|c| c.title.clone()).collect();
                if coord.assessment.contradictions_detected > 0 {
                    conflict_indicated = true;
                }
                gap_count += coord.gaps.len();
                mark(
                    "insight_coordination",
                    true,
                    Some(coord.coordination_id.clone()),
                );
                pattern_labels.extend(cluster_titles);
            }
            None => mark("insight_coordination", false, None),
        }

        if gap_count > 0 || !surfaces_missing.is_empty() {
            risk_labels.push("unresolved_gap".into());
        }
        if conflict_indicated {
            risk_labels.push("conflict_indicator".into());
        }

        // Workspace participation itself is non-authoritative evidence lineage for gaps/risks.
        if evidence_refs.is_empty() {
            evidence_refs.push(CrossWorkspaceEvidenceRef::link(
                "workspace",
                format!("workspace:{workspace_id}"),
                Some(workspace_id.to_string()),
                None,
            ));
        }

        Ok(WorkspaceContribution {
            workspace_id: workspace_id.to_string(),
            workspace_name: workspace_name.to_string(),
            available: true,
            surfaces_present,
            surfaces_missing,
            theme_labels,
            risk_labels,
            constraint_labels,
            pattern_labels,
            gap_count,
            conflict_indicated,
            evidence_refs,
        })
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
