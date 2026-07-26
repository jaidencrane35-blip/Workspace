//! Workspace Working Style Model (Phase 6).
//!
//! Aggregates existing projections into explainable operating-pattern observations.
//! Separates Observed Behaviour from Explicit Preference.
//! Owns nothing. Never profiles, predicts, learns autonomously, or executes.

use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_working_style_summary, validate_working_style_workspace_id, working_style_now_rfc3339,
    ActorContext, IntentContext, PatternKind, WorkingStyleConfidence, WorkingStyleEvidence,
    WorkingStyleKind, WorkingStyleObservation, WorkingStyleOrigin, WorkingStyleSummary,
    WorkspaceExperienceState, WorkspaceIntelligenceState, WorkspaceMilestoneState,
    WorkspaceNavigationState, WorkspaceSessionState, WorkspaceWorkContextState,
    WorkspaceWorkingStyleComparison, WorkspaceWorkingStyleState, WorkspaceWorkingStyleSummary,
    WorkspaceWorkingStyleValidation,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceMilestoneService, WorkspaceNavigationService,
    WorkspaceSessionService, WorkspaceWorkContextService,
};

pub(crate) struct WorkspaceWorkingStyleService;

impl WorkspaceWorkingStyleService {
    /// Standalone generate — loads Intelligence stack, then projects working style.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceWorkingStyleState> {
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
        Self::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
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
    ) -> Result<WorkspaceWorkingStyleState> {
        let _ = validate_working_style_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = session.label.clone();
        let mut observations = Vec::new();

        push_rhythm(intelligence, &mut observations);
        push_organization(intelligence, &mut observations);
        push_workflow(intelligence, milestones, &mut observations);
        push_context_switching(intelligence, work_context, navigation, &mut observations);
        push_observed_usage(intelligence, &mut observations);
        push_explicit_preferences(intelligence, &mut observations);

        let observed_count = observations
            .iter()
            .filter(|o| o.origin == WorkingStyleOrigin::ObservedBehaviour)
            .count();
        let preference_count = observations
            .iter()
            .filter(|o| o.origin == WorkingStyleOrigin::ExplicitPreference)
            .count();
        let rhythm_count = count_kind(&observations, WorkingStyleKind::Rhythm);
        let organization_count = count_kind(&observations, WorkingStyleKind::Organization);
        let workflow_count = count_kind(&observations, WorkingStyleKind::Workflow);
        let context_switching_count =
            count_kind(&observations, WorkingStyleKind::ContextSwitching);

        let style_summary = build_lines(
            &label,
            &observations,
            observed_count,
            preference_count,
            intelligence,
        );
        let summary =
            build_working_style_summary(&label, observations.len(), preference_count);
        let explanation = format!(
            "Working Style for \"{label}\" aggregates Pattern, Activity, Composition, \
             Environment, Continuity, Task Graph, Purpose, Work Context, Navigation, \
             Milestones, and explicit Preferences. Observed behaviour is never treated as \
             intent or permission. authority_effect=none."
        );
        let mut evidence = vec![
            format!("activity:count={}", intelligence.activity_graph.activity_count),
            format!("pattern:count={}", intelligence.pattern.pattern_count),
            format!("composition:{}", intelligence.composition.summary),
            format!(
                "work_context:primary={}",
                work_context
                    .contexts
                    .first()
                    .map(|c| c.context_type.as_str())
                    .unwrap_or("none")
            ),
            format!(
                "milestones:current={:?}",
                milestones.current_milestone().map(|m| &m.title)
            ),
            format!("preferences:highlights={}", intelligence.preference_highlights.len()),
            format!("observed_count={observed_count}"),
            format!("preference_count={preference_count}"),
        ];
        evidence.sort();
        evidence.dedup();

        let state = WorkspaceWorkingStyleState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: working_style_now_rfc3339(),
            label,
            style_summary,
            observation_count: observations.len(),
            observed_count,
            preference_count,
            rhythm_count,
            organization_count,
            workflow_count,
            context_switching_count,
            observations,
            session_generated_at: session.generated_at.clone(),
            experience_generated_at: experience.generated_at.clone(),
            work_context_generated_at: work_context.generated_at.clone(),
            navigation_generated_at: navigation.generated_at.clone(),
            milestones_generated_at: milestones.generated_at.clone(),
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceWorkingStyleState::AUTHORITY_EFFECT_NONE.into(),
        };

        audit_event(
            db,
            actor,
            "workspace.working_style.generated",
            &state,
            json!({
                "observation_count": state.observation_count,
                "observed_count": state.observed_count,
                "preference_count": state.preference_count,
            }),
        )?;
        audit_event(
            db,
            actor,
            "workspace.working_style.updated",
            &state,
            json!({
                "observation_count": state.observation_count,
            }),
        )?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceWorkingStyleState,
        limit: usize,
    ) -> WorkspaceWorkingStyleSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceWorkingStyleState,
        right: &WorkspaceWorkingStyleState,
    ) -> WorkspaceWorkingStyleComparison {
        WorkspaceWorkingStyleComparison::compare(left, right)
    }

    pub(crate) fn validate(state: &WorkspaceWorkingStyleState) -> WorkspaceWorkingStyleValidation {
        WorkspaceWorkingStyleValidation::validate(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceWorkingStyleState,
    ) -> Result<WorkspaceWorkingStyleValidation> {
        let report = Self::validate(state);
        audit_event(
            db,
            actor,
            "workspace.working_style.validated",
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
            workspace_domain::WorkspaceWorkingStyleError::CannotExecute,
        ))
    }
}

