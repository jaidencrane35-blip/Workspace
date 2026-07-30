//! Deterministic desktop attention projected onto WorkspaceState.
//!
//! Attention estimates what deserves notice now from decisions, semantics,
//! memory, and behaviour. Not Programme IV attention. No LLM, no app tables, no store.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::desktop_behaviour::DesktopBehaviourTimeline;
use crate::desktop_decision::{
    DesktopConsistencyIssue, DesktopDecision, DesktopDecisionEvidence, DesktopDecisionProjection,
};
use crate::desktop_grouping::DesktopWindowGroup;
use crate::desktop_runtime_memory::DesktopRuntimeMemory;
use crate::desktop_semantic::DesktopSemanticProjection;

/// Soft bound for attention items.
pub const DESKTOP_ATTENTION_LIMIT: usize = 16;

/// One deterministic attention item for the current projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopAttentionItem {
    pub id: String,
    /// Attention kind — see constants on [`DesktopAttentionItem`].
    pub kind: String,
    pub summary: String,
    /// Human-inspectable reasoning path.
    pub explanation: String,
    /// `emerged` | `strengthening` | `stable` | `decaying` | `resolved`
    pub lifecycle: String,
    /// Relative attention strength from evidence (higher = more now).
    pub strength: i32,
    /// `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
    /// `immediate` | `near_term` | `background`
    pub time_sensitivity: String,
    pub entity_ids: Vec<String>,
    pub evidence: Vec<DesktopDecisionEvidence>,
    pub supporting_planes: Vec<String>,
    pub source_decision_id: Option<String>,
    pub authority_effect: String,
}

impl DesktopAttentionItem {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub const KIND_INTERRUPTED_WORK: &'static str = "interrupted_work";
    pub const KIND_RETURNING_WORK: &'static str = "returning_work";
    pub const KIND_WORKING_FOCUS: &'static str = "working_focus";
    pub const KIND_EMERGING_CONTEXT: &'static str = "emerging_context";
    pub const KIND_FADING_CONTEXT: &'static str = "fading_context";
    pub const KIND_TASK_SWITCHING: &'static str = "task_switching";
    pub const KIND_COMPANION_CONTEXT: &'static str = "companion_context";
    pub const KIND_CLUSTER_CONTEXT: &'static str = "cluster_context";
    pub const KIND_BACKGROUND_MONITOR: &'static str = "background_monitor";
    pub const KIND_INCOMPLETE_DESKTOP: &'static str = "incomplete_desktop";
    pub const KIND_WEAK_EVIDENCE: &'static str = "weak_evidence";
    pub const KIND_UNCERTAIN_CONTEXT: &'static str = "uncertain_context";

    pub const LIFECYCLE_EMERGED: &'static str = "emerged";
    pub const LIFECYCLE_STRENGTHENING: &'static str = "strengthening";
    pub const LIFECYCLE_STABLE: &'static str = "stable";
    pub const LIFECYCLE_DECAYING: &'static str = "decaying";
    pub const LIFECYCLE_RESOLVED: &'static str = "resolved";

    pub const TIME_IMMEDIATE: &'static str = "immediate";
    pub const TIME_NEAR_TERM: &'static str = "near_term";
    pub const TIME_BACKGROUND: &'static str = "background";
}

/// Attention projection on WorkspaceState.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopAttentionProjection {
    pub items: Vec<DesktopAttentionItem>,
    pub authority_effect: String,
}

impl DesktopAttentionProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Project deterministic attention from decision support + evidence planes.
pub fn project_desktop_attention(
    decisions: &DesktopDecisionProjection,
    semantics: &DesktopSemanticProjection,
    memory: &DesktopRuntimeMemory,
    behaviour: &DesktopBehaviourTimeline,
) -> DesktopAttentionProjection {
    let mut items = Vec::new();
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();

    for decision in &decisions.decisions {
        if let Some(item) = attention_from_decision(decision) {
            push_item(&mut items, &mut seen, item);
        }
    }

    for issue in &decisions.consistency_issues {
        if let Some(item) = attention_from_consistency(issue) {
            push_item(&mut items, &mut seen, item);
        }
    }

    // Gap-fill only when no covering attention exists for the entity/kind family.
    gap_fill_from_planes(&mut items, &mut seen, semantics, memory, behaviour);

    let mut projection = DesktopAttentionProjection {
        items,
        authority_effect: DesktopAttentionProjection::AUTHORITY_EFFECT_NONE.into(),
    };
    refine_attention(&mut projection);
    projection
}

