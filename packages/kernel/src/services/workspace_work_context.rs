//! Workspace Work Context Engine (Phase 6).
//!
//! Semantic projection over Session + Experience + Intelligence outputs.
//! Owns nothing. Deterministic classification only — never AI, never authority.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{
    build_work_context_summary, validate_work_context_workspace_id, work_context_now_rfc3339,
    ActorContext, ExperienceSectionKind, IntentContext, ReadinessStatus, SessionMemberKind,
    WorkContext, WorkContextAssociation, WorkContextConfidence, WorkContextEvidence,
    WorkContextRelationKind, WorkContextRelationship, WorkContextStatus, WorkContextType,
    WorkspaceExperienceState, WorkspaceIntelligenceState, WorkspaceSessionState,
    WorkspaceWorkContextComparison, WorkspaceWorkContextState, WorkspaceWorkContextSummary,
    WorkspaceWorkContextValidation,
};

use crate::error::{KernelError, Result};
use crate::services::{
    AssistantWorkflowStore, AuditService, OrchestratedPlanStore, WorkspaceExperienceService,
    WorkspaceIntelligenceService, WorkspaceSessionService,
};

pub(crate) struct WorkspaceWorkContextService;

impl WorkspaceWorkContextService {
    /// Standalone generate — loads Intelligence → Session → Experience, then projects.
    pub(crate) fn generate(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        orchestrated_plans: &Arc<Mutex<OrchestratedPlanStore>>,
        assistant_workflows: &Arc<Mutex<AssistantWorkflowStore>>,
        workspace_id: impl Into<String>,
        workspace_name: impl Into<String>,
        health_label: impl Into<String>,
    ) -> Result<WorkspaceWorkContextState> {
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
        Self::generate_with_inputs(db, actor, &intelligence, &session, &experience)
    }

    /// Preferred path — consume already-assembled projections (never regenerate).
    pub(crate) fn generate_with_inputs(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        intelligence: &WorkspaceIntelligenceState,
        session: &WorkspaceSessionState,
        experience: &WorkspaceExperienceState,
    ) -> Result<WorkspaceWorkContextState> {
        let _ = validate_work_context_workspace_id(intelligence.workspace_id.clone())
            .map_err(KernelError::from)?;
        let label = session.label.clone();
        let corpus = build_signal_corpus(intelligence, session, experience);
        let mut contexts = classify_contexts(intelligence, session, experience, &corpus);

        if contexts.is_empty() {
            contexts.push(fallback_custom_context(intelligence, session, experience));
        }

        contexts.sort_by(|a, b| {
            confidence_rank(b.confidence)
                .cmp(&confidence_rank(a.confidence))
                .then(status_rank(a.current_status).cmp(&status_rank(b.current_status)))
                .then(a.id.cmp(&b.id))
        });
        let primary_context_id = contexts
            .iter()
            .find(|c| c.current_status == WorkContextStatus::Active)
            .or_else(|| contexts.first())
            .map(|c| c.id.clone());

        let relationships = build_relationships(&contexts, primary_context_id.as_deref());

        let active_count = contexts
            .iter()
            .filter(|c| c.current_status == WorkContextStatus::Active)
            .count();
        let blocked_count = contexts
            .iter()
            .filter(|c| c.current_status == WorkContextStatus::Blocked)
            .count();
        let dormant_count = contexts
            .iter()
            .filter(|c| c.current_status == WorkContextStatus::Dormant)
            .count();
        let candidate_count = contexts
            .iter()
            .filter(|c| c.current_status == WorkContextStatus::Candidate)
            .count();

        let primary_name = primary_context_id
            .as_ref()
            .and_then(|id| contexts.iter().find(|c| &c.id == id))
            .map(|c| c.name.clone());

        let evidence = vec![
            format!("Intelligence generated_at: {}", intelligence.generated_at),
            format!("Session generated_at: {}", session.generated_at),
            format!("Experience generated_at: {}", experience.generated_at),
            format!("Purpose label: {}", intelligence.purpose.label),
            format!("Environment: {}", intelligence.environment.summary),
            format!("Composition: {}", intelligence.composition.summary),
            format!(
                "Readiness: {}",
                intelligence.readiness.overall_status.as_str()
            ),
            format!("Contexts classified: {}", contexts.len()),
        ];

        let explanation = format!(
            "Work Context for \"{label}\" is a semantic projection over WorkspaceSessionState, \
             WorkspaceExperienceState, and Intelligence pipeline outputs (Purpose, Composition, \
             Environment, Task Graph, Continuity, Evolution, Activity, Decision Queue, Patterns, \
             Recommendations, Readiness). Classifications are deterministic keyword evidence — \
             not AI inference. Work Context owns no cognition data and never plans, launches, \
             restores, or bypasses the Gateway."
        );
        let summary =
            build_work_context_summary(&label, contexts.len(), primary_name.as_deref());

        let state = WorkspaceWorkContextState {
            workspace_id: intelligence.workspace_id.clone(),
            workspace_name: intelligence.workspace_name.clone(),
            generated_at: work_context_now_rfc3339(),
            label,
            context_count: contexts.len(),
            active_count,
            blocked_count,
            dormant_count,
            candidate_count,
            primary_context_id,
            contexts,
            relationships,
            session_generated_at: session.generated_at.clone(),
            experience_generated_at: experience.generated_at.clone(),
            intelligence_generated_at: intelligence.generated_at.clone(),
            explanation,
            evidence,
            summary,
            authority_effect: WorkspaceWorkContextState::AUTHORITY_EFFECT_NONE.into(),
        };

        Self::audit_generated(db, actor, &state)?;
        Self::audit_updated(db, actor, &state)?;
        Ok(state)
    }

