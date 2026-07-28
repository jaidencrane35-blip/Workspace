//! Workspace Planning Engine — Programme II Batch 2.
//!
//! Higher-order, durable, permanently non-executing planning artefacts.
//! Composes Cognitive Model, Intent, Task Graph, RE, DE, Attention, Purpose,
//! and Memory. Never owns those systems' lifecycles. Never executes.

use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use workspace_database::{Database, WorkspacePlanningRepository};
use workspace_domain::{
    ActorContext, CognitiveNodeKind, CognitiveNodeStatus, IntentContext, PlanningAlternative,
    PlanningAssumption, PlanningConfidence, PlanningConstraintReference, PlanningDependency,
    PlanningEvidenceReference, PlanningExplanation, PlanningGap, PlanningHistoryEntry,
    PlanningPlan, PlanningPlanStatus, PlanningProposal, PlanningRisk, PlanningSection,
    PlanningSnapshot, PlanningStep,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AiMemoryService, AssistantWorkflowStore, AuditService, DecisionEngineService,
    OrchestratedPlanStore, TaskGraphService, WorkspaceAttentionService,
    WorkspaceCognitiveModelService, WorkspaceIntentService, WorkspacePurposeService,
    WorkspaceRecommendationEngineService,
};

pub(crate) struct WorkspacePlanningService;

impl WorkspacePlanningService {
    /// Generate and persist a new planning snapshot. Supersedes any active plan.
    /// Never dispatches, executes, claims, or mutates foreign lifecycles.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<PlanningSnapshot> {
        let workspace_id = workspace_id.into();
        let now = Utc::now().to_rfc3339();

