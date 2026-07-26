//! Workspace Interaction Model (Phase 6).
//!
//! Projects existing understanding into user-facing interaction opportunities.
//! Owns nothing. Never plans, executes, recommends (as engine), or automates.
//! Select creates Intent handoff only — Permission Gateway remains required.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_interaction_summary, interaction_now_rfc3339, validate_interaction_workspace_id,
    ActorContext, AdaptationStatus, DecisionState, IntentContext, InteractionEvidence,
    InteractionHandoff, InteractionItem, InteractionItemState, InteractionKind,
    InteractionPriority, InteractionSelectResult, InteractionSummary, WorkspaceExperienceState,
    WorkspaceIntelligenceState, WorkspaceInteractionComparison, WorkspaceInteractionState,
    WorkspaceInteractionSummary, WorkspaceInteractionValidation, WorkspaceSessionState,
    WorkspaceTransitionState,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceSessionService, WorkspaceTransitionService,
};

/// Process-local selection overlays (Selected / HandedOff). Not a second store;
/// never mutates tasks/layouts — status is informational only.
fn selection_overlays() -> &'static Mutex<HashMap<String, InteractionItemState>> {
    static OVERLAYS: OnceLock<Mutex<HashMap<String, InteractionItemState>>> = OnceLock::new();
    OVERLAYS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn overlay_key(workspace_id: &str, item_id: &str) -> String {
    format!("{workspace_id}::{item_id}")
}

pub(crate) struct WorkspaceInteractionService;

