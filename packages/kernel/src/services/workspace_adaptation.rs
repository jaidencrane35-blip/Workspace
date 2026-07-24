//! Workspace Adaptation Proposal (Phase 5).
//!
//! Aggregates Pattern + Recommendation Engine + Operating State + Composition +
//! Environment + Continuity + Purpose into improvement proposals.
//! Never executes, never mutates layout/windows, never grants authority.
//! Accept returns Intent handoff only — Gateway remains required.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    adaptation_now_rfc3339, build_adaptation_summary, validate_adaptation_workspace_id,
    ActorContext, AdaptationActionResult, AdaptationEvidence, AdaptationHandoff, AdaptationImpact,
    AdaptationKind, AdaptationProposal, AdaptationStatus, AdaptationSummary, AdaptationTarget,
    AdaptationTargetKind, IntentContext, PatternKind, RecommendationKind, WorkspaceAdaptationState,
    WorkspaceAdaptationSummary, WorkspaceCompositionState, WorkspaceContinuityState,
    WorkspaceEnvironmentState, WorkspaceOperatingState, WorkspacePatternState,
    WorkspacePurposeState, WorkspaceRecommendationEngineState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceCompositionService,
    WorkspaceContinuityService, WorkspaceEnvironmentService, WorkspaceOperatingStateService,
    WorkspacePatternService, WorkspacePurposeService, WorkspaceRecommendationEngineService,
};

/// Process-local status overlays (review/accept/reject). Not a second workspace store;
/// never mutates layouts/tasks — status is informational only.
fn status_overlays() -> &'static Mutex<HashMap<String, AdaptationStatus>> {
    static OVERLAYS: OnceLock<Mutex<HashMap<String, AdaptationStatus>>> = OnceLock::new();
    OVERLAYS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn overlay_key(workspace_id: &str, proposal_id: &str) -> String {
    format!("{workspace_id}::{proposal_id}")
}

pub(crate) struct WorkspaceAdaptationService;