fn count_kind(observations: &[WorkingStyleObservation], kind: WorkingStyleKind) -> usize {
    observations.iter().filter(|o| o.kind == kind).count()
}

fn push_rhythm(
    intelligence: &WorkspaceIntelligenceState,
    observations: &mut Vec<WorkingStyleObservation>,
) {
    let apps: Vec<_> = intelligence
        .current_applications
        .iter()
        .take(4)
        .map(|a| a.name.clone())
        .collect();
    if !apps.is_empty() {
        let title = "Frequent application combination".into();
        let joined = apps.join(" · ");
        observations.push(obs(
            "ws-rhythm-apps",
            WorkingStyleKind::Rhythm,
            WorkingStyleOrigin::ObservedBehaviour,
            title,
            format!("Applications commonly present: {joined}"),
            if apps.len() >= 2 {
                WorkingStyleConfidence::Medium
            } else {
                WorkingStyleConfidence::Low
            },
            vec![WorkingStyleEvidence {
                label: "Current applications".into(),
                source_projection: "environment".into(),
                source_ref: joined.clone(),
                why: "Application presence is an observable rhythm signal, not a preference or grant"
                    .into(),
            }],
            format!("Applications: {joined}"),
            "Workspace rhythm from currently associated applications — observational only.".into(),
            "Derived from Environment / Intelligence application inventory without scoring the user."
                .into(),
        ));
    }

    let activity_count = intelligence.activity_graph.activity_count;
    if activity_count > 0 {
        // Keep the human-facing line stable across regenerates: projection audits would
        // otherwise displace durable work signals from the recent timeline window.
        observations.push(obs(
            "ws-rhythm-activity",
            WorkingStyleKind::Rhythm,
            WorkingStyleOrigin::ObservedBehaviour,
            "Common activity sequence".into(),
            "Activity Graph has historical signals (observational only)".into(),
            if activity_count >= 3 {
                WorkingStyleConfidence::Medium
            } else {
                WorkingStyleConfidence::Low
            },
            vec![WorkingStyleEvidence {
                label: "Activity Graph timeline".into(),
                source_projection: "activity_graph".into(),
                source_ref: format!("count={activity_count}"),
                why: "Activity Graph remains historical SoT; Working Style only projects sequences"
                    .into(),
            }],
            "Activity timeline".into(),
            "Typical workflow sequences projected from Activity Graph — not prediction.".into(),
            "Activity Graph is the history source; this observation does not store new history."
                .into(),
        ));
    }

    if !intelligence.continuity.session_anchor.is_empty()
        || !intelligence.continuity.summary.is_empty()
    {
        observations.push(obs(
            "ws-rhythm-focus",
            WorkingStyleKind::Rhythm,
            WorkingStyleOrigin::ObservedBehaviour,
            "Typical focus continuity".into(),
            intelligence.continuity.summary.clone(),
            WorkingStyleConfidence::Medium,
            vec![WorkingStyleEvidence {
                label: "Continuity narrative".into(),
                source_projection: "continuity".into(),
                source_ref: if intelligence.continuity.session_anchor.is_empty() {
                    "continuity".into()
                } else {
                    intelligence.continuity.session_anchor.clone()
                },
                why: "Focus duration / resume patterns come from Continuity, not surveillance"
                    .into(),
            }],
            "Continuity / focus".into(),
            "Focus rhythm from Continuity resume narrative — observational.".into(),
            "Continuity owns resume narrative; Working Style references it as evidence only."
                .into(),
        ));
    }
}