    pub(crate) fn summary_projection(
        state: &WorkspaceWorkContextState,
        limit: usize,
    ) -> WorkspaceWorkContextSummary {
        state.summary_projection(limit)
    }

    pub(crate) fn compare(
        left: &WorkspaceWorkContextState,
        right: &WorkspaceWorkContextState,
    ) -> WorkspaceWorkContextComparison {
        WorkspaceWorkContextComparison::compare(left, right)
    }

    pub(crate) fn validate(state: &WorkspaceWorkContextState) -> WorkspaceWorkContextValidation {
        WorkspaceWorkContextValidation::validate(state)
    }

    pub(crate) fn validate_and_audit(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceWorkContextState,
    ) -> Result<WorkspaceWorkContextValidation> {
        let report = Self::validate(state);
        Self::audit_validated(db, actor, state, &report)?;
        Ok(report)
    }

    pub(crate) fn attempt_execute() -> Result<()> {
        Err(KernelError::from(
            workspace_domain::WorkspaceWorkContextError::CannotExecute,
        ))
    }

    fn audit_generated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceWorkContextState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.work_context.generated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "context_count": state.context_count,
                "primary": state.primary_context().map(|c| c.name.clone()),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_updated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceWorkContextState,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.work_context.updated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "active_count": state.active_count,
                "blocked_count": state.blocked_count,
                "authority_effect": "none",
            })
            .to_string(),
        )
    }

    fn audit_validated(
        db: &Arc<Mutex<Database>>,
        actor: &ActorContext,
        state: &WorkspaceWorkContextState,
        report: &WorkspaceWorkContextValidation,
    ) -> Result<()> {
        AuditService::record_ai_planning_event(
            db,
            actor,
            &IntentContext::user_request(),
            "workspace.work_context.validated",
            true,
            json!({
                "workspace_id": state.workspace_id,
                "valid": report.valid,
                "message_count": report.messages.len(),
                "authority_effect": "none",
            })
            .to_string(),
        )
    }
}

