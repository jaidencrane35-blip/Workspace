//! Workspace Transition Engine (Phase 6).
//!
//! Explains movement between work states from existing projections.
//! Owns nothing. Never restores, automates, schedules, or executes.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_transition_summary, transition_now_rfc3339, validate_transition_workspace_id,
    ActorContext, IntentContext, TransitionAssociation, TransitionConfidence, TransitionEvidence,
    TransitionKind, TransitionRelationKind, TransitionRelationship, TransitionSummary,
    WorkspaceExperienceState, WorkspaceIntelligenceState, WorkspaceMilestoneState,
    WorkspaceNavigationState, WorkspaceSessionState, WorkspaceTransition,
    WorkspaceTransitionComparison, WorkspaceTransitionState, WorkspaceTransitionSummary,
    WorkspaceTransitionValidation, WorkspaceWorkContextState, WorkspaceWorkingStyleState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceMilestoneService, WorkspaceNavigationService,
    WorkspaceSessionService, WorkspaceWorkContextService, WorkspaceWorkingStyleService,
};

pub(crate) struct WorkspaceTransitionService;

impl WorkspaceTransitionService {
    /// Standalone generate — loads Intelligence stack, then projects transitions.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceTransitionState> {
        let workspace_id = workspace_id.into();
        let workspace_name = workspace_name.into();
        let health_label = health_label.into();
        let intelligence = WorkspaceIntelligenceService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            workspace_name.clone(),
            health_label,
        )?;
        let session =
            WorkspaceSessionService::generate_with_inputs(db, actor, &intelligence)?;
        let experience = WorkspaceExperienceService::generate_with_inputs(db, actor, &session)?;
        let work_context = WorkspaceWorkContextService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
        )?;
        let navigation = WorkspaceNavigationService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
        )?;
        let milestones = WorkspaceMilestoneService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
        )?;
        let working_style = WorkspaceWorkingStyleService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
        )?;
        Self::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
            &working_style,
        )
    }

    /// Preferred path — consume already-assembled projections (never regenerate).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
        session: &WorkspaceSessionState,
        experience: &WorkspaceExperienceState,
        work_context: &WorkspaceWorkContextState,
        navigation: &WorkspaceNavigationState,
        milestones: &WorkspaceMilestoneState,
        working_style: &WorkspaceWorkingStyleState,
    ) -> Result<WorkspaceTransitionState> {
        let _ = validate_transition_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = session.label.clone();

        let previous_state = describe_previous(intelligence, session);
        let current_state = describe_current(intelligence, session, work_context);
        let related_context = collect_contexts(work_context);
        let related_milestones = collect_milestones(milestones);
        let open_decisions = collect_decisions(intelligence);
        let interrupted = collect_interrupted(intelligence);

        let mut transitions = Vec::new();

        // Returning / interrupted continuity.
        if intelligence.continuity.interrupted_count > 0
            || intelligence.continuity.current_focus.is_some()
        {
            let focus = intelligence
                .continuity
                .current_focus
                .as_ref()
                .map(|f| f.title.clone())
                .unwrap_or_else(|| previous_state.clone());
            // Keep title/kind stable across regenerates — interrupted_count can fluctuate
            // when projection audits feed Activity/Continuity between consecutive generates.
            transitions.push(transition(
                "tr-returning",
                TransitionKind::ReturningToWork,
                "Returning to prior work".into(),
                previous_state.clone(),
                current_state.clone(),
                vec![
                    format!("Continuity focus: {focus}"),
                    format!(
                        "Interrupted signals: {}",
                        intelligence.continuity.interrupted_count
                    ),
                ],
                vec![TransitionEvidence {
                    label: "Continuity".into(),
                    source_projection: "continuity".into(),
                    source_ref: intelligence.continuity.session_anchor.clone(),
                    why: "Continuity remains unfinished-work SoT; Transition only explains return"
                        .into(),
                }],
                related_context.clone(),
                related_milestones.clone(),
                open_decisions.clone(),
                interrupted.clone(),
                TransitionConfidence::High,
                "Explains returning to prior work without restoring windows or launching apps."
                    .into(),
                "Derived from Continuity + Session — never performs restoration.".into(),
            ));
        }

        // Context enter / leave / switch from Work Context + Navigation.
        if !work_context.contexts.is_empty() {
            let primary = work_context
                .contexts
                .first()
                .map(|c| c.context_type.as_str())
                .unwrap_or("unknown");
            transitions.push(transition(
                "tr-entering-context",
                TransitionKind::EnteringContext,
                format!("Entering {primary} context"),
                previous_state.clone(),
                format!("Context: {primary}"),
                vec![format!("Primary work context is {primary}")],
                vec![TransitionEvidence {
                    label: "Work Context".into(),
                    source_projection: "work_context".into(),
                    source_ref: work_context.workspace_id.clone(),
                    why: "Work Context is semantic work truth; Transition explains entry only"
                        .into(),
                }],
                related_context.clone(),
                related_milestones.clone(),
                open_decisions.clone(),
                interrupted.clone(),
                TransitionConfidence::Medium,
                "Explains entering a semantic work context — never switches focus.".into(),
                "Context entry is observational; Navigation/Session remain separate.".into(),
            ));

            if work_context.contexts.len() > 1 || navigation.suggested_count > 0 {
                let labels: Vec<_> = work_context
                    .contexts
                    .iter()
                    .take(3)
                    .map(|c| c.context_type.as_str().to_string())
                    .collect();
                transitions.push(transition(
                    "tr-switching-focus",
                    TransitionKind::SwitchingFocus,
                    "Switching focus / context".into(),
                    previous_state.clone(),
                    current_state.clone(),
                    vec![
                        format!("Contexts: {}", labels.join(", ")),
                        format!("Navigation suggested: {}", navigation.suggested_count),
                    ],
                    vec![
                        TransitionEvidence {
                            label: "Work Context multiplicity".into(),
                            source_projection: "work_context".into(),
                            source_ref: format!("contexts={}", work_context.contexts.len()),
                            why: "Multiple contexts explain switching without performing it".into(),
                        },
                        TransitionEvidence {
                            label: "Navigation".into(),
                            source_projection: "navigation".into(),
                            source_ref: format!("suggested={}", navigation.suggested_count),
                            why: "Navigation suggests inspection paths; Transition does not route"
                                .into(),
                        },
                    ],
                    related_context.clone(),
                    related_milestones.clone(),
                    open_decisions.clone(),
                    interrupted.clone(),
                    TransitionConfidence::Medium,
                    "Explains a focus/context switch posture — never changes focus.".into(),
                    "Switching is explained from Work Context + Navigation evidence only.".into(),
                ));
            }
        }

        // Evolution / activity change narrative.
        if !intelligence.evolution.summary.is_empty()
            || intelligence.activity_graph.activity_count > 0
        {
            transitions.push(transition(
                "tr-changed",
                TransitionKind::LeavingContext,
                "Changed since last understood state".into(),
                previous_state.clone(),
                current_state.clone(),
                vec![
                    if intelligence.evolution.summary.is_empty() {
                        "Activity Graph signals present".into()
                    } else {
                        intelligence.evolution.summary.clone()
                    },
                ],
                vec![
                    TransitionEvidence {
                        label: "Evolution".into(),
                        source_projection: "evolution".into(),
                        source_ref: intelligence.evolution.workspace_id.clone(),
                        why: "Evolution narrates change; Transition does not mutate history".into(),
                    },
                    TransitionEvidence {
                        label: "Activity Graph".into(),
                        source_projection: "activity_graph".into(),
                        source_ref: format!(
                            "count={}",
                            intelligence.activity_graph.activity_count
                        ),
                        why: "Activity Graph remains historical SoT".into(),
                    },
                ],
                related_context.clone(),
                related_milestones.clone(),
                open_decisions.clone(),
                interrupted.clone(),
                TransitionConfidence::Medium,
                "Explains what changed between understood states — never applies changes.".into(),
                "Change narrative from Evolution + Activity; no restoration or automation."
                    .into(),
            ));
        }

        // Starting / completing from milestones + purpose / task graph.
        if let Some(current) = milestones.current_milestone() {
            transitions.push(transition(
                "tr-milestone-progress",
                TransitionKind::StartingNewWorkState,
                format!("Progress toward \"{}\"", current.title),
                previous_state.clone(),
                format!(
                    "Milestone: {} ({}%)",
                    current.title, current.progress_percent
                ),
                vec![format!(
                    "Milestone readiness: {}",
                    current.readiness.as_str()
                )],
                vec![TransitionEvidence {
                    label: "Current milestone".into(),
                    source_projection: "milestones".into(),
                    source_ref: current.id.clone(),
                    why: "Milestones coordinate outcomes; Transition explains movement only"
                        .into(),
                }],
                related_context.clone(),
                related_milestones.clone(),
                open_decisions.clone(),
                interrupted.clone(),
                TransitionConfidence::Medium,
                "Explains movement toward a milestone — never completes or schedules it.".into(),
                "Milestone progress is observational coordination, not execution.".into(),
            ));
        }

        if milestones.completed_count > 0 {
            transitions.push(transition(
                "tr-completing",
                TransitionKind::CompletingWorkState,
                "Completing a work state (observed)".into(),
                previous_state.clone(),
                current_state.clone(),
                vec![format!(
                    "Completed milestones observed: {}",
                    milestones.completed_count
                )],
                vec![TransitionEvidence {
                    label: "Completed milestones".into(),
                    source_projection: "milestones".into(),
                    source_ref: format!("completed={}", milestones.completed_count),
                    why: "Completion is observed from Milestones — never auto-completed here"
                        .into(),
                }],
                related_context.clone(),
                related_milestones.clone(),
                open_decisions.clone(),
                interrupted.clone(),
                TransitionConfidence::Low,
                "Explains observed completion markers — never marks work complete.".into(),
                "Completion observation is not an action or grant.".into(),
            ));
        }

        // Always at least one current-state transition for empty-ish workspaces.
        if transitions.is_empty() {
            transitions.push(transition(
                "tr-current",
                TransitionKind::StartingNewWorkState,
                "Current work state".into(),
                previous_state.clone(),
                current_state.clone(),
                vec!["No stronger transition signals yet".into()],
                vec![TransitionEvidence {
                    label: "Session".into(),
                    source_projection: "session".into(),
                    source_ref: session.generated_at.clone(),
                    why: "Session is runtime SoT; Transition explains current state only".into(),
                }],
                related_context.clone(),
                related_milestones.clone(),
                open_decisions.clone(),
                interrupted.clone(),
                TransitionConfidence::Low,
                "Baseline current-state explanation — never starts work.".into(),
                "Present when other transition signals are sparse.".into(),
            ));
        }

        // Prefer returning as current when present.
        let current_id = transitions
            .iter()
            .find(|t| t.id == "tr-returning")
            .or_else(|| {
                transitions
                    .iter()
                    .find(|t| t.kind == TransitionKind::SwitchingFocus)
            })
            .or_else(|| transitions.first())
            .map(|t| t.id.clone());

        let relationships = build_relationships(&transitions, &current_id);
        let returning_count = transitions
            .iter()
            .filter(|t| {
                matches!(
                    t.kind,
                    TransitionKind::ReturningToWork | TransitionKind::ContinuingInterruptedWork
                )
            })
            .count();
        let switching_count = transitions
            .iter()
            .filter(|t| t.kind == TransitionKind::SwitchingFocus)
            .count();
        let interrupted_count = transitions
            .iter()
            .filter(|t| !t.interrupted_work.is_empty())
            .count()
            .max(intelligence.continuity.interrupted_count);

        let current_title = current_id
            .as_ref()
            .and_then(|id| transitions.iter().find(|t| &t.id == id))
            .map(|t| t.title.clone());

        let transition_summary = TransitionSummary {
            headline: format!("Recent transitions for \"{label}\""),
            left_off_line: previous_state.clone(),
            current_transition_line: current_title
                .clone()
                .unwrap_or_else(|| "No current transition selected".into()),
            changed_line: if intelligence.evolution.summary.is_empty() {
                "No evolution change narrative yet.".into()
            } else {
                intelligence.evolution.summary.clone()
            },
            returned_line: if returning_count > 0 {
                format!("{returning_count} returning / interrupted transition(s)")
            } else {
                "No returning-work transition identified.".into()
            },
            context_switch_line: if switching_count > 0 {
                format!("{switching_count} context-switch transition(s)")
            } else {
                "No context-switch transition identified.".into()
            },
            interrupted_line: if intelligence.continuity.interrupted_count > 0 {
                format!(
                    "{} interrupted work signal(s)",
                    intelligence.continuity.interrupted_count
                )
            } else {
                "No interrupted work signals.".into()
            },
            narrative: format!(
                "Transitions for \"{label}\" explain movement between work states without \
                 restoring, launching, or executing. {} transition(s). Never certainty.",
                transitions.len()
            ),
        };

        let summary =
            build_transition_summary(&label, transitions.len(), current_title.as_deref());
        let explanation = format!(
            "Transition Engine for \"{label}\" compares Session, Experience, Continuity, \
             Work Context, Navigation, Milestones, Working Style, Evolution, and Activity. \
             Explains movement only — never restores, automates, or executes. \
             authority_effect=none. working_style_obs={}.",
            working_style.observation_count
        );
        let mut evidence = vec![
            format!("session:{}", session.generated_at),
            format!(
                "continuity:interrupted={}",
                intelligence.continuity.interrupted_count
            ),
            format!("work_context:count={}", work_context.contexts.len()),
            format!("navigation:suggested={}", navigation.suggested_count),
            format!(
                "milestones:current={:?}",
                milestones.current_milestone().map(|m| &m.title)
            ),
            format!("working_style:obs={}", working_style.observation_count),
            format!("activity:count={}", intelligence.activity_graph.activity_count),
            format!("evolution:{}", intelligence.evolution.summary),
        ];
        evidence.sort();
        evidence.dedup();

        let state = WorkspaceTransitionState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: transition_now_rfc3339(),
            label,
            transition_summary,
            transition_count: transitions.len(),
            returning_count,
            switching_count,
            interrupted_count,
            current_transition_id: current_id,
            transitions,
            relationships,
            session_generated_at: session.generated_at.clone(),
            experience_generated_at: experience.generated_at.clone(),
            work_context_generated_at: work_context.generated_at.clone(),
            navigation_generated_at: navigation.generated_at.clone(),
            milestones_generated_at: milestones.generated_at.clone(),
            working_style_generated_at: working_style.generated_at.clone(),
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceTransitionState::AUTHORITY_EFFECT_NONE.into(),
        };

        audit_event(
            db,
            actor,
            "workspace.transition.generated",
            &state,
            json!({
                "transition_count": state.transition_count,
                "returning_count": state.returning_count,
                "switching_count": state.switching_count,
            }),
        )?;
        audit_event(
            db,
            actor,
            "workspace.transition.updated",
            &state,
            json!({ "transition_count": state.transition_count }),
        )?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceTransitionState,
        limit: usize,
    ) -> WorkspaceTransitionSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceTransitionState,
        right: &WorkspaceTransitionState,
    ) -> WorkspaceTransitionComparison {
        WorkspaceTransitionComparison::compare(left, right)
    }

    pub(crate) fn validate(state: &WorkspaceTransitionState) -> WorkspaceTransitionValidation {
        WorkspaceTransitionValidation::validate(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceTransitionState,
    ) -> Result<WorkspaceTransitionValidation> {
        let report = Self::validate(state);
        audit_event(
            db,
            actor,
            "workspace.transition.validated",
            state,
            json!({
                "valid": report.valid,
                "message_count": report.messages.len(),
            }),
        )?;
        Ok(report)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceTransitionError::CannotExecute,
        ))
    }
}