fn push_organization(
    intelligence: &WorkspaceIntelligenceState,
    observations: &mut Vec<WorkingStyleObservation>,
) {
    if !intelligence.composition.summary.is_empty() {
        observations.push(obs(
            "ws-org-composition",
            WorkingStyleKind::Organization,
            WorkingStyleOrigin::ObservedBehaviour,
            "Recurring workspace arrangement".into(),
            intelligence.composition.summary.clone(),
            WorkingStyleConfidence::Medium,
            vec![WorkingStyleEvidence {
                label: "Composition summary".into(),
                source_projection: "composition".into(),
                source_ref: intelligence.composition.workspace_id.clone(),
                why: "Composition explains how parts belong together — not a layout mutation"
                    .into(),
            }],
            "Composition".into(),
            "Organization style from Composition groupings.".into(),
            "Composition remains meaning-of-structure SoT; this is a style projection only."
                .into(),
        ));
    }

    if !intelligence.environment.summary.is_empty()
        || !intelligence.environment.top_applications.is_empty()
    {
        let env_line = if intelligence.environment.summary.is_empty() {
            format!(
                "{} application(s) in environment inventory",
                intelligence.environment.top_applications.len()
            )
        } else {
            intelligence.environment.summary.clone()
        };
        observations.push(obs(
            "ws-org-environment",
            WorkingStyleKind::Organization,
            WorkingStyleOrigin::ObservedBehaviour,
            "Preferred layout presence (observed)".into(),
            env_line.clone(),
            WorkingStyleConfidence::Low,
            vec![WorkingStyleEvidence {
                label: "Environment summary".into(),
                source_projection: "environment".into(),
                source_ref: intelligence.environment.workspace_id.clone(),
                why: "Environment describes what is arranged — Working Style does not rearrange"
                    .into(),
            }],
            "Environment / layouts".into(),
            "Observed arrangement from Environment — labelled observed, not preferred.".into(),
            "Observed layout presence is not an explicit preference or authorization.".into(),
        ));
    }
}