fn confidence_rank(c: WorkContextConfidence) -> u8 {
    match c {
        WorkContextConfidence::High => 3,
        WorkContextConfidence::Medium => 2,
        WorkContextConfidence::Low => 1,
    }
}

fn status_rank(s: WorkContextStatus) -> u8 {
    match s {
        WorkContextStatus::Active => 0,
        WorkContextStatus::Blocked => 1,
        WorkContextStatus::Interrupted => 2,
        WorkContextStatus::Candidate => 3,
        WorkContextStatus::Dormant => 4,
    }
}

struct SignalHit {
    keyword: String,
    source_projection: String,
    source_ref: String,
    why: String,
}

struct SignalCorpus {
    hits: Vec<(WorkContextType, SignalHit)>,
}

fn build_signal_corpus(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
    experience: &WorkspaceExperienceState,
) -> SignalCorpus {
    let mut parts: Vec<String> = Vec::new();
    let mut typed_hits: Vec<(WorkContextType, SignalHit)> = Vec::new();

    let push = |parts: &mut Vec<String>,
                typed_hits: &mut Vec<(WorkContextType, SignalHit)>,
                text: &str,
                source_projection: &str,
                source_ref: &str,
                why: &str| {
        if text.trim().is_empty() {
            return;
        }
        parts.push(text.to_string());
        let lower = text.to_lowercase();
        for (ctx_type, keywords) in TYPE_KEYWORDS.iter() {
            for keyword in keywords.iter() {
                if lower.contains(*keyword) {
                    typed_hits.push((
                        *ctx_type,
                        SignalHit {
                            keyword: (*keyword).into(),
                            source_projection: source_projection.into(),
                            source_ref: source_ref.into(),
                            why: format!("{why} (matched `{keyword}`)"),
                        },
                    ));
                }
            }
        }
    };

    if let Some(p) = &intelligence.current_project {
        push(
            &mut parts,
            &mut typed_hits,
            &p.name,
            "intelligence.current_project",
            p.id.as_str(),
            "Active project name contributes to work-kind signals",
        );
    }
    if let Some(t) = &intelligence.current_task {
        push(
            &mut parts,
            &mut typed_hits,
            &t.title,
            "intelligence.current_task",
            t.id.as_str(),
            "Active task title contributes to work-kind signals",
        );
    }
    push(
        &mut parts,
        &mut typed_hits,
        &intelligence.purpose.label,
        "purpose",
        &intelligence.purpose.workspace_id,
        "Purpose label describes why work exists",
    );
    if let Some(goal) = &intelligence.purpose.primary_work_goal_description {
        push(
            &mut parts,
            &mut typed_hits,
            goal,
            "purpose.primary_work_goal",
            &intelligence.purpose.workspace_id,
            "Primary work goal describes intended outcome",
        );
    }
    push(
        &mut parts,
        &mut typed_hits,
        &intelligence.composition.summary,
        "composition",
        &intelligence.composition.workspace_id,
        "Composition describes the logical working environment",
    );
    push(
        &mut parts,
        &mut typed_hits,
        &intelligence.environment.summary,
        "environment",
        &intelligence.environment.workspace_id,
        "Environment summarizes live desktop signals",
    );
    for app in &intelligence.current_applications {
        push(
            &mut parts,
            &mut typed_hits,
            &app.name,
            "environment.application",
            &app.id,
            "Associated application name is a work-kind clue",
        );
    }
    push(
        &mut parts,
        &mut typed_hits,
        &intelligence.continuity.summary,
        "continuity",
        &intelligence.continuity.workspace_id,
        "Continuity narrates resumable / interrupted work",
    );
    push(
        &mut parts,
        &mut typed_hits,
        &intelligence.evolution.summary,
        "evolution",
        &intelligence.evolution.workspace_id,
        "Evolution describes how work changed",
    );
    push(
        &mut parts,
        &mut typed_hits,
        &intelligence.task_graph.summary,
        "task_graph",
        &intelligence.task_graph.workspace_id,
        "Task Graph summarizes structured work",
    );
    for goal in &intelligence.recent_goals {
        push(
            &mut parts,
            &mut typed_hits,
            &goal.description,
            "work_goal",
            goal.id.as_str(),
            "Recent work goal contributes semantic signals",
        );
    }
    for item in &intelligence.decision_queue.items {
        push(
            &mut parts,
            &mut typed_hits,
            &item.title,
            "decision_queue",
            item.id.as_str(),
            "Pending decision title contributes semantic signals",
        );
    }
    for rec in intelligence.recommendation_engine.top_candidates.iter().take(4) {
        push(
            &mut parts,
            &mut typed_hits,
            &rec.title,
            "recommendation_engine",
            &rec.id,
            "Recommendation candidate title contributes semantic signals",
        );
    }
    for pattern in intelligence.pattern.top_patterns.iter().take(4) {
        push(
            &mut parts,
            &mut typed_hits,
            &pattern.title,
            "pattern",
            &pattern.id,
            "Pattern title contributes recurring work-kind signals",
        );
    }
    for member in &session.members {
        push(
            &mut parts,
            &mut typed_hits,
            &member.label,
            "session.member",
            &member.id,
            "Session member label contributes semantic signals",
        );
    }
    push(
        &mut parts,
        &mut typed_hits,
        &experience.experience_summary.headline,
        "experience",
        &experience.workspace_id,
        "Experience headline reflects presented work focus",
    );
    push(
        &mut parts,
        &mut typed_hits,
        &experience.experience_summary.focus_line,
        "experience.focus",
        &experience.workspace_id,
        "Experience focus line mirrors current work",
    );

    let _ = parts;
    SignalCorpus { hits: typed_hits }
}

