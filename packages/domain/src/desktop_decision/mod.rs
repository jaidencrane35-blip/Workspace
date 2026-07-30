//! Deterministic desktop decision support projected onto WorkspaceState.
//!
//! Decisions, recommendations, and consistency issues emerge from behaviour,
//! runtime memory, and semantics only. No LLM, no app-name tables, no store.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::desktop_behaviour::DesktopBehaviourTimeline;
use crate::desktop_grouping::DesktopWindowGroup;
use crate::desktop_runtime_memory::{DesktopObjectMemory, DesktopRuntimeMemory};
use crate::desktop_semantic::{
    DesktopSemanticActivity, DesktopSemanticObject, DesktopSemanticProjection,
    DesktopSemanticRelationship,
};

/// Soft bounds so decision support stays inspectable.
pub const DESKTOP_DECISION_LIMIT: usize = 24;
pub const DESKTOP_RECOMMENDATION_LIMIT: usize = 16;
pub const DESKTOP_CONSISTENCY_ISSUE_LIMIT: usize = 16;

/// One evidence contributor on a decision or consistency issue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopDecisionEvidence {
    /// `behaviour` | `memory` | `semantics` | `session` | `relationship`
    pub plane: String,
    pub reference: String,
    pub detail: String,
}

/// Deterministic runtime conclusion (not an AI suggestion).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopDecision {
    pub id: String,
    /// Decision kind — see constants on [`DesktopDecision`].
    pub kind: String,
    pub summary: String,
    /// Human-inspectable reasoning path.
    pub explanation: String,
    /// `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
    pub evidence_score: i32,
    pub entity_ids: Vec<String>,
    pub evidence: Vec<DesktopDecisionEvidence>,
    pub authority_effect: String,
}

impl DesktopDecision {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const KIND_RESUME_RETURNING_WORK: &'static str = "resume_returning_work";
    pub const KIND_RESUME_INTERRUPTED_WORK: &'static str = "resume_interrupted_work";
    pub const KIND_STABILIZE_WORKING_FOCUS: &'static str = "stabilize_working_focus";
    pub const KIND_KEEP_COMPANION_NEAR: &'static str = "keep_companion_near";
    pub const KIND_SURFACE_ALTERNATING_PAIR: &'static str = "surface_alternating_pair";
    pub const KIND_REDUCE_TASK_SWITCHING: &'static str = "reduce_task_switching";
    pub const KIND_ACKNOWLEDGE_EMERGING: &'static str = "acknowledge_emerging";
    pub const KIND_NOTE_FADING: &'static str = "note_fading";
    pub const KIND_MONITOR_BACKGROUND: &'static str = "monitor_background";
    pub const KIND_ATTEND_CLUSTER: &'static str = "attend_cluster";
    pub const KIND_LOW_CONFIDENCE_KNOWLEDGE: &'static str = "low_confidence_knowledge";
    pub const KIND_INCOMPLETE_DESKTOP: &'static str = "incomplete_desktop";
}

/// Deterministic recommendation derived from a decision (not a command).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopRecommendation {
    pub id: String,
    pub decision_id: String,
    /// Recommendation phrasing kind.
    pub kind: String,
    pub summary: String,
    pub explanation: String,
    pub confidence: String,
    pub entity_ids: Vec<String>,
    pub authority_effect: String,
}

impl DesktopRecommendation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const KIND_RESUME_PREVIOUS_CONTEXT: &'static str = "resume_previous_context";
    pub const KIND_REOPEN_INTERRUPTED_WORK: &'static str = "reopen_interrupted_work";
    pub const KIND_LIKELY_CONTINUATION: &'static str = "likely_continuation";
    pub const KIND_RELATED_BECOMING_ACTIVE: &'static str = "related_becoming_active";
    pub const KIND_DESKTOP_APPEARS_INCOMPLETE: &'static str = "desktop_appears_incomplete";
    pub const KIND_GATHER_MORE_EVIDENCE: &'static str = "gather_more_evidence";
    pub const KIND_STAY_WITH_FOCUS: &'static str = "stay_with_focus";
    pub const KIND_NOTICE_FADING_CONTEXT: &'static str = "notice_fading_context";
}