fn push_workflow(
    intelligence: &WorkspaceIntelligenceState,
    milestones: &WorkspaceMilestoneState,
    observations: &mut Vec<WorkingStyleObservation>,
) {
    for pattern in intelligence
        .pattern
        .top_patterns
        .iter()
        .filter(|p| p.kind == PatternKind::WorkflowPattern || p.kind == PatternKind::TaskPattern)
        .take(3)
    {
        observations.push(obs(
            &format!("ws-workflow-{}", pattern.id),
            WorkingStyleKind::Workflow,
            WorkingStyleOrigin::ObservedBehaviour,
            pattern.title.clone(),
            pattern.observation.clone(),
            match pattern.confidence {
                workspace_domain::PatternConfidence::High => WorkingStyleConfidence::High,
                workspace_domain::PatternConfidence::Medium => WorkingStyleConfidence::Medium,
                workspace_domain::PatternConfidence::Low => WorkingStyleConfidence::Low,
            },
            vec![WorkingStyleEvidence {
                label: "Pattern Model".into(),
                source_projection: "pattern".into(),
                source_ref: pattern.id.clone(),
                why: "Pattern Model remains recurring-structure SoT; style only projects workflow"
                    .into(),
            }],
            format!("Pattern: {}", pattern.kind.as_str()),
            pattern.impact.clone(),
            "Workflow style projected from Pattern Model without predicting next actions.".into(),
        ));
    }

    if intelligence.task_graph.node_count > 0 {
        observations.push(obs(
            "ws-workflow-tasks",
            WorkingStyleKind::Workflow,
            WorkingStyleOrigin::ObservedBehaviour,
            "Common task transitions".into(),
            format!(
                "Task Graph has {} task(s), {} relationship(s)",
                intelligence.task_graph.node_count, intelligence.task_graph.relationship_count
            ),
            if intelligence.task_graph.relationship_count > 0 {
                WorkingStyleConfidence::Medium
            } else {
                WorkingStyleConfidence::Low
            },
            vec![WorkingStyleEvidence {
                label: "Task Graph summary".into(),
                source_projection: "task_graph".into(),
                source_ref: format!("tasks={}", intelligence.task_graph.node_count),
                why: "Task Graph remains work-unit SoT; style does not create or mutate tasks"
                    .into(),
            }],
            "Task Graph".into(),
            "Task transition style from Task Graph structure.".into(),
            "Working Style never creates actions or tasks from workflow observations.".into(),
        ));
    }

    if let Some(current) = milestones.current_milestone() {
        observations.push(obs(
            "ws-workflow-milestone",
            WorkingStyleKind::Workflow,
            WorkingStyleOrigin::ObservedBehaviour,
            "Progress rhythm toward outcomes".into(),
            format!(
                "Current milestone \"{}\" at {}% ({})",
                current.title, current.progress_percent, current.readiness.as_str()
            ),
            WorkingStyleConfidence::Medium,
            vec![WorkingStyleEvidence {
                label: "Current milestone".into(),
                source_projection: "milestones".into(),
                source_ref: current.id.clone(),
                why: "Milestones coordinate outcomes; style observes progress rhythm only".into(),
            }],
            format!("Milestone: {}", current.title),
            "Outcome progress style from Milestones — not a plan.".into(),
            "Milestones remain coordination-only; Working Style does not schedule completion."
                .into(),
        ));
    }
}

fn push_context_switching(
    intelligence: &WorkspaceIntelligenceState,
    work_context: &WorkspaceWorkContextState,
    navigation: &WorkspaceNavigationState,
    observations: &mut Vec<WorkingStyleObservation>,
) {
    if !work_context.contexts.is_empty() {
        let labels: Vec<_> = work_context
            .contexts
            .iter()
            .take(4)
            .map(|c| c.context_type.as_str().to_string())
            .collect();
        observations.push(obs(
            "ws-switch-contexts",
            WorkingStyleKind::ContextSwitching,
            WorkingStyleOrigin::ObservedBehaviour,
            "Frequent work contexts".into(),
            format!("Contexts present: {}", labels.join(", ")),
            if work_context.contexts.len() >= 2 {
                WorkingStyleConfidence::Medium
            } else {
                WorkingStyleConfidence::Low
            },
            vec![WorkingStyleEvidence {
                label: "Work Context".into(),
                source_projection: "work_context".into(),
                source_ref: work_context.workspace_id.clone(),
                why: "Work Context classifies kind-of-work; style observes switching without routing"
                    .into(),
            }],
            labels.join(", "),
            "Frequent contexts from Work Context Engine.".into(),
            "Context switching observations never autonomously change focus.".into(),
        ));
    }

    if intelligence.continuity.interrupted_count > 0
        || intelligence.continuity.summary.to_lowercase().contains("interrupt")
    {
        observations.push(obs(
            "ws-switch-interrupted",
            WorkingStyleKind::ContextSwitching,
            WorkingStyleOrigin::ObservedBehaviour,
            "Interrupted context pattern".into(),
            intelligence.continuity.summary.clone(),
            WorkingStyleConfidence::Medium,
            vec![WorkingStyleEvidence {
                label: "Continuity interrupted work".into(),
                source_projection: "continuity".into(),
                source_ref: "interrupted".into(),
                why: "Interrupted contexts come from Continuity — not behavioural monitoring"
                    .into(),
            }],
            "Interrupted work".into(),
            "Returning / interrupted patterns from Continuity.".into(),
            "Does not resume or restore — Continuity remains narrative SoT.".into(),
        ));
    }

    if navigation.blocked_count > 0 || navigation.suggested_count > 0 {
        observations.push(obs(
            "ws-switch-navigation",
            WorkingStyleKind::ContextSwitching,
            WorkingStyleOrigin::ObservedBehaviour,
            "Inspection / return path style".into(),
            navigation.navigation_summary.narrative.clone(),
            WorkingStyleConfidence::Low,
            vec![WorkingStyleEvidence {
                label: "Navigation narrative".into(),
                source_projection: "navigation".into(),
                source_ref: format!(
                    "blocked={},suggested={}",
                    navigation.blocked_count, navigation.suggested_count
                ),
                why: "Navigation owns inspection paths; style only notes switching posture".into(),
            }],
            navigation.navigation_summary.breadcrumb_line.clone(),
            "Context switching posture from Navigation paths.".into(),
            "Working Style never routes or executes Navigation paths.".into(),
        ));
    }
}