const TYPE_KEYWORDS: &[(WorkContextType, &[&str])] = &[
    (
        WorkContextType::Development,
        &[
            "develop",
            "development",
            "code",
            "coding",
            "build",
            "ship",
            "debug",
            "debugging",
            "rust",
            "typescript",
            "compiler",
            "platform",
            "engine",
            "api",
            "refactor",
            "implement",
            "workspace ai",
            "vibelock",
        ],
    ),
    (
        WorkContextType::Research,
        &[
            "research",
            "investigate",
            "explore",
            "analysis",
            "analyze",
            "study",
            "survey",
        ],
    ),
    (
        WorkContextType::Administration,
        &[
            "admin",
            "administration",
            "invoice",
            "payroll",
            "organize",
            "manage files",
            "compliance",
        ],
    ),
    (
        WorkContextType::Communication,
        &[
            "email",
            "meeting",
            "slack",
            "chat",
            "call",
            "message",
            "communication",
        ],
    ),
    (
        WorkContextType::Creative,
        &[
            "design",
            "creative",
            "video",
            "editing",
            "illustrat",
            "art",
            "music",
            "write novel",
        ],
    ),
    (
        WorkContextType::Learning,
        &[
            "learn",
            "learning",
            "tutorial",
            "course",
            "training",
            "onboarding",
            "documentation",
        ],
    ),
    (
        WorkContextType::Operations,
        &[
            "ops",
            "operations",
            "deploy",
            "incident",
            "monitor",
            "sre",
            "production",
        ],
    ),
    (
        WorkContextType::Planning,
        &[
            "plan",
            "planning",
            "roadmap",
            "sprint",
            "schedule",
            "prioritize",
            "backlog",
        ],
    ),
];

