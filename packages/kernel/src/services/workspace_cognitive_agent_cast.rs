//! Workspace Cognitive Agent Cast — Programme II Batch 7.
//!
//! Downstream evidence layer of specialised cognitive roles. Consumes prior
//! cognitive layers as source references and produces perspectives, critiques,
//! and syntheses. Agents are role representations — never actors or authorities.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{CognitiveAgentCastRepository, Database};
use workspace_domain::{
    default_cast_roles, ActorContext, AgentCritique, AgentPerspective, AgentSynthesis,
    CastEvidenceLink, CastStatus, CognitiveAgent, CognitiveAgentCastMeta,
    CognitiveAgentCastSnapshot, CognitiveAgentCastView,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AuditService, WorkspaceCognitiveGraphService, WorkspaceCognitiveOrchestrationService,
    WorkspaceLearningAdaptationService, WorkspacePlanningService, WorkspaceReasoningMemoryService,
};

pub(crate) struct WorkspaceCognitiveAgentCastService;

impl WorkspaceCognitiveAgentCastService {
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveAgentCastSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();
        let view = Self::compose_view(db, &workspace_id, &now)?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            guard
                .run_in_transaction(|database| {
                    let repo = CognitiveAgentCastRepository::new(database);
                    repo.supersede_current(&workspace_id, &now)?;
                    repo.persist_view(&view)?;
                    Ok(())
                })
                .map_err(KernelError::from)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.cognitive_agent_cast.snapshot_generated",
            &view.meta.cast_id,
            json!({
                "workspace_id": workspace_id,
                "agent_count": view.agents.len(),
                "perspective_count": view.perspectives.len(),
                "critique_count": view.critiques.len(),
                "synthesis_count": view.syntheses.len(),
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<CognitiveAgentCastSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = CognitiveAgentCastRepository::new(&guard);
        let metas = repo.list_meta(&workspace_id)?;
        let history = repo.list_history(&workspace_id)?;
        let history_count = repo.history_count(&workspace_id)?;
        let current = metas
            .into_iter()
            .find(|m| m.status == CastStatus::Current)
            .map(|m| repo.load_view(&m))
            .transpose()?;
        Ok(CognitiveAgentCastSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAgentCastValidation {
            message: "Cognitive agents cannot execute, dispatch, or launch".into(),
        })
    }

    pub(crate) fn attempt_call_command() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAgentCastValidation {
            message: "Cognitive agents cannot call commands directly".into(),
        })
    }

    pub(crate) fn attempt_request_permission() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAgentCastValidation {
            message: "Cognitive agents cannot request permissions".into(),
        })
    }

    pub(crate) fn attempt_create_task() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAgentCastValidation {
            message: "Cognitive agents cannot create tasks".into(),
        })
    }

    pub(crate) fn attempt_accept_decision() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAgentCastValidation {
            message: "Cognitive agents cannot accept decisions".into(),
        })
    }

    pub(crate) fn attempt_mutate_lifecycle() -> Result<()> {
        Err(KernelError::WorkspaceCognitiveAgentCastValidation {
            message: "Cognitive agents cannot mutate lifecycle state".into(),
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
            let repo = CognitiveAgentCastRepository::new(database);
            repo.supersede_current(&workspace_id, &now)?;
            repo.persist_view(&view)?;
            Err(workspace_database::DatabaseError::Migration(
                "forced cognitive agent cast transaction rollback".into(),
            ))
        });
        match err {
            Err(_) => Ok(()),
            Ok(()) => Err(KernelError::WorkspaceCognitiveAgentCastValidation {
                message: "expected forced rollback".into(),
            }),
        }
    }

    fn compose_view(
        db: &Arc<Mutex<Database>>,
        workspace_id: &str,
        now: &str,
    ) -> Result<CognitiveAgentCastView> {
        // Read-only composition — never call foreign generate paths.
        let planning = WorkspacePlanningService::load_snapshot(db, workspace_id)?;
        let reasoning = WorkspaceReasoningMemoryService::load_snapshot(db, workspace_id)?;
        let graph = WorkspaceCognitiveGraphService::load_snapshot(db, workspace_id)?;
        let orchestration =
            WorkspaceCognitiveOrchestrationService::load_snapshot(db, workspace_id)?;
        let learning = WorkspaceLearningAdaptationService::load_snapshot(db, workspace_id)?;

        let mut source_references = Vec::new();
        let mut evidence_links = Vec::new();

        if let Some(p) = &planning.current {
            source_references.push(CastEvidenceLink {
                external_ref: p.plan.id.clone(),
                kind: "planning".into(),
            });
        }
        if let Some(r) = &reasoning.current {
            source_references.push(CastEvidenceLink {
                external_ref: r.id.clone(),
                kind: "reasoning".into(),
            });
        }
        if let Some(g) = &graph.current {
            source_references.push(CastEvidenceLink {
                external_ref: g.meta.id.clone(),
                kind: "cognitive_graph".into(),
            });
        }
        if let Some(o) = &orchestration.current {
            source_references.push(CastEvidenceLink {
                external_ref: o.meta.orchestration_id.clone(),
                kind: "orchestration".into(),
            });
        }
        if let Some(l) = &learning.current {
            source_references.push(CastEvidenceLink {
                external_ref: l.meta.learning_id.clone(),
                kind: "learning".into(),
            });
        }

        let mut agents = Vec::new();
        let mut roles = Vec::new();
        for (name, role, purpose, specialisation, profile) in default_cast_roles() {
            let agent = CognitiveAgent::define(
                *name,
                *role,
                *purpose,
                *specialisation,
                vec![
                    "cannot execute".into(),
                    "cannot call commands".into(),
                    "cannot request permissions".into(),
                    "cannot mutate lifecycle".into(),
                    "perspective only".into(),
                ],
                *profile,
                now,
            )
            .map_err(KernelError::from)?;
            roles.push(role.to_string());
            agents.push(agent);
        }

        let mut perspectives = Vec::new();
        let mut critiques = Vec::new();

        for agent in &agents {
            let (observation, confidence, uncertainty, evidence) = match agent.role.as_str() {
                "planner_critic" => {
                    let gaps = planning
                        .current
                        .as_ref()
                        .map(|p| p.gaps.len())
                        .unwrap_or(0);
                    let refs: Vec<_> = source_references
                        .iter()
                        .filter(|l| l.kind == "planning")
                        .map(|l| l.external_ref.clone())
                        .collect();
                    if planning.current.is_none() {
                        (
                            "Planning snapshot missing — planning critique withheld (unknown)"
                                .into(),
                            40u8,
                            70u8,
                            refs,
                        )
                    } else {
                        (
                            format!(
                                "Planning presents {gaps} gap(s); sequencing assumptions warrant review"
                            ),
                            72,
                            35,
                            refs,
                        )
                    }
                }
                "risk_analyst" => {
                    let risk_n = planning
                        .current
                        .as_ref()
                        .map(|p| p.risks.len())
                        .unwrap_or(0);
                    let unc = reasoning.current.as_ref().map(|r| r.uncertainty).unwrap_or(50);
                    (
                        format!(
                            "Observed {risk_n} planning risk(s); reasoning uncertainty={unc}"
                        ),
                        68,
                        unc,
                        source_references
                            .iter()
                            .filter(|l| l.kind == "planning" || l.kind == "reasoning")
                            .map(|l| l.external_ref.clone())
                            .collect(),
                    )
                }
                "systems_architect" => {
                    let broken = graph
                        .current
                        .as_ref()
                        .map(|g| g.meta.broken_node_count + g.meta.broken_edge_count)
                        .unwrap_or(0);
                    (
                        format!(
                            "Cognitive graph structural integrity: {broken} broken reference(s)"
                        ),
                        if broken == 0 { 75 } else { 55 },
                        if broken == 0 { 30 } else { 55 },
                        source_references
                            .iter()
                            .filter(|l| l.kind == "cognitive_graph")
                            .map(|l| l.external_ref.clone())
                            .collect(),
                    )
                }
                "user_advocate" => {
                    let stale = orchestration
                        .current
                        .as_ref()
                        .map(|o| o.stale_items.len())
                        .unwrap_or(0);
                    (
                        format!(
                            "Orchestration reports {stale} stale item(s) that may affect user clarity"
                        ),
                        60,
                        45,
                        source_references
                            .iter()
                            .filter(|l| l.kind == "orchestration")
                            .map(|l| l.external_ref.clone())
                            .collect(),
                    )
                }
                "evidence_reviewer" => {
                    let missing = learning
                        .current
                        .as_ref()
                        .map(|l| {
                            l.observations
                                .iter()
                                .filter(|o| o.kind == "missing_outcome")
                                .count()
                        })
                        .unwrap_or(1);
                    (
                        format!(
                            "Learning layer reports {missing} missing-outcome observation(s); evidence completeness unknown where absent"
                        ),
                        65,
                        50,
                        source_references
                            .iter()
                            .filter(|l| l.kind == "learning")
                            .map(|l| l.external_ref.clone())
                            .collect(),
                    )
                }
                "efficiency_analyst" => {
                    let patterns = learning
                        .current
                        .as_ref()
                        .map(|l| l.patterns.len())
                        .unwrap_or(0);
                    (
                        format!(
                            "Learning surfaces {patterns} pattern(s) relevant to efficiency review"
                        ),
                        62,
                        40,
                        source_references
                            .iter()
                            .filter(|l| l.kind == "learning")
                            .map(|l| l.external_ref.clone())
                            .collect(),
                    )
                }
                _ => (
                    "Role contribution recorded without authority".into(),
                    50,
                    50,
                    vec![],
                ),
            };

            let perspective = AgentPerspective::new(
                agent.agent_id.clone(),
                observation,
                evidence.clone(),
                confidence,
                uncertainty,
                now,
            )
            .map_err(KernelError::from)?;
            evidence_links.extend(evidence.iter().map(|r| CastEvidenceLink {
                external_ref: r.clone(),
                kind: format!("perspective:{}", agent.role),
            }));
            perspectives.push(perspective);

            // Critiques are informational — never block systems.
            if agent.role == "planner_critic" && planning.current.is_some() {
                let plan_id = planning
                    .current
                    .as_ref()
                    .map(|p| p.plan.id.clone())
                    .unwrap_or_default();
                critiques.push(
                    AgentCritique::new(
                        agent.agent_id.clone(),
                        plan_id,
                        vec![
                            "Planning may understate dependency refresh needs".into(),
                            "Assumptions deserve explicit challenge before refresh".into(),
                        ],
                        "medium",
                        source_references
                            .iter()
                            .filter(|l| l.kind == "planning")
                            .map(|l| l.external_ref.clone())
                            .collect(),
                        70,
                    )
                    .map_err(KernelError::from)?,
                );
            }
            if agent.role == "risk_analyst"
                && reasoning
                    .current
                    .as_ref()
                    .map(|r| r.uncertainty > 60)
                    .unwrap_or(false)
            {
                let rid = reasoning.current.as_ref().unwrap().id.clone();
                critiques.push(
                    AgentCritique::new(
                        agent.agent_id.clone(),
                        rid.clone(),
                        vec!["Elevated reasoning uncertainty increases decision risk".into()],
                        "high",
                        vec![rid],
                        75,
                    )
                    .map_err(KernelError::from)?,
                );
            }
            if agent.role == "evidence_reviewer"
                && graph
                    .current
                    .as_ref()
                    .map(|g| g.meta.broken_node_count > 0)
                    .unwrap_or(false)
            {
                let gid = graph.current.as_ref().unwrap().meta.id.clone();
                critiques.push(
                    AgentCritique::new(
                        agent.agent_id.clone(),
                        gid.clone(),
                        vec!["Broken graph references weaken evidence chains".into()],
                        "medium",
                        vec![gid],
                        72,
                    )
                    .map_err(KernelError::from)?,
                );
            }
        }

        let included: Vec<_> = perspectives
            .iter()
            .map(|p| p.perspective_id.clone())
            .collect();
        let agreement_points = vec![
            "Cast contributions are observational evidence only".into(),
            "No agent may execute or self-authorise".into(),
        ];
        let mut disagreement_points = Vec::new();
        if critiques.iter().any(|c| c.risk_level == "high")
            && critiques.iter().any(|c| c.risk_level == "medium")
        {
            disagreement_points.push(
                "Risk severity weighting differs across critic perspectives".into(),
            );
        }
        let mut open_questions = vec![
            "Which evidence gaps should be refreshed first (orchestration decides ordering)?"
                .into(),
        ];
        if planning.current.is_none() {
            open_questions.push("Planning outcome unknown — cannot invent planning critique".into());
        }
        if learning.current.is_none() {
            open_questions.push("Learning evidence missing — lessons remain unknown".into());
        }

        let synth_confidence = if source_references.is_empty() { 35 } else { 60 };
        let synthesis = AgentSynthesis::new(
            included,
            agreement_points,
            disagreement_points,
            open_questions,
            synth_confidence,
        )
        .map_err(KernelError::from)?;

        let confidence = if source_references.is_empty() {
            30
        } else {
            let avg: u32 = perspectives.iter().map(|p| p.confidence as u32).sum::<u32>()
                / perspectives.len().max(1) as u32;
            avg as u8
        };
        let uncertainty = if source_references.is_empty() {
            80
        } else {
            let avg: u32 = perspectives.iter().map(|p| p.uncertainty as u32).sum::<u32>()
                / perspectives.len().max(1) as u32;
            avg as u8
        };

        let meta = CognitiveAgentCastMeta::new(
            workspace_id,
            now,
            confidence,
            uncertainty,
            agents.len(),
            perspectives.len(),
            critiques.len(),
            1,
        )
        .map_err(KernelError::from)?;

        let view = CognitiveAgentCastView {
            meta,
            agents,
            roles,
            perspectives,
            critiques,
            syntheses: vec![synthesis],
            confidence,
            uncertainty,
            evidence_links,
            source_references,
            authority_effect: CognitiveAgentCastView::AUTHORITY_EFFECT_NONE.into(),
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