fn push_observed_usage(
    intelligence: &WorkspaceIntelligenceState,
    observations: &mut Vec<WorkingStyleObservation>,
) {
    for pattern in intelligence
        .pattern
        .top_patterns
        .iter()
        .filter(|p| {
            p.kind == PatternKind::ApplicationPattern || p.kind == PatternKind::EnvironmentPattern
        })
        .take(2)
    {
        observations.push(obs(
            &format!("ws-usage-{}", pattern.id),
            WorkingStyleKind::ObservedUsage,
            WorkingStyleOrigin::ObservedBehaviour,
            format!("Observed: {}", pattern.title),
            pattern.observation.clone(),
            match pattern.confidence {
                workspace_domain::PatternConfidence::High => WorkingStyleConfidence::High,
                workspace_domain::PatternConfidence::Medium => WorkingStyleConfidence::Medium,
                workspace_domain::PatternConfidence::Low => WorkingStyleConfidence::Low,
            },
            vec![WorkingStyleEvidence {
                label: "Pattern (observed usage)".into(),
                source_projection: "pattern".into(),
                source_ref: pattern.id.clone(),
                why: "Observed usage is labelled separately from explicit preferences".into(),
            }],
            format!("Observed pattern: {}", pattern.kind.as_str()),
            "Observed usage pattern — not an explicit preference.".into(),
            "Never treat observed usage as permission or user intent.".into(),
        ));
    }

    if !intelligence.recommendation_engine.summary.is_empty() {
        observations.push(obs(
            "ws-usage-recommendations",
            WorkingStyleKind::ObservedUsage,
            WorkingStyleOrigin::ObservedBehaviour,
            "Observed recommendation context".into(),
            intelligence.recommendation_engine.summary.clone(),
            WorkingStyleConfidence::Low,
            vec![WorkingStyleEvidence {
                label: "Recommendation Engine summary".into(),
                source_projection: "recommendation_engine".into(),
                source_ref: intelligence.recommendation_engine.workspace_id.clone(),
                why: "Recommendations may later reference style as evidence only".into(),
            }],
            "Recommendation Engine".into(),
            "Observed suggestion context — style does not create or accept recommendations."
                .into(),
            "Working Style cannot create actions; Recommendation Engine remains separate.".into(),
        ));
    }
}