fn classify_contexts(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
    experience: &WorkspaceExperienceState,
    corpus: &SignalCorpus,
) -> Vec<WorkContext> {
    let mut by_type: BTreeMap<WorkContextType, Vec<&SignalHit>> = BTreeMap::new();
    for (ctx_type, hit) in &corpus.hits {
        by_type.entry(*ctx_type).or_default().push(hit);
    }

    let projects = collect_projects(intelligence, session);
    let tasks = collect_tasks(intelligence, session);
    let apps = collect_apps(intelligence, session);
    let purpose = collect_purpose(intelligence, session);
    let decisions = collect_decisions(intelligence, session);
    let activity = collect_activity(intelligence, session);
    let blocked_reasons = collect_blocked_reasons(intelligence, session);
    let recent_progress = collect_recent_progress(intelligence, experience);
    let interrupted = !session.interruptions.is_empty()
        || intelligence.continuity.summary.to_lowercase().contains("interrupt");
    let readiness_blocked = matches!(
        intelligence.readiness.overall_status,
        ReadinessStatus::Blocked
    );

    let mut out = Vec::new();
    for (ctx_type, hits) in by_type {
        // Dedup evidence by keyword+source
        let mut seen = BTreeSet::new();
        let mut evidence = Vec::new();
        for hit in hits {
            let key = format!("{}:{}:{}", hit.keyword, hit.source_projection, hit.source_ref);
            if seen.insert(key) {
                evidence.push(WorkContextEvidence {
                    label: hit.keyword.clone(),
                    source_projection: hit.source_projection.clone(),
                    source_ref: hit.source_ref.clone(),
                    why: hit.why.clone(),
                });
            }
        }
        evidence.truncate(8);
        if evidence.is_empty() {
            continue;
        }

        let confidence = WorkContextConfidence::from_evidence_count(evidence.len());
        let current_status = if readiness_blocked && !blocked_reasons.is_empty() {
            WorkContextStatus::Blocked
        } else if interrupted && ctx_type != WorkContextType::Development {
            WorkContextStatus::Interrupted
        } else if confidence == WorkContextConfidence::Low
            && ctx_type != WorkContextType::Development
        {
            WorkContextStatus::Candidate
        } else {
            WorkContextStatus::Active
        };

        let name = context_name(ctx_type, intelligence, session);
        let id = format!(
            "work_context:{}:{}",
            intelligence.workspace_id,
            ctx_type.as_str()
        );
        let suggested_focus = experience
            .experience_summary
            .next_line
            .clone()
            .if_empty_then(|| session.session_summary.doing_line.clone());

        out.push(WorkContext {
            id,
            name,
            context_type: ctx_type,
            confidence,
            associated_projects: projects.clone(),
            associated_tasks: tasks.clone(),
            associated_applications: apps.clone(),
            associated_purpose: purpose.clone(),
            associated_decisions: decisions.clone(),
            associated_activity: activity.clone(),
            current_status,
            suggested_focus,
            blocked_reasons: blocked_reasons.clone(),
            recent_progress: recent_progress.clone(),
            why: format!(
                "{} context recognized from {} deterministic evidence signal(s) across Session, \
                 Experience, and Intelligence projections.",
                ctx_type.display_name(),
                evidence.len()
            ),
            evidence,
            authority_effect: WorkContext::AUTHORITY_EFFECT_NONE.into(),
        });
    }

    // Dormant supporting context from continuity when interrupted work differs.
    if interrupted {
        if let Some(primary_dev) = out
            .iter()
            .find(|c| c.context_type == WorkContextType::Development)
            .cloned()
        {
            let dormant_id = format!(
                "work_context:{}:dormant_interrupted",
                intelligence.workspace_id
            );
            if !out.iter().any(|c| c.id == dormant_id) {
                out.push(WorkContext {
                    id: dormant_id,
                    name: format!("Interrupted — {}", primary_dev.name),
                    context_type: primary_dev.context_type,
                    evidence: vec![WorkContextEvidence {
                        label: "interrupted".into(),
                        source_projection: "continuity".into(),
                        source_ref: intelligence.continuity.workspace_id.clone(),
                        why: "Continuity reports interrupted work — dormant relative context"
                            .into(),
                    }],
                    confidence: WorkContextConfidence::Medium,
                    associated_projects: projects,
                    associated_tasks: tasks,
                    associated_applications: apps,
                    associated_purpose: purpose,
                    associated_decisions: decisions,
                    associated_activity: activity,
                    current_status: WorkContextStatus::Dormant,
                    suggested_focus: "Resume interrupted work when ready (informational only)"
                        .into(),
                    blocked_reasons,
                    recent_progress,
                    why: "Interrupted continuity signals a dormant related work context.".into(),
                    authority_effect: WorkContext::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }
    }

    out
}

trait IfEmptyThen {
    fn if_empty_then(self, f: impl FnOnce() -> String) -> String;
}

impl IfEmptyThen for String {
    fn if_empty_then(self, f: impl FnOnce() -> String) -> String {
        if self.trim().is_empty() {
            f()
        } else {
            self
        }
    }
}

fn context_name(
    ctx_type: WorkContextType,
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> String {
    let project = intelligence
        .current_project
        .as_ref()
        .map(|p| p.name.as_str())
        .or(session.focus.project_label.as_deref());
    match (ctx_type, project) {
        (WorkContextType::Development, Some(p)) => format!("{p} Development"),
        (WorkContextType::Research, Some(p)) => format!("{p} Research"),
        (WorkContextType::Planning, Some(p)) => format!("{p} Planning"),
        (WorkContextType::Learning, Some(p)) => format!("{p} Learning"),
        (WorkContextType::Creative, Some(p)) => format!("{p} Creative"),
        (WorkContextType::Operations, Some(p)) => format!("{p} Operations"),
        (WorkContextType::Administration, Some(p)) => format!("{p} Administration"),
        (WorkContextType::Communication, Some(p)) => format!("{p} Communication"),
        (WorkContextType::Custom, Some(p)) => format!("{p} Work"),
        (t, _) => format!("{} Work", t.display_name()),
    }
}

fn fallback_custom_context(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
    experience: &WorkspaceExperienceState,
) -> WorkContext {
    WorkContext {
        id: format!("work_context:{}:custom", intelligence.workspace_id),
        name: context_name(WorkContextType::Custom, intelligence, session),
        context_type: WorkContextType::Custom,
        evidence: vec![WorkContextEvidence {
            label: "session.focus".into(),
            source_projection: "session".into(),
            source_ref: session.workspace_id.clone(),
            why: "No typed keyword matches; Custom context anchors on Session focus.".into(),
        }],
        confidence: WorkContextConfidence::Low,
        associated_projects: collect_projects(intelligence, session),
        associated_tasks: collect_tasks(intelligence, session),
        associated_applications: collect_apps(intelligence, session),
        associated_purpose: collect_purpose(intelligence, session),
        associated_decisions: collect_decisions(intelligence, session),
        associated_activity: collect_activity(intelligence, session),
        current_status: WorkContextStatus::Active,
        suggested_focus: experience.experience_summary.focus_line.clone(),
        blocked_reasons: collect_blocked_reasons(intelligence, session),
        recent_progress: collect_recent_progress(intelligence, experience),
        why: "Custom context used when typed classifications lack keyword evidence.".into(),
        authority_effect: WorkContext::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn collect_projects(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Vec<WorkContextAssociation> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    if let Some(p) = &intelligence.current_project {
        if seen.insert(p.id.to_string()) {
            out.push(WorkContextAssociation {
                id: p.id.to_string(),
                label: p.name.clone(),
                kind: "project".into(),
                source_projection: "intelligence.current_project".into(),
                source_ref: p.id.to_string(),
                why: "Active project participates in this work context".into(),
            });
        }
    }
    for member in session.members.iter().filter(|m| m.kind == SessionMemberKind::Project) {
        if seen.insert(member.source_ref.clone()) {
            out.push(WorkContextAssociation {
                id: member.id.clone(),
                label: member.label.clone(),
                kind: "project".into(),
                source_projection: "session.member".into(),
                source_ref: member.source_ref.clone(),
                why: member.why.clone(),
            });
        }
    }
    // Goals may associate additional projects with the same context.
    for goal in &intelligence.recent_goals {
        if let Some(pid) = &goal.project_id {
            let key = pid.to_string();
            if seen.insert(key.clone()) {
                out.push(WorkContextAssociation {
                    id: key.clone(),
                    label: goal.description.clone(),
                    kind: "project".into(),
                    source_projection: "work_goal".into(),
                    source_ref: key,
                    why: "Work Goal links another project into this work context".into(),
                });
            }
        }
    }
    for node in intelligence.task_graph.top_nodes.iter().take(8) {
        if let Some(pid) = &node.task.project_id {
            if seen.insert(pid.clone()) {
                out.push(WorkContextAssociation {
                    id: pid.clone(),
                    label: node.task.title.clone(),
                    kind: "project".into(),
                    source_projection: "task_graph".into(),
                    source_ref: pid.clone(),
                    why: "Task Graph node project participates in this work context".into(),
                });
            }
        }
    }
    out
}

fn collect_tasks(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Vec<WorkContextAssociation> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    if let Some(t) = &intelligence.current_task {
        if seen.insert(t.id.to_string()) {
            out.push(WorkContextAssociation {
                id: t.id.to_string(),
                label: t.title.clone(),
                kind: "task".into(),
                source_projection: "intelligence.current_task".into(),
                source_ref: t.id.to_string(),
                why: "Active task participates in this work context".into(),
            });
        }
    }
    for member in session.members.iter().filter(|m| m.kind == SessionMemberKind::Task) {
        if seen.insert(member.source_ref.clone()) {
            out.push(WorkContextAssociation {
                id: member.id.clone(),
                label: member.label.clone(),
                kind: "task".into(),
                source_projection: "session.member".into(),
                source_ref: member.source_ref.clone(),
                why: member.why.clone(),
            });
        }
    }
    out
}

fn collect_apps(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Vec<WorkContextAssociation> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for app in &intelligence.current_applications {
        if seen.insert(app.id.clone()) {
            out.push(WorkContextAssociation {
                id: app.id.clone(),
                label: app.name.clone(),
                kind: "application".into(),
                source_projection: "environment.application".into(),
                source_ref: app.id.clone(),
                why: "Application appears relevant to the current environment".into(),
            });
        }
    }
    for member in session
        .members
        .iter()
        .filter(|m| m.kind == SessionMemberKind::Application)
    {
        if seen.insert(member.source_ref.clone()) {
            out.push(WorkContextAssociation {
                id: member.id.clone(),
                label: member.label.clone(),
                kind: "application".into(),
                source_projection: "session.member".into(),
                source_ref: member.source_ref.clone(),
                why: member.why.clone(),
            });
        }
    }
    out
}

fn collect_purpose(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Option<WorkContextAssociation> {
    if !intelligence.purpose.label.is_empty() {
        return Some(WorkContextAssociation {
            id: format!("purpose:{}", intelligence.purpose.workspace_id),
            label: intelligence.purpose.label.clone(),
            kind: "purpose".into(),
            source_projection: "purpose".into(),
            source_ref: intelligence.purpose.workspace_id.clone(),
            why: "Purpose model explains why this work exists".into(),
        });
    }
    session
        .members
        .iter()
        .find(|m| m.kind == SessionMemberKind::Purpose)
        .map(|m| WorkContextAssociation {
            id: m.id.clone(),
            label: m.label.clone(),
            kind: "purpose".into(),
            source_projection: "session.member".into(),
            source_ref: m.source_ref.clone(),
            why: m.why.clone(),
        })
}

fn collect_decisions(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Vec<WorkContextAssociation> {
    let mut out: Vec<_> = intelligence
        .decision_queue
        .items
        .iter()
        .take(5)
        .map(|item| WorkContextAssociation {
            id: item.id.to_string(),
            label: item.title.clone(),
            kind: "decision".into(),
            source_projection: "decision_queue".into(),
            source_ref: item.id.to_string(),
            why: "Decision Queue item associated with this work context".into(),
        })
        .collect();
    if out.is_empty() {
        out = session
            .decisions
            .iter()
            .take(5)
            .map(|d| WorkContextAssociation {
                id: d.id.clone(),
                label: d.title.clone(),
                kind: "decision".into(),
                source_projection: "session.decision".into(),
                source_ref: d.id.clone(),
                why: d.why.clone(),
            })
            .collect();
    }
    out
}

fn collect_activity(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Vec<WorkContextAssociation> {
    let mut out: Vec<_> = intelligence
        .activity_graph
        .recent_timeline
        .iter()
        .take(5)
        .map(|a| WorkContextAssociation {
            id: a.id.to_string(),
            label: a.summary.clone(),
            kind: "activity".into(),
            source_projection: "activity_graph".into(),
            source_ref: a.id.to_string(),
            why: "Recent activity associated with this work context".into(),
        })
        .collect();
    if out.is_empty() {
        out = session
            .timeline
            .iter()
            .take(5)
            .map(|t| WorkContextAssociation {
                id: t.id.clone(),
                label: t.summary.clone(),
                kind: "activity".into(),
                source_projection: "session.timeline".into(),
                source_ref: t.id.clone(),
                why: t.why.clone(),
            })
            .collect();
    }
    out
}

fn collect_blocked_reasons(
    intelligence: &WorkspaceIntelligenceState,
    session: &WorkspaceSessionState,
) -> Vec<String> {
    let mut reasons = Vec::new();
    for gap in intelligence.readiness.top_gaps.iter().take(4) {
        reasons.push(format!("{} — {}", gap.title, gap.explanation));
    }
    for risk in session.risks.iter().take(4) {
        reasons.push(format!("{} — {}", risk.title, risk.why));
    }
    reasons
}

fn collect_recent_progress(
    intelligence: &WorkspaceIntelligenceState,
    experience: &WorkspaceExperienceState,
) -> Vec<String> {
    let mut progress = intelligence.purpose.recent_progress.clone();
    if progress.is_empty() {
        progress = intelligence.evolution.top_insights
            .iter()
            .take(3)
            .map(|i| i.title.clone())
            .collect();
    }
    if progress.is_empty() {
        for section in experience.sections.iter() {
            if section.kind == ExperienceSectionKind::RecentProgress {
                for item in section.items.iter().take(3) {
                    progress.push(item.title.clone());
                }
            }
        }
    }
    progress
}

fn build_relationships(
    contexts: &[WorkContext],
    primary_id: Option<&str>,
) -> Vec<WorkContextRelationship> {
    let mut out = Vec::new();
    let Some(primary_id) = primary_id else {
        return out;
    };
    for ctx in contexts {
        if ctx.id == primary_id {
            out.push(WorkContextRelationship {
                from_context_id: ctx.id.clone(),
                to_context_id: ctx.id.clone(),
                kind: WorkContextRelationKind::Primary,
                why: "Primary context for the current Workspace understanding".into(),
                authority_effect: WorkContext::AUTHORITY_EFFECT_NONE.into(),
            });
            continue;
        }
        let kind = match ctx.current_status {
            WorkContextStatus::Dormant => WorkContextRelationKind::Dormant,
            WorkContextStatus::Interrupted => WorkContextRelationKind::Interrupted,
            WorkContextStatus::Candidate => WorkContextRelationKind::Candidate,
            WorkContextStatus::Blocked => WorkContextRelationKind::Supporting,
            WorkContextStatus::Active => WorkContextRelationKind::Supporting,
        };
        out.push(WorkContextRelationship {
            from_context_id: primary_id.into(),
            to_context_id: ctx.id.clone(),
            kind,
            why: format!(
                "{} is {} relative to the primary context",
                ctx.name,
                kind.as_str()
            ),
            authority_effect: WorkContext::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    out
}