        let cognitive =
            WorkspaceCognitiveModelService::generate_model(db, workspace_id.clone())?;
        let goals = WorkspaceIntentService::list_goals(db, &workspace_id, 20).unwrap_or_default();
        let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone())?;
        let memory =
            AiMemoryService::list_active(db, actor, Some(workspace_id.as_str()), 10)
                .unwrap_or_default();

        let attention = WorkspaceAttentionService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )
        .ok();
        let purpose = WorkspacePurposeService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )
        .ok();
        let recommendations = WorkspaceRecommendationEngineService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )
        .ok();
        let decisions = DecisionEngineService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )
        .ok();

        let proposal = Self::compose_proposal(
            &workspace_id,
            &now,
            &cognitive,
            &goals,
            &task_graph,
            &memory,
            attention.as_ref(),
            purpose.as_ref(),
            recommendations.as_ref(),
            decisions.as_ref(),
        )?;

        {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            let repo = WorkspacePlanningRepository::new(&guard);
            repo.supersede_active(&workspace_id, &now)?;
            repo.persist_proposal(&proposal)?;
        }

        Self::audit(
            db,
            actor,
            "workspace.planning.snapshot_generated",
            &proposal.plan.id,
            json!({
                "workspace_id": workspace_id,
                "step_count": proposal.steps.len(),
                "confidence": proposal.plan.confidence,
                "uncertainty": proposal.plan.uncertainty,
                "authority_effect": "none",
            }),
        )?;

        Self::load_snapshot(db, workspace_id)
    }

    /// Reconstruct durable planning snapshot (restart continuity — no replay).
    pub(crate) fn load_snapshot(
        db: &Arc<Mutex<Database>>,
        workspace_id: impl Into<String>,
    ) -> Result<PlanningSnapshot> {
        let workspace_id = workspace_id.into();
        let guard = db
            .lock()
            .map_err(|_| KernelError::lock_poisoned("database"))?;
        let repo = WorkspacePlanningRepository::new(&guard);
        let plans = repo.list_plans(&workspace_id)?;
        let mut current = None;
        let mut history = Vec::new();
        for plan in plans {
            match plan.status {
                PlanningPlanStatus::Active => {
                    if current.is_none() {
                        current = Some(repo.load_proposal(&plan)?);
                    }
                }
                PlanningPlanStatus::Superseded | PlanningPlanStatus::Abandoned => {
                    if let Some(entry) = PlanningHistoryEntry::from_plan(&plan) {
                        history.push(entry);
                    }
                }
            }
        }
        let history_count = history.len();
        Ok(PlanningSnapshot::assemble(
            workspace_id,
            current,
            history,
            history_count,
            Utc::now().to_rfc3339(),
        ))
    }

    /// Architecture guard — planning must never execute.
    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::WorkspacePlanningValidation {
            message: "WorkspacePlanningService cannot execute, dispatch, claim, or launch"
                .into(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn compose_proposal(
        workspace_id: &str,
        now: &str,
        cognitive: &workspace_domain::CognitiveModelState,
        goals: &[workspace_domain::WorkGoal],
        task_graph: &workspace_domain::TaskGraph,
        memory: &[workspace_domain::MemoryEntry],
        attention: Option<&workspace_domain::WorkspaceAttentionState>,
        purpose: Option<&workspace_domain::WorkspacePurposeState>,
        recommendations: Option<&workspace_domain::WorkspaceRecommendationEngineState>,
        decisions: Option<&workspace_domain::DecisionEngineState>,
    ) -> Result<PlanningProposal> {
        let focus_nodes: Vec<_> = cognitive
            .nodes
            .iter()
            .filter(|n| n.is_current_focus)
            .cloned()
            .collect();
        let objectives: Vec<_> = cognitive
            .nodes
            .iter()
            .filter(|n| {
                n.kind == CognitiveNodeKind::Objective
                    && n.status == CognitiveNodeStatus::Active
            })
            .cloned()
            .collect();
        let initiatives: Vec<_> = cognitive
            .nodes
            .iter()
            .filter(|n| n.kind == CognitiveNodeKind::Initiative)
            .cloned()
            .collect();
        let constraints: Vec<_> = cognitive
            .nodes
            .iter()
            .filter(|n| n.kind == CognitiveNodeKind::Constraint)
            .cloned()
            .collect();
        let risks_nodes: Vec<_> = cognitive
            .nodes
            .iter()
            .filter(|n| n.kind == CognitiveNodeKind::Risk)
            .cloned()
            .collect();

        let title = if let Some(focus) = focus_nodes.first() {
            format!("Plan: {}", focus.title)
        } else if let Some(obj) = objectives.first() {
            format!("Plan: {}", obj.title)
        } else if let Some(goal) = goals.first() {
            format!("Plan toward: {}", goal.description)
        } else {
            "Workspace planning snapshot".into()
        };

        let summary = format!(
            "Sequenced from Cognitive Model ({} nodes), Intent ({} goals), Task Graph ({} open), Attention/Purpose/RE/DE references.",
            cognitive.nodes.len(),
            goals.len(),
            task_graph
                .nodes
                .iter()
                .filter(|n| n.task.status.is_open())
                .count()
        );

        let mut draft_plan = PlanningPlan::new(
            workspace_id,
            &title,
            &summary,
            50,
            50,
            now,
        )
        .map_err(KernelError::from)?;

        let mut steps = Vec::new();
        let mut ordinal = 0u32;

        for focus in &focus_nodes {
            let step = PlanningStep::new(
                &draft_plan.id,
                workspace_id,
                ordinal,
                format!("Advance focus: {}", focus.title),
                "Current cognitive focus is the primary sequencing anchor.",
                vec![focus.id.clone()],
                focus.confidence,
                focus.uncertainty,
            )
            .map_err(KernelError::from)?;
            steps.push(step);
            ordinal += 1;
        }

        for obj in objectives.iter().take(5) {
            if focus_nodes.iter().any(|f| f.id == obj.id) {
                continue;
            }
            let step = PlanningStep::new(
                &draft_plan.id,
                workspace_id,
                ordinal,
                format!("Pursue objective: {}", obj.title),
                "Active objective ordered by cognitive importance.",
                vec![obj.id.clone()],
                obj.confidence,
                obj.uncertainty,
            )
            .map_err(KernelError::from)?;
            steps.push(step);
            ordinal += 1;
        }

        for init in initiatives.iter().take(3) {
            let step = PlanningStep::new(
                &draft_plan.id,
                workspace_id,
                ordinal,
                format!("Coordinate initiative: {}", init.title),
                "Initiative provides multi-objective coordination framing.",
                vec![init.id.clone()],
                init.confidence,
                init.uncertainty,
            )
            .map_err(KernelError::from)?;
            steps.push(step);
            ordinal += 1;
        }

        for node in task_graph.nodes.iter().filter(|n| n.task.status.is_open()).take(5) {
            let step = PlanningStep::new(
                &draft_plan.id,
                workspace_id,
                ordinal,
                format!("Address task: {}", node.task.title),
                format!(
                    "Open Task Graph node ({}) referenced — Task Graph remains lifecycle owner.",
                    node.task.status.as_str()
                ),
                vec![format!("task:{}", node.task.id.as_str())],
                65,
                35,
            )
            .map_err(KernelError::from)?;
            steps.push(step);
            ordinal += 1;
        }

        if steps.is_empty() {
            let step = PlanningStep::new(
                &draft_plan.id,
                workspace_id,
                0,
                "Establish cognitive focus and objectives",
                "Insufficient structured inputs — first planning step is to enrich the Cognitive Model.",
                vec![],
                40,
                60,
            )
            .map_err(KernelError::from)?;
            steps.push(step);
        }

        let conf = PlanningConfidence::from_steps(&steps);
        draft_plan.confidence = conf.confidence;
        draft_plan.uncertainty = conf.uncertainty;

        let mut dependencies = Vec::new();
        for window in steps.windows(2) {
            dependencies.push(PlanningDependency::new(
                &draft_plan.id,
                workspace_id,
                &window[0].id,
                &window[1].id,
                "before",
            ));
        }

        let mut assumptions = Vec::new();
        for node in cognitive.nodes.iter().filter(|n| n.confidence < 50).take(5) {
            assumptions.push(
                PlanningAssumption::new(
                    &draft_plan.id,
                    workspace_id,
                    format!("Low-confidence understanding of '{}'", node.title),
                    node.confidence,
                    vec![node.id.clone()],
                )
                .map_err(KernelError::from)?,
            );
        }
        if assumptions.is_empty() {
            assumptions.push(
                PlanningAssumption::new(
                    &draft_plan.id,
                    workspace_id,
                    "Referenced Intent/Task Graph state remains current",
                    70,
                    goals
                        .iter()
                        .take(1)
                        .map(|g| format!("work_goal:{}", g.id.as_str()))
                        .collect(),
                )
                .map_err(KernelError::from)?,
            );
        }

        let mut risks = Vec::new();
        for node in &risks_nodes {
            risks.push(
                PlanningRisk::new(
                    &draft_plan.id,
                    workspace_id,
                    format!("Cognitive risk: {}", node.title),
                    node.uncertainty,
                    vec![node.id.clone()],
                )
                .map_err(KernelError::from)?,
            );
        }
        if let Some(att) = attention {
            for item in att.items.iter().take(3) {
                risks.push(
                    PlanningRisk::new(
                        &draft_plan.id,
                        workspace_id,
                        format!("Attention signal: {}", item.title),
                        50,
                        vec![format!("attention:{}", item.id.as_str())],
                    )
                    .map_err(KernelError::from)?,
                );
            }
        }

        let mut gaps = Vec::new();
        if cognitive.nodes.is_empty() {
            gaps.push(
                PlanningGap::new(
                    &draft_plan.id,
                    workspace_id,
                    "Cognitive Model is empty — goals/objectives missing",
                    vec![],
                )
                .map_err(KernelError::from)?,
            );
        }
        if goals.is_empty() {
            gaps.push(
                PlanningGap::new(
                    &draft_plan.id,
                    workspace_id,
                    "No WorkGoals in Intent — purpose anchoring is weak",
                    vec![],
                )
                .map_err(KernelError::from)?,
            );
        }
        if memory.is_empty() {
            gaps.push(
                PlanningGap::new(
                    &draft_plan.id,
                    workspace_id,
                    "No active memory summaries available for planning context",
                    vec![],
                )
                .map_err(KernelError::from)?,
            );
        }

        let constraint_refs: Vec<_> = constraints
            .iter()
            .map(|c| PlanningConstraintReference {
                external_ref: c.id.clone(),
                note: c.title.clone(),
            })
            .collect();

        let mut evidence_refs = Vec::new();
        for g in goals.iter().take(5) {
            evidence_refs.push(PlanningEvidenceReference {
                external_ref: format!("work_goal:{}", g.id.as_str()),
                kind: "intent".into(),
            });
        }
        for m in memory.iter().take(5) {
            evidence_refs.push(PlanningEvidenceReference {
                external_ref: format!("memory:{}", m.id.as_str()),
                kind: "memory".into(),
            });
        }
        if let Some(p) = purpose {
            evidence_refs.push(PlanningEvidenceReference {
                external_ref: format!("purpose:{}", p.workspace_id),
                kind: "purpose".into(),
            });
        }
        if let Some(re) = recommendations {
            for c in re.candidates.iter().take(3) {
                evidence_refs.push(PlanningEvidenceReference {
                    external_ref: format!("recommendation:{}", c.id),
                    kind: "recommendation".into(),
                });
            }
        }
        if let Some(de) = decisions {
            for c in de.candidates.iter().take(3) {
                evidence_refs.push(PlanningEvidenceReference {
                    external_ref: format!("decision:{}", c.id.as_str()),
                    kind: "decision".into(),
                });
            }
        }
        if let Some(att) = attention {
            evidence_refs.push(PlanningEvidenceReference {
                external_ref: format!("attention:{}", att.workspace_id),
                kind: "attention".into(),
            });
        }

        let step_ids: Vec<_> = steps.iter().map(|s| s.id.clone()).collect();
        let sections = if step_ids.is_empty() {
            vec![]
        } else {
            vec![PlanningSection::new("Primary sequence", step_ids.clone())
                .map_err(KernelError::from)?]
        };

        let alternatives = if steps.len() >= 2 {
            vec![PlanningAlternative::new(
                "Defer secondary tasks",
                "Focus-first alternative: postpone Task Graph steps until cognitive objectives stabilize.",
                steps.iter().take(1).map(|s| s.id.clone()).collect(),
            )
            .map_err(KernelError::from)?]
        } else {
            vec![]
        };

        let explanation = PlanningExplanation {
            summary: summary.clone(),
            why: "Planning sequences Cognitive focus → objectives → initiatives → open tasks, citing Intent/RE/DE/Attention/Purpose/Memory by reference only.".into(),
            next: "Operator may inspect rationale/risks/assumptions. Execution, acceptance, and Task/Intent mutation begin only via their owning command paths.".into(),
        };

        Ok(PlanningProposal {
            plan: draft_plan,
            steps,
            sections,
            assumptions,
            risks,
            gaps,
            dependencies,
            alternatives,
            constraint_refs,
            evidence_refs,
            explanation,
            authority_effect: PlanningProposal::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        event: &str,
        subject_id: &str,
        detail: serde_json::Value,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            event,
            true,
            json!({
                "subject_id": subject_id,
                "detail": detail,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}