fn push_explicit_preferences(
    intelligence: &WorkspaceIntelligenceState,
    observations: &mut Vec<WorkingStyleObservation>,
) {
    for (idx, pref) in intelligence.preference_highlights.iter().take(5).enumerate() {
        observations.push(obs(
            &format!("ws-pref-{}", pref.id),
            WorkingStyleKind::InteractionPreference,
            WorkingStyleOrigin::ExplicitPreference,
            format!("Explicit preference: {}", pref.label),
            pref.summary.clone(),
            WorkingStyleConfidence::High,
            vec![WorkingStyleEvidence {
                label: "User preference highlight".into(),
                source_projection: "preference".into(),
                source_ref: pref.id.clone(),
                why: "Explicit preferences remain AiPersonalizationService DurableStore SoT"
                    .into(),
            }],
            pref.label.clone(),
            format!(
                "Explicit preference (source={}). Never inferred from behaviour.",
                pref.source
            ),
            "Preferences are explicit and inspectable — never treated as observation of behaviour."
                .into(),
        ));
        let _ = idx;
    }
}

fn build_lines(
    label: &str,
    observations: &[WorkingStyleObservation],
    observed_count: usize,
    preference_count: usize,
    intelligence: &WorkspaceIntelligenceState,
) -> WorkingStyleSummary {
    let rhythm = first_line(observations, WorkingStyleKind::Rhythm)
        .unwrap_or_else(|| "No rhythm observations yet.".into());
    let organization = first_line(observations, WorkingStyleKind::Organization)
        .unwrap_or_else(|| "No organization observations yet.".into());
    let workflow = first_line(observations, WorkingStyleKind::Workflow)
        .unwrap_or_else(|| "No workflow observations yet.".into());
    let switching = first_line(observations, WorkingStyleKind::ContextSwitching)
        .unwrap_or_else(|| "No context-switching observations yet.".into());
    let preference = first_line(observations, WorkingStyleKind::InteractionPreference)
        .unwrap_or_else(|| {
            if intelligence.preference_highlights.is_empty() {
                "No explicit preferences recorded.".into()
            } else {
                "Explicit preferences present.".into()
            }
        });
    let observed_vs_preferred = if preference_count == 0 && observed_count > 0 {
        format!(
            "{observed_count} observed behaviour signal(s); no explicit preferences — \
             observations are not preferences."
        )
    } else if preference_count > 0 && observed_count == 0 {
        format!(
            "{preference_count} explicit preference(s); no observed behaviour signals yet."
        )
    } else if preference_count > 0 && observed_count > 0 {
        format!(
            "{observed_count} observed signal(s) kept separate from {preference_count} \
             explicit preference(s) — never merge into intent."
        )
    } else {
        "Insufficient evidence for observed-vs-preferred contrast.".into()
    };

    WorkingStyleSummary {
        headline: format!("How \"{label}\" usually works"),
        rhythm_line: rhythm,
        organization_line: organization,
        workflow_line: workflow,
        context_switching_line: switching,
        preference_line: preference,
        observed_vs_preferred_line: observed_vs_preferred,
        narrative: format!(
            "Working Style for \"{label}\" explains operating patterns without watching or scoring \
             the user. {observed_count} observed · {preference_count} explicit. Never certainty."
        ),
    }
}

fn first_line(observations: &[WorkingStyleObservation], kind: WorkingStyleKind) -> Option<String> {
    observations
        .iter()
        .find(|o| o.kind == kind)
        .map(|o| o.summary.clone())
}

fn obs(
    id: &str,
    kind: WorkingStyleKind,
    origin: WorkingStyleOrigin,
    title: String,
    summary: String,
    confidence: WorkingStyleConfidence,
    evidence: Vec<WorkingStyleEvidence>,
    affected_context: String,
    explanation: String,
    why: String,
) -> WorkingStyleObservation {
    WorkingStyleObservation {
        id: id.into(),
        kind,
        origin,
        title,
        summary,
        confidence,
        evidence,
        affected_context,
        explanation,
        why,
        authority_effect: WorkingStyleObservation::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn audit_event(
    db: &Arc<Mutex<Database>>,
    actor: &ActorContext,
    event: &str,
    state: &WorkspaceWorkingStyleState,
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