fn attention_from_decision(decision: &DesktopDecision) -> Option<DesktopAttentionItem> {
    let (kind, time_sensitivity) = match decision.kind.as_str() {
        DesktopDecision::KIND_RESUME_INTERRUPTED_WORK => {
            (DesktopAttentionItem::KIND_INTERRUPTED_WORK, DesktopAttentionItem::TIME_IMMEDIATE)
        }
        DesktopDecision::KIND_RESUME_RETURNING_WORK => {
            (DesktopAttentionItem::KIND_RETURNING_WORK, DesktopAttentionItem::TIME_IMMEDIATE)
        }
        DesktopDecision::KIND_STABILIZE_WORKING_FOCUS => {
            (DesktopAttentionItem::KIND_WORKING_FOCUS, DesktopAttentionItem::TIME_NEAR_TERM)
        }
        DesktopDecision::KIND_ACKNOWLEDGE_EMERGING => {
            (DesktopAttentionItem::KIND_EMERGING_CONTEXT, DesktopAttentionItem::TIME_NEAR_TERM)
        }
        DesktopDecision::KIND_NOTE_FADING => {
            (DesktopAttentionItem::KIND_FADING_CONTEXT, DesktopAttentionItem::TIME_BACKGROUND)
        }
        DesktopDecision::KIND_REDUCE_TASK_SWITCHING
        | DesktopDecision::KIND_SURFACE_ALTERNATING_PAIR => {
            (DesktopAttentionItem::KIND_TASK_SWITCHING, DesktopAttentionItem::TIME_NEAR_TERM)
        }
        DesktopDecision::KIND_KEEP_COMPANION_NEAR => {
            (DesktopAttentionItem::KIND_COMPANION_CONTEXT, DesktopAttentionItem::TIME_BACKGROUND)
        }
        DesktopDecision::KIND_ATTEND_CLUSTER => {
            (DesktopAttentionItem::KIND_CLUSTER_CONTEXT, DesktopAttentionItem::TIME_BACKGROUND)
        }
        DesktopDecision::KIND_MONITOR_BACKGROUND => {
            (DesktopAttentionItem::KIND_BACKGROUND_MONITOR, DesktopAttentionItem::TIME_BACKGROUND)
        }
        DesktopDecision::KIND_INCOMPLETE_DESKTOP => {
            (DesktopAttentionItem::KIND_INCOMPLETE_DESKTOP, DesktopAttentionItem::TIME_NEAR_TERM)
        }
        DesktopDecision::KIND_LOW_CONFIDENCE_KNOWLEDGE => {
            (DesktopAttentionItem::KIND_WEAK_EVIDENCE, DesktopAttentionItem::TIME_BACKGROUND)
        }
        _ => return None,
    };

    let lifecycle = lifecycle_for(decision.evidence_score, &decision.supporting_planes, kind);
    Some(DesktopAttentionItem {
        id: format!("attention:{}:{}", kind, decision.id),
        kind: kind.into(),
        summary: decision.summary.clone(),
        explanation: format!(
            "Attention from decision {} ({}). {}",
            decision.kind, decision.id, decision.explanation
        ),
        lifecycle: lifecycle.into(),
        strength: decision.evidence_score.saturating_add(time_boost(time_sensitivity)),
        confidence: decision.confidence.clone(),
        time_sensitivity: time_sensitivity.into(),
        entity_ids: decision.entity_ids.clone(),
        evidence: decision.evidence.clone(),
        supporting_planes: decision.supporting_planes.clone(),
        source_decision_id: Some(decision.id.clone()),
        authority_effect: DesktopAttentionItem::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn attention_from_consistency(issue: &DesktopConsistencyIssue) -> Option<DesktopAttentionItem> {
    let kind = match issue.kind.as_str() {
        DesktopConsistencyIssue::KIND_ROLE_MEMORY_CONFLICT
        | DesktopConsistencyIssue::KIND_CONTRADICTORY_CONFIDENCE
        | DesktopConsistencyIssue::KIND_CONFLICTING_RELATIONSHIP => {
            DesktopAttentionItem::KIND_UNCERTAIN_CONTEXT
        }
        DesktopConsistencyIssue::KIND_WEAK_EVIDENCE
        | DesktopConsistencyIssue::KIND_UNSTABLE_IDENTITY => {
            DesktopAttentionItem::KIND_WEAK_EVIDENCE
        }
        _ => return None,
    };
    let time_sensitivity = if kind == DesktopAttentionItem::KIND_UNCERTAIN_CONTEXT {
        DesktopAttentionItem::TIME_NEAR_TERM
    } else {
        DesktopAttentionItem::TIME_BACKGROUND
    };
    Some(DesktopAttentionItem {
        id: format!("attention:{}:{}", kind, issue.id),
        kind: kind.into(),
        summary: issue.summary.clone(),
        explanation: format!(
            "Attention from consistency {} ({}). {}",
            issue.kind, issue.id, issue.explanation
        ),
        lifecycle: DesktopAttentionItem::LIFECYCLE_EMERGED.into(),
        strength: issue.evidence_score.saturating_add(time_boost(time_sensitivity)),
        confidence: issue.confidence.clone(),
        time_sensitivity: time_sensitivity.into(),
        entity_ids: issue.entity_ids.clone(),
        evidence: issue.evidence.clone(),
        supporting_planes: planes_from_evidence(&issue.evidence),
        source_decision_id: None,
        authority_effect: DesktopAttentionItem::AUTHORITY_EFFECT_NONE.into(),
    })
}

fn gap_fill_from_planes(
    items: &mut Vec<DesktopAttentionItem>,
    seen: &mut BTreeSet<(String, String)>,
    semantics: &DesktopSemanticProjection,
    memory: &DesktopRuntimeMemory,
    behaviour: &DesktopBehaviourTimeline,
) {
    let covered_kinds: BTreeSet<String> = items.iter().map(|item| item.kind.clone()).collect();

    if !covered_kinds.contains(DesktopAttentionItem::KIND_RETURNING_WORK) {
        if behaviour.sessions.iter().any(|session| session.kind == "returning") {
            let entity_ids: Vec<String> = semantics
                .objects
                .iter()
                .filter(|object| object.role == "returning" || object.role == "working")
                .map(|object| object.stable_window_id.clone())
                .take(4)
                .collect();
            push_item(
                items,
                seen,
                DesktopAttentionItem {
                    id: "attention:returning_work:session".into(),
                    kind: DesktopAttentionItem::KIND_RETURNING_WORK.into(),
                    summary: "Returning observation session deserves attention".into(),
                    explanation: "Behaviour session kind=returning with no covering decision."
                        .into(),
                    lifecycle: DesktopAttentionItem::LIFECYCLE_EMERGED.into(),
                    strength: 4,
                    confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                    time_sensitivity: DesktopAttentionItem::TIME_IMMEDIATE.into(),
                    entity_ids,
                    evidence: vec![DesktopDecisionEvidence {
                        plane: "session".into(),
                        reference: "sessions".into(),
                        detail: "kind=returning".into(),
                    }],
                    supporting_planes: vec!["session".into()],
                    source_decision_id: None,
                    authority_effect: DesktopAttentionItem::AUTHORITY_EFFECT_NONE.into(),
                },
            );
        }
    }

    if !covered_kinds.contains(DesktopAttentionItem::KIND_INCOMPLETE_DESKTOP)
        && memory.absent_count >= 2
    {
        let entity_ids: Vec<String> = memory
            .entities
            .iter()
            .filter(|entity| entity.presence == "absent")
            .map(|entity| entity.stable_window_id.clone())
            .take(6)
            .collect();
        push_item(
            items,
            seen,
            DesktopAttentionItem {
                id: "attention:incomplete_desktop:memory".into(),
                kind: DesktopAttentionItem::KIND_INCOMPLETE_DESKTOP.into(),
                summary: "Absent known objects relative to recent memory".into(),
                explanation: format!(
                    "Runtime memory absent_count={} with no covering incomplete_desktop decision.",
                    memory.absent_count
                ),
                lifecycle: DesktopAttentionItem::LIFECYCLE_STRENGTHENING.into(),
                strength: 2 + memory.absent_count.min(4),
                confidence: DesktopWindowGroup::confidence_for_evidence(
                    2 + memory.absent_count.min(4),
                )
                .into(),
                time_sensitivity: DesktopAttentionItem::TIME_NEAR_TERM.into(),
                entity_ids,
                evidence: vec![DesktopDecisionEvidence {
                    plane: "memory".into(),
                    reference: "runtime_memory".into(),
                    detail: format!("absent={}", memory.absent_count),
                }],
                supporting_planes: vec!["memory".into()],
                source_decision_id: None,
                authority_effect: DesktopAttentionItem::AUTHORITY_EFFECT_NONE.into(),
            },
        );
    }
}

fn refine_attention(projection: &mut DesktopAttentionProjection) {
    // Merge equivalent kinds with overlapping entities.
    let mut merged: Vec<DesktopAttentionItem> = Vec::new();
    for item in projection.items.drain(..) {
        if let Some(existing) = merged.iter_mut().find(|other| {
            other.kind == item.kind
                && (other.entity_ids.is_empty()
                    || item.entity_ids.is_empty()
                    || other
                        .entity_ids
                        .iter()
                        .any(|id| item.entity_ids.iter().any(|other_id| other_id == id)))
        }) {
            existing.strength = existing.strength.max(item.strength).saturating_add(1);
            for id in item.entity_ids {
                if !existing.entity_ids.contains(&id) {
                    existing.entity_ids.push(id);
                }
            }
            for row in item.evidence {
                if !existing.evidence.iter().any(|e| {
                    e.plane == row.plane && e.reference == row.reference && e.detail == row.detail
                }) {
                    existing.evidence.push(row);
                }
            }
            existing.supporting_planes = planes_from_evidence(&existing.evidence);
            existing.explanation =
                format!("{} Also: {}", existing.explanation, item.explanation);
            if existing.source_decision_id.is_none() {
                existing.source_decision_id = item.source_decision_id;
            }
            existing.lifecycle = lifecycle_for(
                existing.strength,
                &existing.supporting_planes,
                existing.kind.as_str(),
            )
            .into();
            existing.confidence =
                DesktopWindowGroup::confidence_for_evidence(existing.strength).into();
            // Prefer more urgent time sensitivity.
            if time_rank(item.time_sensitivity.as_str())
                < time_rank(existing.time_sensitivity.as_str())
            {
                existing.time_sensitivity = item.time_sensitivity;
            }
        } else {
            merged.push(item);
        }
    }

    // Suppress weak evidence when stronger attention covers the same entities.
    let strong_entities: BTreeSet<String> = merged
        .iter()
        .filter(|item| {
            matches!(
                item.kind.as_str(),
                DesktopAttentionItem::KIND_INTERRUPTED_WORK
                    | DesktopAttentionItem::KIND_RETURNING_WORK
                    | DesktopAttentionItem::KIND_WORKING_FOCUS
                    | DesktopAttentionItem::KIND_INCOMPLETE_DESKTOP
            ) && item.strength >= 3
        })
        .flat_map(|item| item.entity_ids.iter().cloned())
        .collect();
    merged.retain(|item| {
        if item.kind == DesktopAttentionItem::KIND_WEAK_EVIDENCE
            || item.kind == DesktopAttentionItem::KIND_BACKGROUND_MONITOR
        {
            !item
                .entity_ids
                .iter()
                .any(|id| strong_entities.contains(id))
        } else {
            true
        }
    });

    // Drop resolved lifecycle (should not accumulate).
    merged.retain(|item| item.lifecycle != DesktopAttentionItem::LIFECYCLE_RESOLVED);

    // Detect contradictory attention: working_focus vs interrupted on same entity → keep interrupted.
    let interrupted_entities: BTreeSet<String> = merged
        .iter()
        .filter(|item| item.kind == DesktopAttentionItem::KIND_INTERRUPTED_WORK)
        .flat_map(|item| item.entity_ids.iter().cloned())
        .collect();
    merged.retain(|item| {
        if item.kind == DesktopAttentionItem::KIND_WORKING_FOCUS {
            !item
                .entity_ids
                .iter()
                .any(|id| interrupted_entities.contains(id))
        } else {
            true
        }
    });

    for item in &mut merged {
        if item.supporting_planes.len() >= 2 {
            item.strength = item.strength.saturating_add(1);
            item.explanation = format!(
                "{} Supported by planes: {}.",
                item.explanation,
                item.supporting_planes.join(", ")
            );
            item.lifecycle =
                lifecycle_for(item.strength, &item.supporting_planes, item.kind.as_str()).into();
            item.confidence =
                DesktopWindowGroup::confidence_for_evidence(item.strength).into();
        }
    }

    merged.sort_by(|a, b| {
        time_rank(&a.time_sensitivity)
            .cmp(&time_rank(&b.time_sensitivity))
            .then_with(|| kind_priority(&a.kind).cmp(&kind_priority(&b.kind)))
            .then_with(|| b.strength.cmp(&a.strength))
            .then_with(|| a.id.cmp(&b.id))
    });
    merged.truncate(DESKTOP_ATTENTION_LIMIT);
    projection.items = merged;
}

fn lifecycle_for(strength: i32, planes: &[String], kind: &str) -> &'static str {
    if kind == DesktopAttentionItem::KIND_FADING_CONTEXT {
        return DesktopAttentionItem::LIFECYCLE_DECAYING;
    }
    if planes.len() >= 2 && strength >= 5 {
        DesktopAttentionItem::LIFECYCLE_STABLE
    } else if planes.len() >= 2 || strength >= 4 {
        DesktopAttentionItem::LIFECYCLE_STRENGTHENING
    } else if strength <= 1 {
        DesktopAttentionItem::LIFECYCLE_DECAYING
    } else {
        DesktopAttentionItem::LIFECYCLE_EMERGED
    }
}

fn time_boost(time_sensitivity: &str) -> i32 {
    match time_sensitivity {
        DesktopAttentionItem::TIME_IMMEDIATE => 2,
        DesktopAttentionItem::TIME_NEAR_TERM => 1,
        _ => 0,
    }
}

fn time_rank(time_sensitivity: &str) -> i32 {
    match time_sensitivity {
        DesktopAttentionItem::TIME_IMMEDIATE => 0,
        DesktopAttentionItem::TIME_NEAR_TERM => 1,
        _ => 2,
    }
}

fn kind_priority(kind: &str) -> i32 {
    match kind {
        DesktopAttentionItem::KIND_INTERRUPTED_WORK => 0,
        DesktopAttentionItem::KIND_RETURNING_WORK => 1,
        DesktopAttentionItem::KIND_UNCERTAIN_CONTEXT => 2,
        DesktopAttentionItem::KIND_INCOMPLETE_DESKTOP => 3,
        DesktopAttentionItem::KIND_TASK_SWITCHING => 4,
        DesktopAttentionItem::KIND_WORKING_FOCUS => 5,
        DesktopAttentionItem::KIND_EMERGING_CONTEXT => 6,
        DesktopAttentionItem::KIND_FADING_CONTEXT => 7,
        DesktopAttentionItem::KIND_COMPANION_CONTEXT => 8,
        DesktopAttentionItem::KIND_CLUSTER_CONTEXT => 9,
        DesktopAttentionItem::KIND_BACKGROUND_MONITOR => 10,
        DesktopAttentionItem::KIND_WEAK_EVIDENCE => 11,
        _ => 12,
    }
}

fn push_item(
    items: &mut Vec<DesktopAttentionItem>,
    seen: &mut BTreeSet<(String, String)>,
    item: DesktopAttentionItem,
) {
    let key_entity = item
        .entity_ids
        .first()
        .cloned()
        .unwrap_or_else(|| "*".into());
    if !seen.insert((item.kind.clone(), key_entity)) {
        return;
    }
    items.push(item);
}

fn planes_from_evidence(evidence: &[DesktopDecisionEvidence]) -> Vec<String> {
    let mut planes = BTreeSet::new();
    for row in evidence {
        planes.insert(row.plane.clone());
    }
    planes.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop_behaviour::project_desktop_behaviour;
    use crate::desktop_decision::project_desktop_decisions;
    use crate::desktop_runtime_memory::{
        project_desktop_runtime_memory, DesktopObjectMemory,
    };
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
    fn projects_attention_from_decisions_with_lifecycle() {
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
        let attention = project_desktop_attention(&decisions, &semantics, &memory, &behaviour);

        assert!(!attention.items.is_empty());
        assert!(attention.items.iter().all(|item| {
            !item.explanation.is_empty()
                && !item.evidence.is_empty()
                && item.lifecycle != DesktopAttentionItem::LIFECYCLE_RESOLVED
        }));
        assert!(attention.items.len() <= DESKTOP_ATTENTION_LIMIT);
    }

    #[test]
    fn prefers_interrupted_over_working_on_same_entity() {
        let mut decisions = DesktopDecisionProjection::empty();
        decisions.decisions = vec![
            DesktopDecision {
                id: "decision:stabilize_working_focus:x".into(),
                kind: DesktopDecision::KIND_STABILIZE_WORKING_FOCUS.into(),
                summary: "Working".into(),
                explanation: "working".into(),
                confidence: DesktopWindowGroup::CONFIDENCE_RECURRING.into(),
                evidence_score: 4,
                entity_ids: vec!["stable-x".into()],
                evidence: vec![DesktopDecisionEvidence {
                    plane: "semantics".into(),
                    reference: "stable-x".into(),
                    detail: "role=working".into(),
                }],
                supporting_planes: vec!["semantics".into()],
                authority_effect: DesktopDecision::AUTHORITY_EFFECT_NONE.into(),
            },
            DesktopDecision {
                id: "decision:resume_interrupted_work:x".into(),
                kind: DesktopDecision::KIND_RESUME_INTERRUPTED_WORK.into(),
                summary: "Interrupted".into(),
                explanation: "interrupted".into(),
                confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                evidence_score: 3,
                entity_ids: vec!["stable-x".into()],
                evidence: vec![DesktopDecisionEvidence {
                    plane: "semantics".into(),
                    reference: "stable-x".into(),
                    detail: "role=interrupted".into(),
                }],
                supporting_planes: vec!["semantics".into()],
                authority_effect: DesktopDecision::AUTHORITY_EFFECT_NONE.into(),
            },
        ];
        let semantics = DesktopSemanticProjection::empty();
        let memory = DesktopRuntimeMemory::empty();
        let behaviour = DesktopBehaviourTimeline::empty();
        let attention = project_desktop_attention(&decisions, &semantics, &memory, &behaviour);
        assert!(attention
            .items
            .iter()
            .any(|item| item.kind == DesktopAttentionItem::KIND_INTERRUPTED_WORK));
        assert!(!attention.items.iter().any(|item| {
            item.kind == DesktopAttentionItem::KIND_WORKING_FOCUS
                && item.entity_ids.iter().any(|id| id == "stable-x")
        }));
    }

    #[test]
    fn gap_fills_incomplete_desktop_from_memory() {
        let decisions = DesktopDecisionProjection::empty();
        let semantics = DesktopSemanticProjection::empty();
        let mut memory = DesktopRuntimeMemory::empty();
        memory.absent_count = 3;
        memory.present_count = 1;
        for i in 0..3 {
            memory.entities.push(DesktopObjectMemory {
                stable_window_id: format!("absent-{i}"),
                hwnd: format!("0x{i}"),
                title: format!("Absent {i}"),
                process_id: i,
                process_name: None,
                first_observed_at: "2026-07-30T10:00:00Z".into(),
                last_observed_at: "2026-07-30T10:00:00Z".into(),
                identity_confidence: "high".into(),
                presence: "absent".into(),
                sample_presence_count: 1,
                session_presence_count: 1,
                focus_count: 0,
                opened_count: 0,
                closed_count: 1,
                recurrence_count: 0,
                stability: "ephemeral".into(),
                continuity_confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
                knowledge: "temporary".into(),
                authority_effect: DesktopObjectMemory::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        let behaviour = DesktopBehaviourTimeline::empty();
        let attention = project_desktop_attention(&decisions, &semantics, &memory, &behaviour);
        assert!(attention.items.iter().any(|item| {
            item.kind == DesktopAttentionItem::KIND_INCOMPLETE_DESKTOP
        }));
    }
}