impl WorkspaceInteractionService {
    /// Standalone generate — loads Intelligence stack, then projects interactions.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceInteractionState> {
        let workspace_id = workspace_id.into();
        let workspace_name = workspace_name.into();
        let health_label = health_label.into();
        let intelligence = WorkspaceIntelligenceService::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            workspace_name,
            health_label,
        )?;
        let session = WorkspaceSessionService::generate_with_inputs(db, actor, &intelligence)?;
        let experience = WorkspaceExperienceService::generate_with_inputs(db, actor, &session)?;
        // Prefer Intelligence-embedded transition summary path: rebuild full Transition
        // from the same stack Intelligence used (deterministic, no new cognition).
        let work_context = crate::services::WorkspaceWorkContextService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
        )?;
        let navigation = crate::services::WorkspaceNavigationService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
        )?;
        let milestones = crate::services::WorkspaceMilestoneService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
        )?;
        let working_style = crate::services::WorkspaceWorkingStyleService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
        )?;
        let transition = WorkspaceTransitionService::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &work_context,
            &navigation,
            &milestones,
            &working_style,
        )?;
        Self::generate_with_inputs(
            db,
            actor,
            &intelligence,
            &session,
            &experience,
            &transition,
        )
    }

    /// Preferred path — consume already-assembled projections (never regenerate cognition).
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
        session: &WorkspaceSessionState,
        experience: &WorkspaceExperienceState,
        transition: &WorkspaceTransitionState,
    ) -> Result<WorkspaceInteractionState> {
        let _ = validate_interaction_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = if !session.label.is_empty() {
            session.label.clone()
        } else {
            intelligence.workspace_name.clone()
        };

        let mut items = Vec::new();

        // Continue interrupted / focused work (Continuity + Session).
        if let Some(focus) = &intelligence.continuity.current_focus {
            items.push(item(
                format!("interaction:continue:{}", focus.id.as_str()),
                InteractionKind::ContinueWork,
                "continuity",
                format!("Continue your interrupted {}", focus.title),
                focus.summary.clone(),
                format!(
                    "Continuity still lists \"{}\" as current focus — Interaction surfaces it; \
                     Continuity remains SoT.",
                    focus.title
                ),
                "Open this work in context and continue via Intent".into(),
                format!(
                    "Continue work on: {} — {}",
                    focus.title, focus.summary
                ),
                InteractionPriority::High,
                vec![evidence(
                    "Continuity focus",
                    "continuity",
                    focus.id.as_str(),
                    "You see this because Continuity reports unfinished focus.",
                )],
                format!(
                    "Why am I seeing this? Continuity focus \"{}\" is still open \
                     (interrupted_count={}).",
                    focus.title, intelligence.continuity.interrupted_count
                ),
            ));
        } else if intelligence.continuity.interrupted_count > 0
            || intelligence.continuity.resumable_count > 0
        {
            let title = intelligence
                .continuity
                .suggested_next_step
                .as_ref()
                .map(|s| s.title.clone())
                .unwrap_or_else(|| "Continue prior work".into());
            let source_ref = intelligence
                .continuity
                .suggested_next_step
                .as_ref()
                .map(|s| s.id.as_str().to_string())
                .unwrap_or_else(|| "continuity:interrupted".into());
            items.push(item(
                format!("interaction:continue:{source_ref}"),
                InteractionKind::ContinueWork,
                "continuity",
                format!("Continue your interrupted work: {title}"),
                intelligence.continuity.summary.clone(),
                "Interrupted or resumable Continuity signals exist — Interaction does not restore."
                    .into(),
                "Review Continuity and submit Intent to resume".into(),
                format!("Resume interrupted work: {title}"),
                InteractionPriority::High,
                vec![evidence(
                    "Interrupted continuity",
                    "continuity",
                    &source_ref,
                    "You see this because Continuity reports interrupted/resumable work.",
                )],
                format!(
                    "Why am I seeing this? Continuity interrupted={} resumable={}.",
                    intelligence.continuity.interrupted_count,
                    intelligence.continuity.resumable_count
                ),
            ));
        }

        // Review pending decisions — reference Decision Queue; do not re-rank.
        for decision in intelligence.decision_queue.items.iter().take(4) {
            if decision.decision_state == DecisionState::Dismissed
                || decision.decision_state == DecisionState::Accepted
                || decision.decision_state == DecisionState::Rejected
            {
                continue;
            }
            let id = format!("interaction:decision:{}", decision.id.as_str());
            items.push(item(
                id,
                InteractionKind::ReviewDecision,
                "decision_queue",
                format!("Review pending decision: {}", decision.title),
                decision.summary.clone(),
                format!(
                    "Surfaced from Decision Queue (source {} / {}). Interaction does not own \
                     the queue — open Decision Queue to act.",
                    decision.source_type.as_str(),
                    decision.source_id
                ),
                decision.recommended_action.clone(),
                format!(
                    "Review Decision Queue item: {} — {}",
                    decision.title, decision.explanation
                ),
                match decision.priority {
                    workspace_domain::DecisionPriority::Critical
                    | workspace_domain::DecisionPriority::High => InteractionPriority::High,
                    workspace_domain::DecisionPriority::Normal => InteractionPriority::Medium,
                    workspace_domain::DecisionPriority::Low => InteractionPriority::Low,
                },
                vec![evidence(
                    "Decision Queue item",
                    "decision_queue",
                    decision.id.as_str(),
                    "You see this because Decision Queue still lists this pending item.",
                )],
                format!(
                    "Why am I seeing this? Decision Queue pending item \"{}\" \
                     (state={}, priority={}).",
                    decision.title,
                    decision.decision_state.as_str(),
                    decision.priority.as_str()
                ),
            ));
        }

        // Inspect recommendations — reference Recommendation Engine; do not duplicate engine.
        // Sort by id before take so the same candidate set yields stable interaction ids.
        let mut rec_candidates = intelligence.recommendation_engine.top_candidates.clone();
        rec_candidates.sort_by(|a, b| a.id.cmp(&b.id));
        for rec in rec_candidates.iter().take(3) {
            items.push(item(
                format!("interaction:recommendation:{}", rec.id),
                InteractionKind::InspectRecommendation,
                "recommendation_engine",
                format!("Look at recommended next step: {}", rec.title),
                rec.reason.clone(),
                format!(
                    "Surfaced from Recommendation Engine candidate {}. Interaction does not \
                     invent suggestions — inspect RE for full evidence.",
                    rec.id
                ),
                "Inspect Recommendation Engine candidate, then submit Intent if desired".into(),
                format!(
                    "Inspect recommendation: {} — {}",
                    rec.title, rec.reason
                ),
                match rec.confidence {
                    workspace_domain::RecommendationConfidence::High => InteractionPriority::Medium,
                    workspace_domain::RecommendationConfidence::Medium => InteractionPriority::Medium,
                    workspace_domain::RecommendationConfidence::Low => InteractionPriority::Low,
                },
                vec![evidence(
                    "Recommendation candidate",
                    "recommendation_engine",
                    &rec.id,
                    "You see this because Recommendation Engine listed this candidate.",
                )],
                format!(
                    "Why am I seeing this? Recommendation Engine candidate \"{}\" (kind={}).",
                    rec.title,
                    rec.kind.as_str()
                ),
            ));
        }

        // Review adaptations — reference Adaptation proposals.
        let mut adaptation_proposals = intelligence.adaptation.top_proposals.clone();
        adaptation_proposals.sort_by(|a, b| a.id.cmp(&b.id));
        for proposal in adaptation_proposals.iter().take(3) {
            if matches!(
                proposal.status,
                AdaptationStatus::Rejected | AdaptationStatus::Accepted
            ) {
                continue;
            }
            items.push(item(
                format!("interaction:adaptation:{}", proposal.id),
                InteractionKind::ReviewAdaptation,
                "adaptation",
                format!("Look at recommended workspace improvement: {}", proposal.title),
                proposal.reason.clone(),
                format!(
                    "Surfaced from Adaptation Proposal {}. Interaction never applies changes — \
                     review Adaptation, accept only creates Intent handoff.",
                    proposal.id
                ),
                "Review Adaptation Proposal (accept → Intent handoff only)".into(),
                format!(
                    "Review adaptation: {} — {}",
                    proposal.title, proposal.reason
                ),
                InteractionPriority::Medium,
                vec![evidence(
                    "Adaptation proposal",
                    "adaptation",
                    &proposal.id,
                    "You see this because Adaptation still has an open proposal.",
                )],
                format!(
                    "Why am I seeing this? Open Adaptation proposal \"{}\" (status={}).",
                    proposal.title,
                    proposal.status.as_str()
                ),
            ));
        }

        // Resolve blockers — Continuity / Readiness / blocked_actions / milestones.
        for blocked in intelligence.blocked_actions.iter().take(3) {
            items.push(item(
                format!("interaction:blocker:action:{}", blocked.id),
                InteractionKind::ResolveBlocker,
                "blocked_actions",
                format!("Resolve blocked action: {}", blocked.summary),
                blocked.explanation.clone(),
                "Blocked action listed on Intelligence — Interaction only surfaces it.".into(),
                "Inspect blocker and submit Intent to resolve".into(),
                format!(
                    "Resolve blocker: {} — {}",
                    blocked.summary, blocked.explanation
                ),
                InteractionPriority::High,
                vec![evidence(
                    "Blocked action",
                    "blocked_actions",
                    &blocked.id,
                    "You see this because Intelligence lists this blocked action.",
                )],
                format!(
                    "Why am I seeing this? Blocked action \"{}\" is unresolved.",
                    blocked.summary
                ),
            ));
        }
        if intelligence.continuity.blocker_count > 0 && intelligence.blocked_actions.is_empty() {
            items.push(item(
                "interaction:blocker:continuity".into(),
                InteractionKind::ResolveBlocker,
                "continuity",
                "Resolve blocked task or commitment".into(),
                format!(
                    "Continuity reports {} blocker signal(s).",
                    intelligence.continuity.blocker_count
                ),
                "Continuity blocker_count > 0 — Interaction does not clear blockers.".into(),
                "Inspect Continuity blockers via Intent".into(),
                "Investigate Continuity blockers".into(),
                InteractionPriority::High,
                vec![evidence(
                    "Continuity blockers",
                    "continuity",
                    "blocker_count",
                    "You see this because Continuity reports blockers.",
                )],
                format!(
                    "Why am I seeing this? Continuity blocker_count={}.",
                    intelligence.continuity.blocker_count
                ),
            ));
        }
        if intelligence.readiness.blocked_count > 0 || intelligence.readiness.gap_count > 0 {
            let gap = intelligence
                .readiness
                .top_gaps
                .first()
                .map(|g| g.title.clone())
                .unwrap_or_else(|| "readiness gap".into());
            items.push(item(
                format!("interaction:blocker:readiness:{gap}"),
                InteractionKind::ResolveBlocker,
                "readiness",
                format!("Resolve readiness gap: {gap}"),
                intelligence.readiness.readiness_summary.gap_line.clone(),
                "Readiness reports gaps/blocked assessments — Interaction never prepares."
                    .into(),
                "Review Readiness gaps, then Intent".into(),
                format!("Address readiness gap: {gap}"),
                InteractionPriority::Medium,
                vec![evidence(
                    "Readiness gap",
                    "readiness",
                    &gap,
                    "You see this because Readiness reports a gap or blocked assessment.",
                )],
                format!(
                    "Why am I seeing this? Readiness gaps={} blocked={}.",
                    intelligence.readiness.gap_count, intelligence.readiness.blocked_count
                ),
            ));
        }
        if intelligence.milestones.blocked_count > 0 {
            let title = intelligence
                .milestones
                .top_milestones
                .iter()
                .find(|m| m.status == workspace_domain::MilestoneStatus::Blocked)
                .map(|m| m.title.clone())
                .or_else(|| intelligence.milestones.current_milestone_title.clone())
                .unwrap_or_else(|| "blocked milestone".into());
            items.push(item(
                format!("interaction:blocker:milestone:{title}"),
                InteractionKind::ResolveBlocker,
                "milestones",
                format!("Resolve blocked milestone: {title}"),
                intelligence.milestones.milestone_summary.blocked_line.clone(),
                "Milestone Engine reports blocked progress — Interaction never completes milestones."
                    .into(),
                "Inspect Milestone blockers via Intent".into(),
                format!("Unblock milestone progress: {title}"),
                InteractionPriority::Medium,
                vec![evidence(
                    "Blocked milestone",
                    "milestones",
                    &title,
                    "You see this because Milestones report blocked progress.",
                )],
                format!(
                    "Why am I seeing this? Milestone blocked_count={}.",
                    intelligence.milestones.blocked_count
                ),
            ));
        }

        // Open context — Navigation / Work Context.
        if let Some(path) = intelligence.navigation.top_paths.first() {
            let path_ref = format!("{}:{}", path.kind.as_str(), path.title);
            items.push(item(
                format!("interaction:context:nav:{path_ref}"),
                InteractionKind::OpenContext,
                "navigation",
                format!("Open related context: {}", path.title),
                path.why.clone(),
                "Navigation suggests an inspection path — Interaction never routes or applies."
                    .into(),
                "Inspect Navigation path (read-only), then Intent".into(),
                format!("Open navigation context: {}", path.title),
                InteractionPriority::Low,
                vec![evidence(
                    "Navigation path",
                    "navigation",
                    &path_ref,
                    "You see this because Navigation lists a suggested path.",
                )],
                format!(
                    "Why am I seeing this? Navigation suggested path \"{}\".",
                    path.title
                ),
            ));
        } else if let Some(ctx) = intelligence.work_context.top_contexts.first() {
            items.push(item(
                format!("interaction:context:wc:{}", ctx.id),
                InteractionKind::OpenContext,
                "work_context",
                format!("Open work context: {}", ctx.name),
                ctx.why.clone(),
                "Work Context classifies kind-of-work — Interaction never changes context."
                    .into(),
                "Inspect Work Context via Intent".into(),
                format!("Open work context: {}", ctx.name),
                InteractionPriority::Low,
                vec![evidence(
                    "Work context",
                    "work_context",
                    &ctx.id,
                    "You see this because Work Context lists this context.",
                )],
                format!(
                    "Why am I seeing this? Work Context \"{}\" is available.",
                    ctx.name
                ),
            ));
        }

        // Review progress — Milestones / Continuity recent progress.
        if intelligence.milestones.milestone_count > 0 {
            let title = intelligence
                .milestones
                .current_milestone_title
                .clone()
                .unwrap_or_else(|| "workspace progress".into());
            items.push(item(
                format!("interaction:progress:{}", title),
                InteractionKind::ReviewProgress,
                "milestones",
                format!("View recent progress: {title}"),
                intelligence.milestones.milestone_summary.narrative.clone(),
                "Milestone progress is informational — Interaction never schedules or completes."
                    .into(),
                "Review Milestone Engine progress".into(),
                format!("Review progress toward: {title}"),
                InteractionPriority::Low,
                vec![evidence(
                    "Milestone progress",
                    "milestones",
                    &title,
                    "You see this because Milestones report current progress.",
                )],
                format!(
                    "Why am I seeing this? Milestones current=\"{title}\" completed={}.",
                    intelligence.milestones.completed_count
                ),
            ));
        } else if let Some(progress) = intelligence.continuity.recent_progress.first() {
            items.push(item(
                format!("interaction:progress:cont:{}", progress.id.as_str()),
                InteractionKind::ReviewProgress,
                "continuity",
                format!("View recent progress: {}", progress.title),
                progress.summary.clone(),
                "Continuity recent progress — Interaction never mutates Continuity.".into(),
                "Review Continuity progress narrative".into(),
                format!("Review progress: {}", progress.title),
                InteractionPriority::Low,
                vec![evidence(
                    "Continuity progress",
                    "continuity",
                    progress.id.as_str(),
                    "You see this because Continuity recorded recent progress.",
                )],
                format!(
                    "Why am I seeing this? Continuity recent progress \"{}\".",
                    progress.title
                ),
            ));
        }

        // Understand change — Transition / Evolution.
        if let Some(t) = transition.transitions.first() {
            items.push(item(
                format!("interaction:change:{}", t.id),
                InteractionKind::UnderstandChange,
                "transition",
                format!("Understand recent change: {}", t.title),
                t.explanation.clone(),
                "Transition explains movement — Interaction never restores or executes.".into(),
                "Inspect Transition Engine explanation".into(),
                format!("Understand transition: {} — {}", t.title, t.why),
                InteractionPriority::Medium,
                vec![evidence(
                    "Transition",
                    "transition",
                    &t.id,
                    "You see this because Transition Engine explained a state change.",
                )],
                format!(
                    "Why am I seeing this? Transition \"{}\" (kind={}).",
                    t.title,
                    t.kind.as_str()
                ),
            ));
        } else if !intelligence.evolution.summary.is_empty() {
            items.push(item(
                "interaction:change:evolution".into(),
                InteractionKind::UnderstandChange,
                "evolution",
                "Understand how work evolved".into(),
                intelligence.evolution.summary.clone(),
                "Evolution Model summary — Interaction does not invent history.".into(),
                "Inspect Evolution Model".into(),
                format!("Understand evolution: {}", intelligence.evolution.summary),
                InteractionPriority::Low,
                vec![evidence(
                    "Evolution",
                    "evolution",
                    "summary",
                    "You see this because Evolution summarized workspace change.",
                )],
                "Why am I seeing this? Evolution Model has a non-empty summary.".into(),
            ));
        }

        // Deterministic order: priority desc, then kind, then id.
        items.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
                .then_with(|| a.id.cmp(&b.id))
        });

        // Apply process-local selection overlays.
        if let Ok(guard) = selection_overlays().lock() {
            for it in &mut items {
                if let Some(state) = guard.get(&overlay_key(&intelligence.workspace_id, &it.id)) {
                    it.state = *state;
                }
            }
        }

        let continue_count = count_kind(&items, InteractionKind::ContinueWork);
        let decision_count = count_kind(&items, InteractionKind::ReviewDecision);
        let recommendation_count = count_kind(&items, InteractionKind::InspectRecommendation);
        let adaptation_count = count_kind(&items, InteractionKind::ReviewAdaptation);
        let blocker_count = count_kind(&items, InteractionKind::ResolveBlocker);

        let interaction_summary = InteractionSummary {
            headline: if items.is_empty() {
                format!("No interaction opportunities for \"{label}\" right now")
            } else {
                format!(
                    "{} thing(s) you can do in \"{label}\"",
                    items.len()
                )
            },
            continue_line: line_for(
                &items,
                InteractionKind::ContinueWork,
                "No interrupted work to continue",
            ),
            decision_line: line_for(
                &items,
                InteractionKind::ReviewDecision,
                "No pending decisions to review",
            ),
            recommendation_line: line_for(
                &items,
                InteractionKind::InspectRecommendation,
                "No recommendations to inspect",
            ),
            adaptation_line: line_for(
                &items,
                InteractionKind::ReviewAdaptation,
                "No adaptations to review",
            ),
            blocker_line: line_for(
                &items,
                InteractionKind::ResolveBlocker,
                "No blockers to resolve",
            ),
            progress_line: line_for(
                &items,
                InteractionKind::ReviewProgress,
                "No progress to review",
            ),
            narrative: format!(
                "Interaction Model aggregates Session, Experience, Decision Queue, \
                 Recommendation Engine, Adaptation, Readiness, Milestones, Transition, \
                 and Navigation into opportunities. Owns nothing. Never executes. \
                 Selecting an item creates Intent handoff only."
            ),
        };

        let explanation = format!(
            "Interaction Model for \"{label}\" surfaces meaningful next interactions from \
             existing cognition. Does not duplicate Decision Queue or Recommendation Engine — \
             references them. authority_effect=none. session={} experience={} transition={}.",
            session.generated_at, experience.generated_at, transition.generated_at
        );

        let mut evidence_lines = vec![
            format!("session:{}", session.generated_at),
            format!("experience:{}", experience.generated_at),
            format!("transition:{}", transition.generated_at),
            format!(
                "decision_queue:pending={}",
                intelligence.decision_queue.pending_count
            ),
            format!(
                "recommendation_engine:candidates={}",
                intelligence.recommendation_engine.candidate_count
            ),
            format!(
                "adaptation:open={}",
                intelligence.adaptation.open_count
            ),
            format!(
                "readiness:gaps={}",
                intelligence.readiness.gap_count
            ),
            format!(
                "milestones:count={}",
                intelligence.milestones.milestone_count
            ),
            format!(
                "navigation:suggested={}",
                intelligence.navigation.suggested_count
            ),
            format!(
                "continuity:interrupted={}",
                intelligence.continuity.interrupted_count
            ),
        ];
        evidence_lines.sort();
        evidence_lines.dedup();

        let summary = build_interaction_summary(&label, items.len());

        let state = WorkspaceInteractionState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: interaction_now_rfc3339(),
            label,
            interaction_summary,
            item_count: items.len(),
            continue_count,
            decision_count,
            recommendation_count,
            adaptation_count,
            blocker_count,
            items,
            session_generated_at: session.generated_at.clone(),
            experience_generated_at: experience.generated_at.clone(),
            transition_generated_at: transition.generated_at.clone(),
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence: evidence_lines,
            summary,
            authority_effect: WorkspaceInteractionState::AUTHORITY_EFFECT_NONE.into(),
        };

        audit_event(
            db,
            actor,
            "workspace.interaction.generated",
            &state,
            json!({
                "item_count": state.item_count,
                "decision_count": state.decision_count,
                "recommendation_count": state.recommendation_count,
            }),
        )?;
        audit_event(
            db,
            actor,
            "workspace.interaction.updated",
            &state,
            json!({ "item_count": state.item_count }),
        )?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceInteractionState,
        limit: usize,
    ) -> WorkspaceInteractionSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceInteractionState,
        right: &WorkspaceInteractionState,
    ) -> WorkspaceInteractionComparison {
        WorkspaceInteractionComparison::compare(left, right)
    }

    pub(crate) fn validate(state: &WorkspaceInteractionState) -> WorkspaceInteractionValidation {
        WorkspaceInteractionValidation::validate(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceInteractionState,
    ) -> Result<WorkspaceInteractionValidation> {
        let report = Self::validate(state);
        audit_event(
            db,
            actor,
            "workspace.interaction.updated",
            state,
            json!({
                "validated": report.valid,
                "message_count": report.messages.len(),
            }),
        )?;
        Ok(report)
    }

    /// Select an interaction — Intent handoff only. Never executes.
    pub(crate) fn select(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
        interaction_id: impl Into<String>,
    ) -> Result<InteractionSelectResult> {
        let workspace_id = workspace_id.into();
        let interaction_id = interaction_id.into();
        let state = Self::generate(
            db,
            actor,
            orchestrated_plans,
            assistant_workflows,
            workspace_id.clone(),
            workspace_name,
            health_label,
        )?;
        let found = state
            .find_item(&interaction_id)
            .cloned()
            .ok_or_else(|| {
                KernelError::from(workspace_domain::WorkspaceInteractionError::NotFound(
                    interaction_id.clone(),
                ))
            })?;

        let mut updated = found;
        updated.state = InteractionItemState::HandedOff;
        if let Ok(mut guard) = selection_overlays().lock() {
            guard.insert(
                overlay_key(&workspace_id, &updated.id),
                InteractionItemState::HandedOff,
            );
        }

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.interaction.selected",
            true,
            json!({
                "workspace_id": workspace_id,
                "interaction_id": updated.id,
                "kind": updated.kind.as_str(),
                "authority_effect": "none",
            })
            .to_string(),
        )?;

        let handoff = InteractionHandoff {
            interaction_id: updated.id.clone(),
            kind: updated.kind,
            next_command: InteractionHandoff::NEXT_SUBMIT_ASSISTANT_GOAL.into(),
            intent_statement: updated.required_intent.clone(),
            workspace_id: workspace_id.clone(),
            note: "Interaction selected as a human decision to act. Call submit_assistant_goal \
                   to plan — Interaction never executes, mutates layout, or bypasses Permission \
                   Gateway."
                .into(),
            authority_effect: InteractionHandoff::AUTHORITY_EFFECT_NONE.into(),
        };

        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.interaction.handoff_created",
            true,
            json!({
                "workspace_id": workspace_id,
                "interaction_id": handoff.interaction_id,
                "next_command": handoff.next_command,
                "authority_effect": "none",
            })
            .to_string(),
        )?;

        Ok(InteractionSelectResult {
            item: Some(updated),
            handoff: Some(handoff),
            authority_effect: InteractionItem::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceInteractionError::CannotExecute,
        ))
    }
}