/// Consistency / uncertainty finding — never fabricates certainty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopConsistencyIssue {
    pub id: String,
    pub kind: String,
    pub summary: String,
    pub explanation: String,
    pub confidence: String,
    pub evidence_score: i32,
    pub entity_ids: Vec<String>,
    pub evidence: Vec<DesktopDecisionEvidence>,
    pub authority_effect: String,
}

impl DesktopConsistencyIssue {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const KIND_ROLE_MEMORY_CONFLICT: &'static str = "role_memory_conflict";
    pub const KIND_WEAK_EVIDENCE: &'static str = "weak_evidence";
    pub const KIND_CONFLICTING_RELATIONSHIP: &'static str = "conflicting_relationship";
    pub const KIND_UNSTABLE_IDENTITY: &'static str = "unstable_identity";
    pub const KIND_CONTRADICTORY_CONFIDENCE: &'static str = "contradictory_confidence";
}

/// Decision support projection on WorkspaceState.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopDecisionProjection {
    pub decisions: Vec<DesktopDecision>,
    pub recommendations: Vec<DesktopRecommendation>,
    pub consistency_issues: Vec<DesktopConsistencyIssue>,
    pub authority_effect: String,
}

impl DesktopDecisionProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            decisions: Vec::new(),
            recommendations: Vec::new(),
            consistency_issues: Vec::new(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Project deterministic decisions, recommendations, and consistency issues.
pub fn project_desktop_decisions(
    behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
    semantics: &DesktopSemanticProjection,
) -> DesktopDecisionProjection {
    let decisions = project_decisions(behaviour, memory, semantics);
    let recommendations = project_recommendations(&decisions);
    let consistency_issues = project_consistency_issues(behaviour, memory, semantics);

    DesktopDecisionProjection {
        decisions,
        recommendations,
        consistency_issues,
        authority_effect: DesktopDecisionProjection::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn project_decisions(
    behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
    semantics: &DesktopSemanticProjection,
) -> Vec<DesktopDecision> {
    let mut decisions = Vec::new();
    let mut seen_kinds_entities: BTreeSet<(String, String)> = BTreeSet::new();

    let returning_session = behaviour
        .sessions
        .iter()
        .find(|session| session.kind == "returning");
    if let Some(session) = returning_session {
        let mut entity_ids = Vec::new();
        if let Some(focus) = &session.dominant_focus {
            if let Some(id) = focus.stable_window_id.clone() {
                entity_ids.push(id);
            }
        }
        for object in semantics.objects.iter().filter(|o| {
            o.role == DesktopSemanticObject::ROLE_RETURNING
                || o.role == DesktopSemanticObject::ROLE_WORKING
        }) {
            if !entity_ids.contains(&object.stable_window_id) {
                entity_ids.push(object.stable_window_id.clone());
            }
        }
        push_decision(
            &mut decisions,
            &mut seen_kinds_entities,
            DesktopDecision::KIND_RESUME_RETURNING_WORK,
            "Returning observation session detected",
            format!(
                "Session {} is kind=returning with {} samples; dominant focus and returning/working roles support resume.",
                session.id, session.sample_count
            ),
            3 + session.sample_count.min(4),
            entity_ids,
            vec![
                evidence("session", &session.id, "session.kind=returning"),
                evidence(
                    "behaviour",
                    "sessions",
                    &format!("focus_transition_count={}", session.focus_transition_count),
                ),
            ],
        );
    }

    for object in &semantics.objects {
        if object.role == DesktopSemanticObject::ROLE_RETURNING {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_RESUME_RETURNING_WORK,
                &format!("Returning work: {}", object.title),
                format!(
                    "Semantic role=returning (importance={}, confidence={}) for {}.",
                    object.importance, object.confidence, object.stable_window_id
                ),
                object.evidence_score.max(3),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "role=returning",
                )],
            );
        }
        if object.role == DesktopSemanticObject::ROLE_INTERRUPTED {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_RESUME_INTERRUPTED_WORK,
                &format!("Interrupted work: {}", object.title),
                format!(
                    "Semantic role=interrupted for {}; memory/knowledge indicates interrupted or fading presence.",
                    object.stable_window_id
                ),
                object.evidence_score.max(3),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "role=interrupted",
                )],
            );
        }
        if object.role == DesktopSemanticObject::ROLE_WORKING
            && (object.importance == DesktopSemanticObject::IMPORTANCE_IMPORTANT
                || object.importance == DesktopSemanticObject::IMPORTANCE_ROUTINE)
        {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_STABILIZE_WORKING_FOCUS,
                &format!("Active working focus: {}", object.title),
                format!(
                    "Role=working with importance={} and evidence_score={}.",
                    object.importance, object.evidence_score
                ),
                object.evidence_score.max(3),
                vec![object.stable_window_id.clone()],
                vec![
                    evidence("semantics", &object.stable_window_id, "role=working"),
                    evidence(
                        "semantics",
                        &object.stable_window_id,
                        &format!("importance={}", object.importance),
                    ),
                ],
            );
        }
        if object.role == DesktopSemanticObject::ROLE_COMPANION {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_KEEP_COMPANION_NEAR,
                &format!("Companion context: {}", object.title),
                format!(
                    "Role=companion; co-presence/supports relationships may keep this near working focus."
                ),
                object.evidence_score.max(2),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "role=companion",
                )],
            );
        }
        if object.role == DesktopSemanticObject::ROLE_ALTERNATING {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_SURFACE_ALTERNATING_PAIR,
                &format!("Alternating focus: {}", object.title),
                format!(
                    "Role=alternating indicates bidirectional focus follows involving {}.",
                    object.stable_window_id
                ),
                object.evidence_score.max(3),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "role=alternating",
                )],
            );
        }
        if object.role == DesktopSemanticObject::ROLE_EMERGING
            || object.importance == DesktopSemanticObject::IMPORTANCE_EMERGING
        {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_ACKNOWLEDGE_EMERGING,
                &format!("Emerging context: {}", object.title),
                format!(
                    "Role/importance emerging for {} (score={}).",
                    object.stable_window_id, object.evidence_score
                ),
                object.evidence_score.max(2),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "emerging",
                )],
            );
        }
        if object.importance == DesktopSemanticObject::IMPORTANCE_FADING {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_NOTE_FADING,
                &format!("Fading context: {}", object.title),
                format!(
                    "Importance=fading for {}; evidence suggests declining continuity.",
                    object.stable_window_id
                ),
                object.evidence_score.max(2),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "importance=fading",
                )],
            );
        }
        if object.role == DesktopSemanticObject::ROLE_CLUSTER {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_ATTEND_CLUSTER,
                &format!("Cluster member: {}", object.title),
                format!(
                    "Role=cluster for {}; belongs to a strengthened group.",
                    object.stable_window_id
                ),
                object.evidence_score.max(2),
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "role=cluster",
                )],
            );
        }
        if object.confidence == DesktopWindowGroup::CONFIDENCE_STRUCTURAL
            && object.evidence_score <= 2
        {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_LOW_CONFIDENCE_KNOWLEDGE,
                &format!("Low-confidence knowledge: {}", object.title),
                format!(
                    "Semantic confidence is structural with evidence_score={} for {}; more observation needed.",
                    object.evidence_score, object.stable_window_id
                ),
                2,
                vec![object.stable_window_id.clone()],
                vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "confidence=structural",
                )],
            );
        }
    }

    for rel in &semantics.relationships {
        if rel.kind == DesktopSemanticRelationship::KIND_FREQUENTLY_ALTERNATES {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_SURFACE_ALTERNATING_PAIR,
                "Frequently alternating pair",
                format!(
                    "Relationship frequently_alternates between {} and {} (evidence={}, sessions={}).",
                    rel.from_stable_window_id,
                    rel.to_stable_window_id,
                    rel.evidence_count,
                    rel.session_count
                ),
                rel.evidence_count.max(3),
                vec![
                    rel.from_stable_window_id.clone(),
                    rel.to_stable_window_id.clone(),
                ],
                vec![evidence(
                    "relationship",
                    &format!(
                        "{}:{}",
                        rel.from_stable_window_id, rel.to_stable_window_id
                    ),
                    "frequently_alternates",
                )],
            );
        }
    }

    for activity in &semantics.activities {
        if activity.kind == DesktopSemanticActivity::KIND_TASK_SWITCHING {
            let transitions = behaviour
                .sessions
                .iter()
                .find(|session| {
                    activity
                        .session_id
                        .as_ref()
                        .is_some_and(|id| id == &session.id)
                })
                .map(|session| session.focus_transition_count)
                .unwrap_or(behaviour.focus_transitions.len() as i32);
            if transitions >= 3 {
                push_decision(
                    &mut decisions,
                    &mut seen_kinds_entities,
                    DesktopDecision::KIND_REDUCE_TASK_SWITCHING,
                    "High task switching detected",
                    format!(
                        "Activity kind=task_switching with ~{} focus transitions in retained samples/session.",
                        transitions
                    ),
                    activity.evidence_score.max(3),
                    activity.member_ids.clone(),
                    vec![
                        evidence("semantics", &activity.id, "kind=task_switching"),
                        evidence(
                            "behaviour",
                            "focus_transitions",
                            &format!("count≈{transitions}"),
                        ),
                    ],
                );
            }
        }
        if activity.kind == DesktopSemanticActivity::KIND_MONITORING {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_MONITOR_BACKGROUND,
                "Monitoring / background activity",
                format!(
                    "Activity kind=monitoring (confidence={}).",
                    activity.confidence
                ),
                activity.evidence_score.max(2),
                activity.member_ids.clone(),
                vec![evidence("semantics", &activity.id, "kind=monitoring")],
            );
        }
        if activity.kind == DesktopSemanticActivity::KIND_INTERRUPTED_WORK {
            push_decision(
                &mut decisions,
                &mut seen_kinds_entities,
                DesktopDecision::KIND_RESUME_INTERRUPTED_WORK,
                "Interrupted activity detected",
                format!(
                    "Activity kind=interrupted_work for session {:?}.",
                    activity.session_id
                ),
                activity.evidence_score.max(3),
                activity.member_ids.clone(),
                vec![evidence(
                    "semantics",
                    &activity.id,
                    "kind=interrupted_work",
                )],
            );
        }
    }

    if memory.absent_count >= 2 && memory.present_count >= 1 {
        let absent_ids: Vec<String> = memory
            .entities
            .iter()
            .filter(|entity| entity.presence == DesktopObjectMemory::PRESENCE_ABSENT)
            .take(6)
            .map(|entity| entity.stable_window_id.clone())
            .collect();
        push_decision(
            &mut decisions,
            &mut seen_kinds_entities,
            DesktopDecision::KIND_INCOMPLETE_DESKTOP,
            "Desktop appears incomplete relative to recent memory",
            format!(
                "Runtime memory has {} absent and {} present known objects in retained history.",
                memory.absent_count, memory.present_count
            ),
            2 + memory.absent_count.min(4),
            absent_ids,
            vec![evidence(
                "memory",
                "runtime_memory",
                &format!(
                    "absent={}, present={}, returning={}",
                    memory.absent_count, memory.present_count, memory.returning_count
                ),
            )],
        );
    }

    decisions.sort_by(|a, b| {
        b.evidence_score
            .cmp(&a.evidence_score)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.id.cmp(&b.id))
    });
    decisions.truncate(DESKTOP_DECISION_LIMIT);
    decisions
}