fn describe_previous(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> String {
    if let Some(focus) = &intelligence.continuity.current_focus {
        return format!("Left off: {}", focus.title);
    }
    if !intelligence.continuity.summary.is_empty() {
        return intelligence.continuity.summary.clone();
    }
    if !session.summary.is_empty() {
        return format!("Prior session: {}", session.summary);
    }
    "No prior work state identified.".into()
}

fn describe_current(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
    work_context: &WorkspaceWorkContextState,
) -> String {
    let project = intelligence
        .current_project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or("no project");
    let task = intelligence
        .current_task
        .as_ref()
        .map(|t| t.title.as_str())
        .unwrap_or("no task");
    let ctx = work_context
        .contexts
        .first()
        .map(|c| c.context_type.as_str())
        .unwrap_or("unclassified");
    format!(
        "Now: {project} / {task} · context {ctx} · session {}",
        session.label
    )
}

fn collect_contexts(work_context: &WorkspaceWorkContextState) -> Vec<TransitionAssociation> {
    work_context
        .contexts
        .iter()
        .take(5)
        .map(|c| TransitionAssociation {
            id: c.id.clone(),
            label: c.context_type.as_str().to_string(),
            kind: "work_context".into(),
            source_projection: "work_context".into(),
            source_ref: c.id.clone(),
            why: "Related semantic work context for this transition".into(),
        })
        .collect()
}

fn collect_milestones(milestones: &WorkspaceMilestoneState) -> Vec<TransitionAssociation> {
    milestones
        .milestones
        .iter()
        .take(5)
        .map(|m| TransitionAssociation {
            id: m.id.clone(),
            label: m.title.clone(),
            kind: m.status.as_str().into(),
            source_projection: "milestones".into(),
            source_ref: m.id.clone(),
            why: "Related milestone coordination for this transition".into(),
        })
        .collect()
}

fn collect_decisions(intelligence: &WorkspaceIntelligenceState) -> Vec<TransitionAssociation> {
    intelligence
        .decision_queue
        .items
        .iter()
        .take(5)
        .map(|d| TransitionAssociation {
            id: d.id.to_string(),
            label: d.title.clone(),
            kind: "decision".into(),
            source_projection: "decision_queue".into(),
            source_ref: d.id.to_string(),
            why: "Open decision that may relate to this transition".into(),
        })
        .collect()
}

fn collect_interrupted(intelligence: &WorkspaceIntelligenceState) -> Vec<TransitionAssociation> {
    let mut out = Vec::new();
    if let Some(focus) = &intelligence.continuity.current_focus {
        if intelligence.continuity.interrupted_count > 0 {
            out.push(TransitionAssociation {
                id: focus.id.to_string(),
                label: focus.title.clone(),
                kind: "interrupted".into(),
                source_projection: "continuity".into(),
                source_ref: focus.id.to_string(),
                why: "Interrupted / resumable work from Continuity".into(),
            });
        }
    }
    if let Some(next) = &intelligence.continuity.suggested_next_step {
        out.push(TransitionAssociation {
            id: next.id.to_string(),
            label: next.title.clone(),
            kind: "suggested_next".into(),
            source_projection: "continuity".into(),
            source_ref: next.id.to_string(),
            why: "Suggested next step from Continuity (informational)".into(),
        });
    }
    out
}

fn build_relationships(
    transitions: &[WorkspaceTransition],
    current_id: &Option<String>,
) -> Vec<TransitionRelationship> {
    let mut out = Vec::new();
    if let Some(current) = current_id {
        out.push(TransitionRelationship {
            id: "rel-current".into(),
            from_transition_id: current.clone(),
            to_ref: "current_state".into(),
            kind: TransitionRelationKind::Current,
            why: "Marks the primary explained transition for the current snapshot".into(),
            authority_effect: WorkspaceTransition::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    for t in transitions {
        out.push(TransitionRelationship {
            id: format!("rel-prev-{}", t.id),
            from_transition_id: t.id.clone(),
            to_ref: t.previous_state.clone(),
            kind: TransitionRelationKind::Previous,
            why: "Links transition to its previous-state description".into(),
            authority_effect: WorkspaceTransition::AUTHORITY_EFFECT_NONE.into(),
        });
        if matches!(
            t.kind,
            TransitionKind::ReturningToWork | TransitionKind::ContinuingInterruptedWork
        ) {
            out.push(TransitionRelationship {
                id: format!("rel-returned-{}", t.id),
                from_transition_id: t.id.clone(),
                to_ref: t.previous_state.clone(),
                kind: TransitionRelationKind::ReturnedFrom,
                why: "Returning / interrupted transition references prior work".into(),
                authority_effect: WorkspaceTransition::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if !t.interrupted_work.is_empty() {
            out.push(TransitionRelationship {
                id: format!("rel-interrupted-{}", t.id),
                from_transition_id: t.id.clone(),
                to_ref: t
                    .interrupted_work
                    .first()
                    .map(|i| i.label.clone())
                    .unwrap_or_else(|| "interrupted".into()),
                kind: TransitionRelationKind::InterruptedBy,
                why: "Transition associated with interrupted work evidence".into(),
                authority_effect: WorkspaceTransition::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if !t.open_decisions.is_empty() {
            out.push(TransitionRelationship {
                id: format!("rel-blocked-{}", t.id),
                from_transition_id: t.id.clone(),
                to_ref: t
                    .open_decisions
                    .first()
                    .map(|d| d.label.clone())
                    .unwrap_or_else(|| "decision".into()),
                kind: TransitionRelationKind::BlockedBy,
                why: "Open decisions may informally block completing this transition narrative"
                    .into(),
                authority_effect: WorkspaceTransition::AUTHORITY_EFFECT_NONE.into(),
            });
        }
    }
    out
}

fn transition(
    id: &str,
    kind: TransitionKind,
    title: String,
    previous_state: String,
    current_state: String,
    changed_elements: Vec<String>,
    evidence: Vec<TransitionEvidence>,
    related_context: Vec<TransitionAssociation>,
    related_milestones: Vec<TransitionAssociation>,
    open_decisions: Vec<TransitionAssociation>,
    interrupted_work: Vec<TransitionAssociation>,
    confidence: TransitionConfidence,
    explanation: String,
    why: String,
) -> WorkspaceTransition {
    WorkspaceTransition {
        id: id.into(),
        kind,
        title,
        previous_state,
        current_state,
        changed_elements,
        evidence,
        related_context,
        related_milestones,
        open_decisions,
        interrupted_work,
        confidence,
        explanation,
        why,
        authority_effect: WorkspaceTransition::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn audit_event(
    db: &Arc<Mutex<Database>>,
    actor: &ActorContext,
    event: &str,
    state: &WorkspaceTransitionState,
    details: serde_json::Value,
) -> Result<()> {
    let mut payload = details.as_object().cloned().unwrap_or_default();
    payload.insert("workspace_id".into(), json!(state.workspace_id));
    payload.insert("authority_effect".into(), json!("none"));
    AuditService::record_ai_planning_event(
        db,
        actor,
        &IntentContext::user_request(),
        event,
        true,
        serde_json::Value::Object(payload).to_string(),
    )
}