fn count_kind(items: &[InteractionItem], kind: InteractionKind) -> usize {
    items.iter().filter(|i| i.kind == kind).count()
}

fn line_for(items: &[InteractionItem], kind: InteractionKind, empty: &str) -> String {
    items
        .iter()
        .find(|i| i.kind == kind)
        .map(|i| i.title.clone())
        .unwrap_or_else(|| empty.into())
}

fn evidence(label: &str, source: &str, source_ref: &str, why: &str) -> InteractionEvidence {
    InteractionEvidence {
        label: label.into(),
        source_projection: source.into(),
        source_ref: source_ref.into(),
        why: why.into(),
    }
}

#[allow(clippy::too_many_arguments)]
fn item(
    id: String,
    kind: InteractionKind,
    source_projection: &str,
    title: String,
    description: String,
    explanation: String,
    available_action: String,
    required_intent: String,
    priority: InteractionPriority,
    evidence: Vec<InteractionEvidence>,
    why: String,
) -> InteractionItem {
    InteractionItem {
        id,
        kind,
        source_projection: source_projection.into(),
        title,
        description,
        explanation,
        available_action,
        required_intent,
        state: InteractionItemState::Available,
        priority,
        evidence,
        why,
        authority_effect: InteractionItem::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn audit_event(
    db: &Arc<Mutex<Database>>,
    actor: &ActorContext,
    event: &str,
    state: &WorkspaceInteractionState,
    extra: serde_json::Value,
) -> Result<()> {
    let mut payload = json!({
        "workspace_id": state.workspace_id,
        "item_count": state.item_count,
        "authority_effect": "none",
    });
    if let Some(obj) = payload.as_object_mut() {
        if let Some(extra_obj) = extra.as_object() {
            for (k, v) in extra_obj {
                obj.insert(k.clone(), v.clone());
            }
        }
    }
    AuditService::record_ai_planning_event(
        db,
        actor,
        &IntentContext::user_request(),
        event,
        true,
        payload.to_string(),
    )
}