impl WorkspaceAdaptationService {
    /// Standalone generate — loads existing aggregators; does not invent state.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
    ) -> Result<WorkspaceAdaptationState> {
        let workspace_id = workspace_id.into();
        let patterns = WorkspacePatternService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let recommendations = WorkspaceRecommendationEngineService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let operating = WorkspaceOperatingStateService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let purpose = WorkspacePurposeService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let environment =
            WorkspaceEnvironmentService::generate(db, actor, workspace_id.clone())?;
        let continuity = {
            // Rebuild continuity via OS path inputs is heavy; use purpose/OS continuity focus.
            use crate::services::{DecisionQueueService, WorkspaceActivityGraphService};
            let dq = DecisionQueueService::aggregate_readonly(
                db,
                actor,
                orchestrated_plans,
                assistant_workflows,
                workspace_id.clone(),
            )?;
            let activity = WorkspaceActivityGraphService::generate_with_decision_queue(
                db,
                actor,
                orchestrated_plans,
                assistant_workflows,
                workspace_id.clone(),
                Some(&dq),
            )?;
            WorkspaceContinuityService::generate_with_inputs(
                db,
                actor,
                workspace_id.clone(),
                &dq,
                &activity,
            )?
        };
        let workflow =
            crate::services::WorkspaceIntentService::get_workflow_context_readonly(db, &workspace_id)?;
        let project = workflow
            .active_project_id
            .as_ref()
            .and_then(|id| crate::services::WorkspaceIntentService::get_project(db, id.as_str()).ok());
        let composition = {
            use crate::services::{DecisionQueueService, TaskGraphService, WorkspaceActivityGraphService};
            let dq = DecisionQueueService::aggregate_readonly(
                db,
                actor,
                orchestrated_plans,
                assistant_workflows,
                workspace_id.clone(),
            )?;
            let activity = WorkspaceActivityGraphService::generate_with_decision_queue(
                db,
                actor,
                orchestrated_plans,
                assistant_workflows,
                workspace_id.clone(),
                Some(&dq),
            )?;
            let task_graph = TaskGraphService::generate(db, actor, workspace_id.clone())?;
            WorkspaceCompositionService::generate_with_inputs(
                db,
                actor,
                workspace_id.clone(),
                &environment,
                Some(&task_graph),
                &continuity,
                &activity,
                &workflow,
                &dq,
                project.as_ref(),
            )?
        };
        Self::generate_with_inputs(
            db,
            actor,
            &workspace_id,
            &patterns,
            &recommendations,
            &operating,
            &composition,
            &environment,
            &continuity,
            &purpose,
        )
    }

    /// Preferred path — Intelligence injects shared aggregator inputs.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        workspace_id: impl Into<String>,
        patterns: &WorkspacePatternState,
        recommendations: &WorkspaceRecommendationEngineState,
        operating: &WorkspaceOperatingState,
        composition: &WorkspaceCompositionState,
        environment: &WorkspaceEnvironmentState,
        continuity: &WorkspaceContinuityState,
        purpose: &WorkspacePurposeState,
    ) -> Result<WorkspaceAdaptationState> {
        let workspace_id =
            validate_adaptation_workspace_id(workspace_id).map_err(KernelError::from)?;
        let ws = workspace_id.as_str();
        let label = if !purpose.label.is_empty() {
            purpose.label.clone()
        } else {
            patterns.label.clone()
        };

        let mut proposals = Vec::new();
        let mut evidence = Vec::new();
        let mut seen = HashSet::new();

        // Application grouping / layout from ApplicationPattern + EnvironmentPattern.
        for pattern in patterns.patterns.iter().filter(|p| {
            matches!(
                p.kind,
                PatternKind::ApplicationPattern | PatternKind::EnvironmentPattern
            )
        }) {
            let id = format!("adaptation:grouping:{}", pattern.id);
            if !seen.insert(id.clone()) {
                continue;
            }
            let kind = if pattern.kind == PatternKind::ApplicationPattern {
                AdaptationKind::ApplicationGrouping
            } else {
                AdaptationKind::LayoutImprovement
            };
            proposals.push(AdaptationProposal {
                id,
                kind,
                title: format!("Consider preset: {}", pattern.title),
                reason: format!(
                    "Pattern observation suggests a recurring working set: {}",
                    pattern.observation
                ),
                evidence: pattern
                    .evidence
                    .iter()
                    .map(|e| AdaptationEvidence {
                        id: e.id.clone(),
                        source_model: format!("pattern:{}", e.source_model),
                        source_ref: e.source_ref.clone(),
                        summary: e.summary.clone(),
                    })
                    .collect(),
                impact: AdaptationImpact {
                    benefit: "Reduce setup time by restoring a familiar application group.".into(),
                    risk: "Creating a preset would change how the Workspace is organized later — only via Intent + Gateway.".into(),
                },
                target: AdaptationTarget {
                    kind: AdaptationTargetKind::ApplicationGroup,
                    ref_id: pattern.id.clone(),
                    label: composition.label.clone(),
                },
                status: AdaptationStatus::Proposed,
                related_pattern_id: Some(pattern.id.clone()),
                related_recommendation_id: None,
                authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
            });
        }

        // Context restoration from Continuity interrupted + RestoreContext recommendations.
        if !continuity.interrupted_work.is_empty() {
            let facet = &continuity.interrupted_work[0];
            let id = format!("adaptation:restore:{}", facet.id);
            if seen.insert(id.clone()) {
                proposals.push(AdaptationProposal {
                    id,
                    kind: AdaptationKind::ContextRestoration,
                    title: format!("Restore composition for \"{}\"", facet.title),
                    reason: format!(
                        "Continuity reports interrupted work \"{}\"; Operating State purpose is \"{}\".",
                        facet.title, operating.context.purpose_label
                    ),
                    evidence: vec![
                        AdaptationEvidence {
                            id: format!("ev:continuity:{}", facet.id),
                            source_model: "continuity".into(),
                            source_ref: facet.id.to_string(),
                            summary: facet.title.clone(),
                        },
                        AdaptationEvidence {
                            id: format!("ev:operating:{ws}"),
                            source_model: "operating_state".into(),
                            source_ref: operating.workspace_id.clone(),
                            summary: operating.summary.clone(),
                        },
                    ],
                    impact: AdaptationImpact {
                        benefit: "Resume interrupted work faster with the prior composition."
                            .into(),
                        risk: "Restoring context must not move windows automatically — human Intent required."
                            .into(),
                    },
                    target: AdaptationTarget {
                        kind: AdaptationTargetKind::Continuity,
                        ref_id: facet.id.to_string(),
                        label: facet.title.clone(),
                    },
                    status: AdaptationStatus::Proposed,
                    related_pattern_id: None,
                    related_recommendation_id: recommendations
                        .candidates
                        .iter()
                        .find(|c| c.kind == RecommendationKind::RestoreContext)
                        .map(|c| c.id.clone()),
                    authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // Workspace organization from Composition gaps / Environment disconnected.
        if composition.missing_application_count > 0 || !composition.gaps.is_empty() {
            let id = format!("adaptation:organize:{ws}");
            if seen.insert(id.clone()) {
                proposals.push(AdaptationProposal {
                    id,
                    kind: AdaptationKind::WorkspaceOrganization,
                    title: format!("Improve organization of \"{}\"", composition.label),
                    reason: format!(
                        "Composition reports {} missing application(s) for Purpose \"{}\".",
                        composition.missing_application_count, purpose.label
                    ),
                    evidence: vec![
                        AdaptationEvidence {
                            id: format!("ev:composition:{ws}"),
                            source_model: "composition".into(),
                            source_ref: composition.workspace_id.clone(),
                            summary: composition.summary.clone(),
                        },
                        AdaptationEvidence {
                            id: format!("ev:environment:{ws}"),
                            source_model: "environment".into(),
                            source_ref: environment.workspace_id.clone(),
                            summary: environment.summary.clone(),
                        },
                    ],
                    impact: AdaptationImpact {
                        benefit: "A coherent composition makes Purpose work easier to continue."
                            .into(),
                        risk: "Reorganization could change membership — never applied without Intent + Gateway."
                            .into(),
                    },
                    target: AdaptationTarget {
                        kind: AdaptationTargetKind::Composition,
                        ref_id: composition.workspace_id.clone(),
                        label: composition.label.clone(),
                    },
                    status: AdaptationStatus::Proposed,
                    related_pattern_id: None,
                    related_recommendation_id: recommendations
                        .candidates
                        .iter()
                        .find(|c| c.kind == RecommendationKind::ReorganizeWorkspace)
                        .map(|c| c.id.clone()),
                    authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        } else if environment.disconnected_work {
            let id = format!("adaptation:reconnect:{ws}");
            if seen.insert(id.clone()) {
                proposals.push(AdaptationProposal {
                    id,
                    kind: AdaptationKind::WorkspaceOrganization,
                    title: "Reconnect desktop apps to active work".into(),
                    reason: "Environment Model reports active work appears disconnected from open windows."
                        .into(),
                    evidence: vec![AdaptationEvidence {
                        id: format!("ev:environment:disconnected:{ws}"),
                        source_model: "environment".into(),
                        source_ref: environment.workspace_id.clone(),
                        summary: environment.summary.clone(),
                    }],
                    impact: AdaptationImpact {
                        benefit: "Aligning apps with Purpose reduces Continuity friction.".into(),
                        risk: "Must not move or launch windows from Adaptation — Intent required."
                            .into(),
                    },
                    target: AdaptationTarget {
                        kind: AdaptationTargetKind::Environment,
                        ref_id: environment.workspace_id.clone(),
                        label: "desktop environment".into(),
                    },
                    status: AdaptationStatus::Proposed,
                    related_pattern_id: None,
                    related_recommendation_id: None,
                    authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // Workflow shortcut from WorkflowPattern / Continuity focus.
        if let Some(pattern) = patterns
            .patterns
            .iter()
            .find(|p| p.kind == PatternKind::WorkflowPattern)
        {
            let id = format!("adaptation:workflow:{}", pattern.id);
            if seen.insert(id.clone()) {
                proposals.push(AdaptationProposal {
                    id,
                    kind: AdaptationKind::WorkflowShortcut,
                    title: format!("Capture workflow shortcut: {}", pattern.title),
                    reason: format!(
                        "Workflow pattern for Purpose \"{}\": {}",
                        purpose.label, pattern.observation
                    ),
                    evidence: pattern
                        .evidence
                        .iter()
                        .map(|e| AdaptationEvidence {
                            id: e.id.clone(),
                            source_model: format!("pattern:{}", e.source_model),
                            source_ref: e.source_ref.clone(),
                            summary: e.summary.clone(),
                        })
                        .collect(),
                    impact: AdaptationImpact {
                        benefit: "A shortcut could reduce repeated setup for known workflows."
                            .into(),
                        risk: "Shortcuts must not become hidden automation — human Intent + Gateway only."
                            .into(),
                    },
                    target: AdaptationTarget {
                        kind: AdaptationTargetKind::Purpose,
                        ref_id: purpose.workspace_id.clone(),
                        label: purpose.label.clone(),
                    },
                    status: AdaptationStatus::Proposed,
                    related_pattern_id: Some(pattern.id.clone()),
                    related_recommendation_id: None,
                    authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        // Task organization from TaskPattern.
        if let Some(pattern) = patterns
            .patterns
            .iter()
            .find(|p| p.kind == PatternKind::TaskPattern)
        {
            let id = format!("adaptation:tasks:{}", pattern.id);
            if seen.insert(id.clone()) {
                proposals.push(AdaptationProposal {
                    id,
                    kind: AdaptationKind::TaskOrganization,
                    title: "Clarify recurring task ordering".into(),
                    reason: format!(
                        "Task pattern: {}. Adaptation proposes organizing task structure — not editing tasks here.",
                        pattern.observation
                    ),
                    evidence: pattern
                        .evidence
                        .iter()
                        .map(|e| AdaptationEvidence {
                            id: e.id.clone(),
                            source_model: format!("pattern:{}", e.source_model),
                            source_ref: e.source_ref.clone(),
                            summary: e.summary.clone(),
                        })
                        .collect(),
                    impact: AdaptationImpact {
                        benefit: "Clearer task order can reduce Continuity and Attention drag."
                            .into(),
                        risk: "Task changes require Intent through the Task Graph command path."
                            .into(),
                    },
                    target: AdaptationTarget {
                        kind: AdaptationTargetKind::TaskGraph,
                        ref_id: ws.to_string(),
                        label: "Task Graph".into(),
                    },
                    status: AdaptationStatus::Proposed,
                    related_pattern_id: Some(pattern.id.clone()),
                    related_recommendation_id: None,
                    authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }

        proposals.sort_by(|a, b| {
            kind_rank(a.kind)
                .cmp(&kind_rank(b.kind))
                .then_with(|| a.id.cmp(&b.id))
        });
        if proposals.len() > 12 {
            proposals.truncate(12);
        }

        // Apply process-local review overlays (informational only).
        if let Ok(guard) = status_overlays().lock() {
            for proposal in &mut proposals {
                if let Some(status) = guard.get(&overlay_key(ws, &proposal.id)) {
                    proposal.status = *status;
                }
            }
        }

        evidence.push(format!("Pattern count: {}", patterns.pattern_count));
        evidence.push(format!(
            "Recommendation candidates: {}",
            recommendations.candidate_count
        ));
        evidence.push(format!("Operating signals: {}", operating.signal_count));
        evidence.push(format!("Composition: {}", composition.label));

        let open_count = proposals
            .iter()
            .filter(|p| {
                matches!(
                    p.status,
                    AdaptationStatus::Proposed | AdaptationStatus::Reviewed
                )
            })
            .count();
        let top_line = proposals
            .first()
            .map(|p| p.title.clone())
            .unwrap_or_else(|| "No adaptation proposals right now.".into());
        let adaptation_summary = AdaptationSummary {
            headline: format!("Possible improvements for \"{label}\""),
            top_proposal_line: top_line.clone(),
            review_line: format!("{open_count} open proposal(s) awaiting human decision"),
            narrative: format!(
                "Adaptation proposes possible Workspace improvements for \"{label}\". \
                 Top: {top_line}. Distinct from Recommendation Engine next-step suggestions. \
                 Human decides; Intent → Gateway → Execution remains required."
            ),
        };

        let explanation = format!(
            "Adaptation proposals for \"{label}\" are projected from Pattern, Recommendation Engine, \
             Operating State, Composition, Environment, Continuity, and Purpose. They are proposals \
             only — distinct from recommendations and Decision Engine accept/handoff. Never execute."
        );
        let summary = build_adaptation_summary(&label, proposals.len());

        let state = WorkspaceAdaptationState {
            workspace_id: ws.to_string(),
            generated_at: adaptation_now_rfc3339(),
            label,
            proposal_count: proposals.len(),
            open_count,
            proposals,
            adaptation_summary,
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceAdaptationState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceAdaptationState,
        limit: usize,
    ) -> WorkspaceAdaptationSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn review(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        proposal_id: impl Into<String>,
    ) -> Result<AdaptationActionResult> {
        Self::transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            proposal_id,
            AdaptationStatus::Reviewed,
            "workspace.adaptation.reviewed",
            false,
        )
    }

    pub(crate) fn accept(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        proposal_id: impl Into<String>,
    ) -> Result<AdaptationActionResult> {
        Self::transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            proposal_id,
            AdaptationStatus::Accepted,
            "workspace.adaptation.accepted",
            true,
        )
    }

    pub(crate) fn reject(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        proposal_id: impl Into<String>,
    ) -> Result<AdaptationActionResult> {
        Self::transition(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id,
            proposal_id,
            AdaptationStatus::Rejected,
            "workspace.adaptation.rejected",
            false,
        )
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceAdaptationError::CannotExecute,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn transition(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        proposal_id: impl Into<String>,
        to: AdaptationStatus,
        audit_event: &str,
        include_handoff: bool,
    ) -> Result<AdaptationActionResult> {
        let workspace_id = workspace_id.into();
        let proposal_id = proposal_id.into();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
        )?;
        let proposal = state
            .proposals
            .iter()
            .find(|p| p.id == proposal_id)
            .cloned()
            .ok_or_else(|| {
                KernelError::from(workspace_domain::WorkspaceAdaptationError::NotFound)
            })?;

        if !proposal.status.allows_transition(to) {
            return Err(KernelError::from(
                workspace_domain::WorkspaceAdaptationError::InvalidTransition {
                    from: proposal.status.as_str().into(),
                    to: to.as_str().into(),
                },
            ));
        }

        let mut updated = proposal;
        updated.status = to;
        if let Ok(mut guard) = status_overlays().lock() {
            guard.insert(overlay_key(&workspace_id, &updated.id), to);
        }
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            audit_event,
            true,
            json!({
                "workspace_id": workspace_id,
                "proposal_id": updated.id,
                "kind": updated.kind.as_str(),
                "status": updated.status.as_str(),
                "authority_effect": "none",
            })
            .to_string(),
        )?;

        let handoff = if include_handoff {
            Some(AdaptationHandoff {
                proposal_id: updated.id.clone(),
                next_command: AdaptationProposal::HANDOFF_SUBMIT_ASSISTANT_GOAL.into(),
                goal_statement: format!(
                    "Consider adaptation: {} — {}. Must plan via Intent; never apply from Adaptation.",
                    updated.title, updated.reason
                ),
                workspace_id: workspace_id.clone(),
                note: "Adaptation accepted as a human decision to consider. Call submit_assistant_goal to plan — Adaptation never executes, mutates layout, or bypasses Permission Gateway.".into(),
                authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
            })
        } else {
            None
        };

        Ok(AdaptationActionResult {
            proposal: Some(updated),
            handoff,
            authority_effect: AdaptationProposal::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceAdaptationState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.adaptation.proposal_generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "proposal_count": state.proposal_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn kind_rank(kind: AdaptationKind) -> u8 {
    match kind {
        AdaptationKind::ContextRestoration => 0,
        AdaptationKind::ApplicationGrouping => 1,
        AdaptationKind::LayoutImprovement => 2,
        AdaptationKind::WorkspaceOrganization => 3,
        AdaptationKind::WorkflowShortcut => 4,
        AdaptationKind::TaskOrganization => 5,
    }
}
