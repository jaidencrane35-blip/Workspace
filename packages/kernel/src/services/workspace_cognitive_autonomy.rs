//! Workspace Cognitive Autonomy — Programme II Batch 8 (final).
//!
//! Top observational suggestion layer. Evaluates cognitive stack evidence and
//! produces autonomy opportunities, proposals, and safety assessments.
//! Autonomy may suggest. Authority must still approve.
//! Never executes, self-approves, or mutates lifecycles.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{CognitiveAutonomyRepository, Database};
use workspace_domain::{
    ActorContext, AutonomyEvidenceLink, AutonomyOpportunity, AutonomyRecommendation,
    AutonomySafetyAssessment, AutonomyStatus, AutomationProposal, CognitiveAutonomyMeta,
    CognitiveAutonomySnapshot, CognitiveAutonomyView,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceCognitiveAgentCastService, WorkspaceCognitiveGraphService,
    WorkspaceCognitiveOrchestrationService, WorkspaceLearningAdaptationService,
    WorkspacePlanningService, WorkspaceReasoningMemoryService,
};

pub(crate) struct WorkspaceCognitiveAutonomyService;

impl WorkspaceCognitiveAutonomyService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveAutonomySnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = CognitiveAutonomyRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.cognitive_autonomy.snapshot_generated",
            &view.meta.autonomy_id,
            json!({
                "workspace_id": workspace_id,
                "opportunity_count": view.opportunities.len(),
                "proposal_count": view.proposals.len(),
                "recommendation_count": view.recommendations.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveAutonomySnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = CognitiveAutonomyRepository::new(&guard);
        let metas = repo.list_meta(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = metas
            .into_iter()
            .find(|m| m.status == AutonomyStatus::Current)
            .map(|m| repo.load_view(&m))
            .transpose()?;
        Ok(CognitiveAutonomySnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAutonomyValidation {
            message: "Cognitive Autonomy cannot execute commands".into(),
        })
    }

    pub(crate) fn attempt_self_approve() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAutonomyValidation {
            message: "Cognitive Autonomy cannot approve itself".into(),
        })
    }

    pub(crate) fn attempt_grant_permission() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAutonomyValidation {
            message: "Cognitive Autonomy cannot grant permissions".into(),
        })
    }

    pub(crate) fn attempt_create_hidden_task() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAutonomyValidation {
            message: "Cognitive Autonomy cannot create hidden tasks".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAutonomyValidation {
            message: "Cognitive Autonomy cannot mutate lifecycle state".into(),
        })
    }

    pub(crate) fn attempt_convert_confidence_to_authority() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAutonomyValidation {
            message: "Confidence must never become authority".into(),
        })
    }

    pub(crate) fn generate_with_forced_rollback(
        db: &Arc<Mutex<Database>>,
        _actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<()> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let err = guard.run_in_transaction(|database| {
            let repo = CognitiveAutonomyRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced cognitive autonomy transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceCognitiveAutonomyValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<CognitiveAutonomyView> {
        // Read-only composition — never call foreign generate paths.
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id)?;
        let reasoning = WorkspaceReasoningMemoryService::load_snapshot(db, workspace_id)?;
        let graph = WorkspaceCognitiveGraphService::load_snapshot(db, workspace_id)?;
        let orchestration =
            WorkspaceCognitiveOrchestrationService::load_snapshot(db, workspace_id)?;
        let learning = WorkspaceLearningAdaptationService::load_snapshot(db, workspace_id)?;
        let cast = WorkspaceCognitiveAgentCastService::load_snapshot(db, workspace_id)?;

        let mut evidence_links = Vec::new();
        let mut opportunities = Vec::new();
        let mut proposals = Vec::new();
        let mut recommendations = Vec::new();

        if let Some(p) = &planning.current {
            evidence_links.push(AutonomyEvidenceLink {
                external_ref: p.plan.id.clone(),
                kind: "planning".into(),
            });
        }
        if let Some(r) = &reasoning.current {
            evidence_links.push(AutonomyEvidenceLink {
                external_ref: r.id.clone(),
                kind: "reasoning".into(),
            });
        }
        if let Some(g) = &graph.current {
            evidence_links.push(AutonomyEvidenceLink {
                external_ref: g.meta.id.clone(),
                kind: "cognitive_graph".into(),
            });
        }
        if let Some(o) = &orchestration.current {
            evidence_links.push(AutonomyEvidenceLink {
                external_ref: o.meta.orchestration_id.clone(),
                kind: "orchestration".into(),
            });
            if o.stale_items.len() >= 2 {
                let refs: Vec<_> = o
                    .stale_items
                    .iter()
                    .flat_map(|s| s.evidence_refs.clone())
                    .take(10)
                    .collect();
                opportunities.push(
                    AutonomyOpportunity::suggest(
                        o.meta.orchestration_id.clone(),
                        format!(
                            "Orchestration reports {} stale item(s) — refresh coordination opportunity",
                            o.stale_items.len()
                        ),
                        "Reduce cognitive inconsistency via governed refresh",
                        "low",
                        72,
                        refs,
                    )
                    .map_err(KernelError::from)?,
                );
            }
        }
        if let Some(l) = &learning.current {
            evidence_links.push(AutonomyEvidenceLink {
                external_ref: l.meta.learning_id.clone(),
                kind: "learning".into(),
            });
            if l.patterns.len() >= 2 {
                let refs: Vec<_> = l
                    .patterns
                    .iter()
                    .flat_map(|p| p.evidence_refs.clone())
                    .take(14)
                    .collect();
                opportunities.push(
                    AutonomyOpportunity::suggest(
                        l.meta.learning_id.clone(),
                        format!(
                            "Repeated patterns detected ({} pattern(s)) — potential automation opportunity",
                            l.patterns.len()
                        ),
                        "Reduce repeated manual cognitive refresh work",
                        "low",
                        78,
                        refs,
                    )
                    .map_err(KernelError::from)?,
                );
            }
            for candidate in &l.adaptation_candidates {
                opportunities.push(
                    AutonomyOpportunity::suggest(
                        candidate.id.clone(),
                        format!("Learning adaptation candidate: {}", candidate.title),
                        candidate.suggestion.clone(),
                        "medium",
                        candidate.confidence,
                        candidate.supporting_evidence_refs.clone(),
                    )
                    .map_err(KernelError::from)?,
                );
            }
        }
        if let Some(c) = &cast.current {
            evidence_links.push(AutonomyEvidenceLink {
                external_ref: c.meta.cast_id.clone(),
                kind: "agent_cast".into(),
            });
            if c.critiques.iter().any(|cr| cr.risk_level == "high") {
                opportunities.push(
                    AutonomyOpportunity::suggest(
                        c.meta.cast_id.clone(),
                        "Agent cast reports high-risk critiques — review before any automation",
                        "Improve safety posture via human review",
                        "high",
                        65,
                        c.critiques
                            .iter()
                            .filter(|cr| cr.risk_level == "high")
                            .map(|cr| cr.critique_id.clone())
                            .collect(),
                    )
                    .map_err(KernelError::from)?,
                );
            }
        }

        // Missing upstream evidence stays unknown — do not invent opportunities from absence alone
        // beyond recording that evidence is incomplete in safety assessment.

        if !opportunities.is_empty() {
            let first = &opportunities[0];
            proposals.push(
                AutomationProposal::suggest(
                    format!("Suggested workflow for: {}", first.description),
                    vec![
                        "Review autonomy opportunity evidence".into(),
                        "Obtain explicit user/system approval".into(),
                        "Route approved Intent through CommandPipeline".into(),
                        "PermissionGateway authorises capable service".into(),
                    ],
                    evidence_links
                        .iter()
                        .map(|l| l.external_ref.clone())
                        .take(5)
                        .collect(),
                    vec!["work_context.write".into()],
                    first.confidence.saturating_sub(5),
                )
                .map_err(KernelError::from)?,
            );
            recommendations.push(
                AutonomyRecommendation::suggest(
                    "Consider approved automation only after Gateway-bound Intent confirmation",
                    opportunities
                        .iter()
                        .map(|o| o.opportunity_id.clone())
                        .collect(),
                    70,
                )
                .map_err(KernelError::from)?,
            );
        }

        let mut risks = vec![
            "Over-automation without human review".into(),
            "Treating confidence as permission".into(),
        ];
        if evidence_links.is_empty() {
            risks.push("Insufficient upstream evidence — opportunities unknown".into());
        }
        if opportunities.iter().any(|o| o.risk_level == "high") {
            risks.push("High-risk opportunity requires elevated approval scrutiny".into());
        }

        let safety = AutonomySafetyAssessment::assess(
            risks,
            vec![
                "Autonomy may suggest only".into(),
                "Authority must still approve".into(),
                "No executable command payloads in proposals".into(),
            ],
            vec![
                "Silent AI→execute path must never exist".into(),
                "Self-approval would bypass Gateway".into(),
                "Hidden task creation would bypass Intent".into(),
            ],
            vec![
                "CommandPipeline routing".into(),
                "PermissionGateway capability checks".into(),
                "Explicit required_approval on all opportunities".into(),
            ],
        );

        let approval_requirements = vec![
            "Every autonomy opportunity requires approval".into(),
            "Automation proposals contain no executable payloads".into(),
            "Valid path: Suggestion → Approval → Intent/Task/Command → Pipeline → Gateway → Service → Execution".into(),
        ];

        let constraints = vec![
            "autonomy ≠ authority".into(),
            "suggestions ≠ commands".into(),
            "confidence ≠ permission".into(),
            "proposals require approval".into(),
            "execution remains external".into(),
        ];

        let confidence = if opportunities.is_empty() {
            if evidence_links.is_empty() {
                25
            } else {
                45
            }
        } else {
            let avg: u32 = opportunities.iter().map(|o| o.confidence as u32).sum::<u32>()
                / opportunities.len() as u32;
            avg as u8
        };
        let uncertainty = if evidence_links.is_empty() {
            85
        } else if opportunities.is_empty() {
            60
        } else {
            100u8.saturating_sub(confidence).saturating_add(15).min(100)
        };

        let _ = now;
        let meta = CognitiveAutonomyMeta::new(
            workspace_id,
            now,
            confidence,
            uncertainty,
            opportunities.len(),
            proposals.len(),
            recommendations.len(),
        )
        .map_err(KernelError::from)?;

        let view = CognitiveAutonomyView {
            meta,
            opportunities,
            proposals,
            recommendations,
            risk_assessments: vec![safety],
            approval_requirements,
            confidence,
            uncertainty,
            evidence_links,
            constraints,
            authority_effect: CognitiveAutonomyView::AUTHORITY_EFFECT_NONE.into(),
        };
        view.validate().map_err(KernelError::from)?;
        Ok(view)
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