fn project_recommendations(decisions: &[DesktopDecision]) -> Vec<DesktopRecommendation> {
    let mut recommendations = Vec::new();
    for decision in decisions {
        let mapped = match decision.kind.as_str() {
            DesktopDecision::KIND_RESUME_RETURNING_WORK => Some((
                DesktopRecommendation::KIND_RESUME_PREVIOUS_CONTEXT,
                "Resume previous working context.",
            )),
            DesktopDecision::KIND_RESUME_INTERRUPTED_WORK => Some((
                DesktopRecommendation::KIND_REOPEN_INTERRUPTED_WORK,
                "Reopen interrupted work when ready.",
            )),
            DesktopDecision::KIND_STABILIZE_WORKING_FOCUS => Some((
                DesktopRecommendation::KIND_STAY_WITH_FOCUS,
                "Likely continuation: stay with current working focus.",
            )),
            DesktopDecision::KIND_ACKNOWLEDGE_EMERGING => Some((
                DesktopRecommendation::KIND_RELATED_BECOMING_ACTIVE,
                "Related workspace becoming active.",
            )),
            DesktopDecision::KIND_INCOMPLETE_DESKTOP => Some((
                DesktopRecommendation::KIND_DESKTOP_APPEARS_INCOMPLETE,
                "Desktop appears incomplete relative to recent memory.",
            )),
            DesktopDecision::KIND_LOW_CONFIDENCE_KNOWLEDGE => Some((
                DesktopRecommendation::KIND_GATHER_MORE_EVIDENCE,
                "Low-confidence runtime knowledge — more observation needed.",
            )),
            DesktopDecision::KIND_NOTE_FADING => Some((
                DesktopRecommendation::KIND_NOTICE_FADING_CONTEXT,
                "Recently fading activity detected.",
            )),
            DesktopDecision::KIND_KEEP_COMPANION_NEAR
            | DesktopDecision::KIND_SURFACE_ALTERNATING_PAIR => Some((
                DesktopRecommendation::KIND_LIKELY_CONTINUATION,
                "Likely continuation available from related context.",
            )),
            _ => None,
        };
        let Some((kind, summary)) = mapped else {
            continue;
        };
        recommendations.push(DesktopRecommendation {
            id: format!("recommendation:{}", decision.id),
            decision_id: decision.id.clone(),
            kind: kind.into(),
            summary: summary.into(),
            explanation: decision.explanation.clone(),
            confidence: decision.confidence.clone(),
            entity_ids: decision.entity_ids.clone(),
            authority_effect: DesktopRecommendation::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    recommendations.truncate(DESKTOP_RECOMMENDATION_LIMIT);
    recommendations
}

fn project_consistency_issues(
    _behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
    semantics: &DesktopSemanticProjection,
) -> Vec<DesktopConsistencyIssue> {
    let mut issues = Vec::new();
    let memory_by_id: std::collections::HashMap<&str, &DesktopObjectMemory> = memory
        .entities
        .iter()
        .map(|entity| (entity.stable_window_id.as_str(), entity))
        .collect();

    for object in &semantics.objects {
        let Some(entity) = memory_by_id.get(object.stable_window_id.as_str()) else {
            continue;
        };
        if object.role == DesktopSemanticObject::ROLE_WORKING
            && (entity.knowledge == DesktopObjectMemory::KNOWLEDGE_TEMPORARY
                || entity.focus_count == 0)
        {
            issues.push(DesktopConsistencyIssue {
                id: format!("consistency:role_memory:{}", object.stable_window_id),
                kind: DesktopConsistencyIssue::KIND_ROLE_MEMORY_CONFLICT.into(),
                summary: format!("Working role conflicts with memory for {}", object.title),
                explanation: format!(
                    "Semantics role=working but memory knowledge={} focus_count={}.",
                    entity.knowledge, entity.focus_count
                ),
                confidence: DesktopWindowGroup::confidence_for_evidence(3).into(),
                evidence_score: 3,
                entity_ids: vec![object.stable_window_id.clone()],
                evidence: vec![
                    evidence("semantics", &object.stable_window_id, "role=working"),
                    evidence(
                        "memory",
                        &object.stable_window_id,
                        &format!(
                            "knowledge={}, focus_count={}",
                            entity.knowledge, entity.focus_count
                        ),
                    ),
                ],
                authority_effect: DesktopConsistencyIssue::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if object.confidence == DesktopWindowGroup::CONFIDENCE_STRONG
            && entity.continuity_confidence == DesktopWindowGroup::CONFIDENCE_STRUCTURAL
        {
            issues.push(DesktopConsistencyIssue {
                id: format!("consistency:confidence:{}", object.stable_window_id),
                kind: DesktopConsistencyIssue::KIND_CONTRADICTORY_CONFIDENCE.into(),
                summary: format!("Contradictory confidence for {}", object.title),
                explanation: format!(
                    "Semantic confidence={} while continuity_confidence={}.",
                    object.confidence, entity.continuity_confidence
                ),
                confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                evidence_score: 2,
                entity_ids: vec![object.stable_window_id.clone()],
                evidence: vec![
                    evidence(
                        "semantics",
                        &object.stable_window_id,
                        &format!("confidence={}", object.confidence),
                    ),
                    evidence(
                        "memory",
                        &object.stable_window_id,
                        &format!("continuity_confidence={}", entity.continuity_confidence),
                    ),
                ],
                authority_effect: DesktopConsistencyIssue::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if entity.identity_confidence == "ephemeral" || entity.identity_confidence == "low" {
            issues.push(DesktopConsistencyIssue {
                id: format!("consistency:identity:{}", object.stable_window_id),
                kind: DesktopConsistencyIssue::KIND_UNSTABLE_IDENTITY.into(),
                summary: format!("Unstable identity for {}", object.title),
                explanation: format!(
                    "Identity confidence={} for {}.",
                    entity.identity_confidence, object.stable_window_id
                ),
                confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                evidence_score: 2,
                entity_ids: vec![object.stable_window_id.clone()],
                evidence: vec![evidence(
                    "memory",
                    &object.stable_window_id,
                    &format!("identity_confidence={}", entity.identity_confidence),
                )],
                authority_effect: DesktopConsistencyIssue::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        if object.evidence_score <= 1
            && object.confidence == DesktopWindowGroup::CONFIDENCE_STRUCTURAL
        {
            issues.push(DesktopConsistencyIssue {
                id: format!("consistency:weak:{}", object.stable_window_id),
                kind: DesktopConsistencyIssue::KIND_WEAK_EVIDENCE.into(),
                summary: format!("Weak semantic evidence for {}", object.title),
                explanation: format!(
                    "evidence_score={} with structural confidence.",
                    object.evidence_score
                ),
                confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
                evidence_score: 1,
                entity_ids: vec![object.stable_window_id.clone()],
                evidence: vec![evidence(
                    "semantics",
                    &object.stable_window_id,
                    "weak_evidence",
                )],
                authority_effect: DesktopConsistencyIssue::AUTHORITY_EFFECT_NONE.into(),
            });
        }
    }

    // Conflicting relationship kinds between same pair (works_with vs frequently_alternates is OK;
    // precedes and follows both directions already collapsed — flag supports + fading importance).
    for rel in &semantics.relationships {
        if rel.kind == DesktopSemanticRelationship::KIND_SUPPORTS {
            let fading_target = semantics.objects.iter().any(|object| {
                object.stable_window_id == rel.to_stable_window_id
                    && object.importance == DesktopSemanticObject::IMPORTANCE_FADING
            });
            if fading_target {
                issues.push(DesktopConsistencyIssue {
                    id: format!(
                        "consistency:rel:{}:{}",
                        rel.from_stable_window_id, rel.to_stable_window_id
                    ),
                    kind: DesktopConsistencyIssue::KIND_CONFLICTING_RELATIONSHIP.into(),
                    summary: "Supports relationship points at fading context".into(),
                    explanation: format!(
                        "{} supports {} but target importance=fading.",
                        rel.from_stable_window_id, rel.to_stable_window_id
                    ),
                    confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                    evidence_score: 2,
                    entity_ids: vec![
                        rel.from_stable_window_id.clone(),
                        rel.to_stable_window_id.clone(),
                    ],
                    evidence: vec![evidence(
                        "relationship",
                        &format!(
                            "{}:{}",
                            rel.from_stable_window_id, rel.to_stable_window_id
                        ),
                        "supports→fading",
                    )],
                    authority_effect: DesktopConsistencyIssue::AUTHORITY_EFFECT_NONE.into(),
                });
            }
        }
    }

    issues.sort_by(|a, b| {
        b.evidence_score
            .cmp(&a.evidence_score)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.id.cmp(&b.id))
    });
    issues.truncate(DESKTOP_CONSISTENCY_ISSUE_LIMIT);
    issues
}

fn push_decision(
    decisions: &mut Vec<DesktopDecision>,
    seen: &mut BTreeSet<(String, String)>,
    kind: &str,
    summary: &str,
    explanation: String,
    score: i32,
    entity_ids: Vec<String>,
    evidence_rows: Vec<DesktopDecisionEvidence>,
) {
    let key_entity = entity_ids.first().cloned().unwrap_or_else(|| "*".into());
    let dedupe_key = (kind.to_string(), key_entity);
    if !seen.insert(dedupe_key) {
        return;
    }
    let id = format!(
        "decision:{}:{}",
        kind,
        entity_ids
            .first()
            .map(String::as_str)
            .unwrap_or("desktop")
    );
    decisions.push(DesktopDecision {
        id,
        kind: kind.into(),
        summary: summary.into(),
        explanation,
        confidence: DesktopWindowGroup::confidence_for_evidence(score).into(),
        evidence_score: score,
        entity_ids,
        evidence: evidence_rows,
        authority_effect: DesktopDecision::AUTHORITY_EFFECT_NONE.into(),
    });
}

fn evidence(plane: &str, reference: &str, detail: &str) -> DesktopDecisionEvidence {
    DesktopDecisionEvidence {
        plane: plane.into(),
        reference: reference.into(),
        detail: detail.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop_behaviour::project_desktop_behaviour;
    use crate::desktop_runtime_memory::project_desktop_runtime_memory;
    use crate::desktop_semantic::project_desktop_semantics;
    use crate::workspace_observation::{
        ObservedMonitor, ObservedWindow, ObservationWindowIdentity, WindowIdentityConfidence,
        WorkspaceObservationPass, WorkspaceObservationSnapshot,
    };

    fn snap(
        pass_id: &str,
        captured_at: &str,
        windows: Vec<(&str, &str, bool)>,
    ) -> WorkspaceObservationSnapshot {
        let mon_id = format!("{pass_id}-mon");
        WorkspaceObservationSnapshot {
            pass: WorkspaceObservationPass {
                id: pass_id.into(),
                captured_at: captured_at.into(),
                schema_version: 1,
                source: "test".into(),
                foreground_hwnd: windows
                    .iter()
                    .find(|(_, _, focused)| *focused)
                    .map(|(_, hwnd, _)| (*hwnd).into()),
                window_count: windows.len() as i32,
                monitor_count: 1,
                duration_ms: Some(1),
                metadata_json: "{}".into(),
                authority_effect: WorkspaceObservationPass::AUTHORITY_EFFECT_NONE.into(),
            },
            monitors: vec![ObservedMonitor {
                id: mon_id.clone(),
                pass_id: pass_id.into(),
                monitor_index: 0,
                name: "Primary".into(),
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
                work_x: 0,
                work_y: 0,
                work_w: 1920,
                work_h: 1040,
                is_primary: true,
                dpi_scale: None,
                authority_effect: ObservedMonitor::AUTHORITY_EFFECT_NONE.into(),
            }],
            windows: windows
                .into_iter()
                .enumerate()
                .map(|(index, (stable, hwnd, focused))| ObservedWindow {
                    id: format!("{pass_id}-{stable}"),
                    pass_id: pass_id.into(),
                    hwnd: hwnd.into(),
                    stable_window_id: Some(stable.into()),
                    title: format!("Window {stable}"),
                    process_id: 100 + index as i32,
                    process_name: Some("app.exe".into()),
                    x: 0,
                    y: 0,
                    width: 800,
                    height: 600,
                    monitor_id: Some(mon_id.clone()),
                    visible: true,
                    minimized: false,
                    focused,
                    z_order: Some(index as i32),
                    authority_effect: ObservedWindow::AUTHORITY_EFFECT_NONE.into(),
                })
                .collect(),
            identities: Vec::new(),
            authority_effect: WorkspaceObservationSnapshot::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    #[test]
    fn projects_decisions_recommendations_and_explanations() {
        let history = vec![
            snap(
                "p1",
                "2026-07-30T10:00:00Z",
                vec![("stable-a", "0x1", true), ("stable-b", "0x2", false)],
            ),
            snap(
                "p2",
                "2026-07-30T10:01:00Z",
                vec![("stable-a", "0x1", true), ("stable-b", "0x2", false)],
            ),
            snap(
                "p3",
                "2026-07-30T10:02:00Z",
                vec![("stable-a", "0x1", false), ("stable-b", "0x2", true)],
            ),
            snap(
                "p4",
                "2026-07-30T10:03:00Z",
                vec![("stable-a", "0x1", true), ("stable-b", "0x2", false)],
            ),
        ];
        let identities = vec![
            ObservationWindowIdentity {
                id: "stable-a".into(),
                process_id: 100,
                title_fingerprint: "Window stable-a".into(),
                first_seen_at: "2026-07-30T10:00:00Z".into(),
                last_seen_at: "2026-07-30T10:03:00Z".into(),
                last_hwnd: "0x1".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            },
            ObservationWindowIdentity {
                id: "stable-b".into(),
                process_id: 101,
                title_fingerprint: "Window stable-b".into(),
                first_seen_at: "2026-07-30T10:00:00Z".into(),
                last_seen_at: "2026-07-30T10:03:00Z".into(),
                last_hwnd: "0x2".into(),
                confidence: WindowIdentityConfidence::High,
                authority_effect: ObservationWindowIdentity::AUTHORITY_EFFECT_NONE.into(),
            },
        ];
        let behaviour = project_desktop_behaviour(&history);
        let memory = project_desktop_runtime_memory(&history, &identities, &behaviour);
        let semantics = project_desktop_semantics(&behaviour, &memory, &[]);
        let decisions = project_desktop_decisions(&behaviour, &memory, &semantics);

        assert!(!decisions.decisions.is_empty());
        assert!(decisions.decisions.iter().all(|d| !d.explanation.is_empty()));
        assert!(decisions.decisions.iter().all(|d| !d.evidence.is_empty()));
        assert!(!decisions.recommendations.is_empty());
        assert!(decisions
            .recommendations
            .iter()
            .all(|r| decisions.decisions.iter().any(|d| d.id == r.decision_id)));
    }

    #[test]
    fn detects_returning_decision_from_memory_role() {
        let mut memory = DesktopRuntimeMemory::empty();
        memory.entities.push(DesktopObjectMemory {
            stable_window_id: "stable-r".into(),
            hwnd: "0xr".into(),
            title: "Returning".into(),
            process_id: 1,
            process_name: None,
            first_observed_at: "2026-07-30T09:00:00Z".into(),
            last_observed_at: "2026-07-30T11:00:00Z".into(),
            identity_confidence: "high".into(),
            presence: DesktopObjectMemory::PRESENCE_RETURNING.into(),
            sample_presence_count: 3,
            session_presence_count: 2,
            focus_count: 2,
            opened_count: 1,
            closed_count: 1,
            recurrence_count: 2,
            stability: DesktopObjectMemory::STABILITY_INTERMITTENT.into(),
            continuity_confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
            knowledge: DesktopObjectMemory::KNOWLEDGE_RETURNING.into(),
            authority_effect: DesktopObjectMemory::AUTHORITY_EFFECT_NONE.into(),
        });
        let behaviour = DesktopBehaviourTimeline::empty();
        let semantics = project_desktop_semantics(&behaviour, &memory, &[]);
        let decisions = project_desktop_decisions(&behaviour, &memory, &semantics);
        assert!(decisions.decisions.iter().any(|d| {
            d.kind == DesktopDecision::KIND_RESUME_RETURNING_WORK
        }));
        assert!(decisions.recommendations.iter().any(|r| {
            r.kind == DesktopRecommendation::KIND_RESUME_PREVIOUS_CONTEXT
        }));
    }
}
