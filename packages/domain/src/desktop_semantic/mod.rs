//! Deterministic desktop semantic projection from WorkspaceState evidence planes.
//!
//! Roles, relationships, and later activities/graph emerge from behaviour,
//! runtime memory, and groups only. No app-name tables, no LLM, no Win32.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

use crate::desktop_behaviour::DesktopBehaviourTimeline;
use crate::desktop_grouping::DesktopWindowGroup;
use crate::desktop_runtime_memory::{DesktopObjectMemory, DesktopRuntimeMemory};

/// Soft bound for semantic object rows.
pub const DESKTOP_SEMANTIC_OBJECT_LIMIT: usize = 100;

/// Soft bound for semantic relationship rows.
pub const DESKTOP_SEMANTIC_RELATIONSHIP_LIMIT: usize = 64;

/// Soft bound for inferred activities.
pub const DESKTOP_SEMANTIC_ACTIVITY_LIMIT: usize = 16;

/// Soft bound for projected graph edges.
pub const DESKTOP_SEMANTIC_GRAPH_EDGE_LIMIT: usize = 96;

/// Evidence-driven semantic role for a desktop object.
///
/// Roles are behavioural — never product taxonomies or application names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticObject {
    pub stable_window_id: String,
    pub hwnd: String,
    pub title: String,
    /// Primary role: `working` | `companion` | `alternating` | `background` |
    /// `utility` | `returning` | `interrupted` | `emerging` | `cluster`
    pub role: String,
    /// `ephemeral` | `routine` | `emerging` | `important` | `fading`
    pub importance: String,
    /// `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
    pub evidence_score: i32,
    pub authority_effect: String,
}

impl DesktopSemanticObject {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ROLE_WORKING: &'static str = "working";
    pub const ROLE_COMPANION: &'static str = "companion";
    pub const ROLE_ALTERNATING: &'static str = "alternating";
    pub const ROLE_BACKGROUND: &'static str = "background";
    pub const ROLE_UTILITY: &'static str = "utility";
    pub const ROLE_RETURNING: &'static str = "returning";
    pub const ROLE_INTERRUPTED: &'static str = "interrupted";
    pub const ROLE_EMERGING: &'static str = "emerging";
    pub const ROLE_CLUSTER: &'static str = "cluster";
    pub const IMPORTANCE_EPHEMERAL: &'static str = "ephemeral";
    pub const IMPORTANCE_ROUTINE: &'static str = "routine";
    pub const IMPORTANCE_EMERGING: &'static str = "emerging";
    pub const IMPORTANCE_IMPORTANT: &'static str = "important";
    pub const IMPORTANCE_FADING: &'static str = "fading";
}

/// Typed semantic relationship inferred from observation edges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticRelationship {
    pub from_stable_window_id: String,
    pub to_stable_window_id: String,
    /// `works_with` | `commonly_accompanies` | `precedes` | `follows` |
    /// `frequently_alternates` | `belongs_inside` | `supports`
    pub kind: String,
    pub evidence_count: i32,
    pub session_count: i32,
    /// `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
    pub authority_effect: String,
}

impl DesktopSemanticRelationship {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const KIND_WORKS_WITH: &'static str = "works_with";
    pub const KIND_COMMONLY_ACCOMPANIES: &'static str = "commonly_accompanies";
    pub const KIND_PRECEDES: &'static str = "precedes";
    pub const KIND_FOLLOWS: &'static str = "follows";
    pub const KIND_FREQUENTLY_ALTERNATES: &'static str = "frequently_alternates";
    pub const KIND_BELONGS_INSIDE: &'static str = "belongs_inside";
    pub const KIND_SUPPORTS: &'static str = "supports";
}

/// Session-scoped activity inferred from aggregate behaviour (not user labels).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticActivity {
    pub id: String,
    pub session_id: Option<String>,
    /// `focused_work` | `task_switching` | `comparing` | `monitoring` |
    /// `returning_work` | `interrupted_work` | `ambient`
    pub kind: String,
    /// `structural` | `emerging` | `recurring` | `strong`
    pub confidence: String,
    pub evidence_score: i32,
    pub member_ids: Vec<String>,
    pub authority_effect: String,
}

impl DesktopSemanticActivity {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const KIND_FOCUSED_WORK: &'static str = "focused_work";
    pub const KIND_TASK_SWITCHING: &'static str = "task_switching";
    pub const KIND_COMPARING: &'static str = "comparing";
    pub const KIND_MONITORING: &'static str = "monitoring";
    pub const KIND_RETURNING_WORK: &'static str = "returning_work";
    pub const KIND_INTERRUPTED_WORK: &'static str = "interrupted_work";
    pub const KIND_AMBIENT: &'static str = "ambient";
}

/// Graph node projected from semantic objects / groups / sessions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticGraphNode {
    pub id: String,
    /// `object` | `group` | `session` | `activity`
    pub kind: String,
    pub label: String,
    pub authority_effect: String,
}

/// Graph edge projected from semantic relationships / membership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticGraphEdge {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    pub confidence: String,
    pub authority_effect: String,
}

/// Deterministic desktop knowledge graph view (not a separate store).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticGraph {
    pub nodes: Vec<DesktopSemanticGraphNode>,
    pub edges: Vec<DesktopSemanticGraphEdge>,
    pub authority_effect: String,
}

impl DesktopSemanticGraph {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Semantic projection attached to WorkspaceState.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopSemanticProjection {
    pub objects: Vec<DesktopSemanticObject>,
    pub relationships: Vec<DesktopSemanticRelationship>,
    pub activities: Vec<DesktopSemanticActivity>,
    pub graph: DesktopSemanticGraph,
    pub authority_effect: String,
}

impl DesktopSemanticProjection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            relationships: Vec::new(),
            activities: Vec::new(),
            graph: DesktopSemanticGraph::empty(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Project semantic understanding from existing WorkspaceState evidence planes.
pub fn project_desktop_semantics(
    behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
    groups: &[DesktopWindowGroup],
) -> DesktopSemanticProjection {
    let objects = project_semantic_objects(behaviour, memory, groups);
    let relationships = project_semantic_relationships(behaviour, groups, &objects);
    let activities = project_semantic_activities(behaviour, memory, &objects, &relationships);
    let graph = project_semantic_graph(&objects, &relationships, &activities, groups, behaviour);

    let mut projection = DesktopSemanticProjection {
        objects,
        relationships,
        activities,
        graph,
        authority_effect: DesktopSemanticProjection::AUTHORITY_EFFECT_NONE.into(),
    };
    refine_semantic_confidence(&mut projection, behaviour, memory);
    // Rebuild graph after confidence/relationship refinement so edges stay consistent.
    projection.graph = project_semantic_graph(
        &projection.objects,
        &projection.relationships,
        &projection.activities,
        groups,
        behaviour,
    );
    projection
}

/// Strengthen or weaken semantic confidence from multi-signal agreement.
fn refine_semantic_confidence(
    projection: &mut DesktopSemanticProjection,
    behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
) {
    let memory_by_id: HashMap<&str, &DesktopObjectMemory> = memory
        .entities
        .iter()
        .map(|entity| (entity.stable_window_id.as_str(), entity))
        .collect();
    let dominant_ids: BTreeSet<String> = behaviour
        .sessions
        .iter()
        .filter(|session| session.kind == "active" || session.kind == "returning")
        .filter_map(|session| {
            session
                .dominant_focus
                .as_ref()
                .and_then(|window| window.stable_window_id.clone())
        })
        .collect();

    for object in &mut projection.objects {
        let mut score = object.evidence_score;
        if let Some(entity) = memory_by_id.get(object.stable_window_id.as_str()) {
            let agrees_working = object.role == DesktopSemanticObject::ROLE_WORKING
                && (entity.knowledge == DesktopObjectMemory::KNOWLEDGE_RISING
                    || entity.knowledge == DesktopObjectMemory::KNOWLEDGE_ESTABLISHED
                    || entity.knowledge == DesktopObjectMemory::KNOWLEDGE_PERSISTENT
                    || dominant_ids.contains(&object.stable_window_id));
            let agrees_returning = object.role == DesktopSemanticObject::ROLE_RETURNING
                && (entity.presence == DesktopObjectMemory::PRESENCE_RETURNING
                    || entity.knowledge == DesktopObjectMemory::KNOWLEDGE_RETURNING);
            let conflict_working = object.role == DesktopSemanticObject::ROLE_WORKING
                && (entity.knowledge == DesktopObjectMemory::KNOWLEDGE_TEMPORARY
                    || entity.focus_count == 0);
            let conflict_background = object.role == DesktopSemanticObject::ROLE_BACKGROUND
                && entity.focus_count >= 3;

            if agrees_working || agrees_returning {
                score = score.saturating_add(2);
            }
            if conflict_working || conflict_background {
                score = score.saturating_sub(2).max(1);
                if conflict_working && entity.knowledge == DesktopObjectMemory::KNOWLEDGE_TEMPORARY
                {
                    object.role = DesktopSemanticObject::ROLE_UTILITY.into();
                }
                if conflict_background {
                    object.role = DesktopSemanticObject::ROLE_WORKING.into();
                }
                object.importance = importance_for(entity, object.role.as_str()).into();
            }
        }
        object.evidence_score = score;
        object.confidence = DesktopWindowGroup::confidence_for_evidence(score).into();
    }

    let role_by_id: HashMap<&str, &str> = projection
        .objects
        .iter()
        .map(|object| (object.stable_window_id.as_str(), object.role.as_str()))
        .collect();

    // Drop directional follow edges already covered by alternation.
    let alternate_pairs: BTreeSet<(String, String)> = projection
        .relationships
        .iter()
        .filter(|rel| rel.kind == DesktopSemanticRelationship::KIND_FREQUENTLY_ALTERNATES)
        .map(|rel| ordered_pair(&rel.from_stable_window_id, &rel.to_stable_window_id))
        .collect();
    projection.relationships.retain(|rel| {
        if rel.kind == DesktopSemanticRelationship::KIND_PRECEDES
            || rel.kind == DesktopSemanticRelationship::KIND_FOLLOWS
        {
            !alternate_pairs
                .contains(&ordered_pair(&rel.from_stable_window_id, &rel.to_stable_window_id))
        } else {
            true
        }
    });

    for rel in &mut projection.relationships {
        let mut evidence = rel.evidence_count;
        let from_role = role_by_id.get(rel.from_stable_window_id.as_str()).copied();
        let to_role = role_by_id.get(rel.to_stable_window_id.as_str()).copied();
        let both_meaningful = matches!(
            from_role,
            Some(
                DesktopSemanticObject::ROLE_WORKING
                    | DesktopSemanticObject::ROLE_COMPANION
                    | DesktopSemanticObject::ROLE_ALTERNATING
                    | DesktopSemanticObject::ROLE_EMERGING
                    | DesktopSemanticObject::ROLE_RETURNING
            )
        ) && matches!(
            to_role,
            Some(
                DesktopSemanticObject::ROLE_WORKING
                    | DesktopSemanticObject::ROLE_COMPANION
                    | DesktopSemanticObject::ROLE_ALTERNATING
                    | DesktopSemanticObject::ROLE_EMERGING
                    | DesktopSemanticObject::ROLE_RETURNING
                    | DesktopSemanticObject::ROLE_CLUSTER
            )
        );
        if both_meaningful && rel.session_count >= 2 {
            evidence = evidence.saturating_add(2);
        }
        if rel.kind == DesktopSemanticRelationship::KIND_WORKS_WITH && both_meaningful {
            evidence = evidence.saturating_add(1);
        }
        if matches!(
            from_role,
            Some(DesktopSemanticObject::ROLE_UTILITY)
        ) && matches!(to_role, Some(DesktopSemanticObject::ROLE_UTILITY))
            && rel.kind != DesktopSemanticRelationship::KIND_BELONGS_INSIDE
        {
            evidence = evidence.saturating_sub(1).max(1);
        }
        rel.evidence_count = evidence;
        rel.confidence = DesktopWindowGroup::confidence_for_evidence(
            evidence.saturating_add(rel.session_count.saturating_sub(1)),
        )
        .into();
    }

    for activity in &mut projection.activities {
        let members_agree = activity.member_ids.iter().any(|id| {
            role_by_id.get(id.as_str()) == Some(&DesktopSemanticObject::ROLE_WORKING)
                || role_by_id.get(id.as_str()) == Some(&DesktopSemanticObject::ROLE_ALTERNATING)
        });
        let mut score = activity.evidence_score;
        if members_agree {
            score = score.saturating_add(1);
        }
        if activity.kind == DesktopSemanticActivity::KIND_COMPARING
            && projection.relationships.iter().any(|rel| {
                rel.kind == DesktopSemanticRelationship::KIND_FREQUENTLY_ALTERNATES
            })
        {
            score = score.saturating_add(2);
        }
        activity.evidence_score = score;
        activity.confidence = DesktopWindowGroup::confidence_for_evidence(score).into();
    }
}

fn project_semantic_objects(
    behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
    groups: &[DesktopWindowGroup],
) -> Vec<DesktopSemanticObject> {
    if memory.entities.is_empty() && behaviour.sample_count <= 0 {
        return Vec::new();
    }

    let focus_by_id = focus_counts(behaviour);
    let span_seconds_by_id = span_seconds(behaviour);
    let outbound_follows = follow_counts(behaviour, true);
    let inbound_follows = follow_counts(behaviour, false);
    let alternate_pairs = alternating_pairs(behaviour);
    let companion_of_hub = companion_candidates(behaviour, &focus_by_id);
    let strong_group_members = strong_group_member_ids(groups);

    let mut objects = Vec::new();
    for entity in &memory.entities {
        let id = entity.stable_window_id.as_str();
        let focus = focus_by_id.get(id).copied().unwrap_or(entity.focus_count);
        let span = span_seconds_by_id.get(id).copied().unwrap_or(0);
        let out = outbound_follows.get(id).copied().unwrap_or(0);
        let inn = inbound_follows.get(id).copied().unwrap_or(0);
        let alternates = alternate_pairs.contains(id);
        let companion = companion_of_hub.contains(id);
        let in_strong_group = strong_group_members.contains(id);

        let (role, score) = assign_role(
            entity,
            focus,
            span,
            out,
            inn,
            alternates,
            companion,
            in_strong_group,
            behaviour,
        );

        objects.push(DesktopSemanticObject {
            stable_window_id: entity.stable_window_id.clone(),
            hwnd: entity.hwnd.clone(),
            title: entity.title.clone(),
            role: role.into(),
            importance: importance_for(entity, role).into(),
            confidence: DesktopWindowGroup::confidence_for_evidence(score).into(),
            evidence_score: score,
            authority_effect: DesktopSemanticObject::AUTHORITY_EFFECT_NONE.into(),
        });
    }

    objects.sort_by(|a, b| {
        b.evidence_score
            .cmp(&a.evidence_score)
            .then_with(|| a.stable_window_id.cmp(&b.stable_window_id))
    });
    objects.truncate(DESKTOP_SEMANTIC_OBJECT_LIMIT);
    objects
}

fn importance_for(entity: &DesktopObjectMemory, role: &str) -> &'static str {
    if entity.knowledge == DesktopObjectMemory::KNOWLEDGE_FADING
        || role == DesktopSemanticObject::ROLE_INTERRUPTED
    {
        return DesktopSemanticObject::IMPORTANCE_FADING;
    }
    if entity.knowledge == DesktopObjectMemory::KNOWLEDGE_RISING
        || role == DesktopSemanticObject::ROLE_EMERGING
    {
        return DesktopSemanticObject::IMPORTANCE_EMERGING;
    }
    if role == DesktopSemanticObject::ROLE_WORKING
        || (entity.knowledge == DesktopObjectMemory::KNOWLEDGE_PERSISTENT && entity.focus_count >= 2)
    {
        return DesktopSemanticObject::IMPORTANCE_IMPORTANT;
    }
    if entity.knowledge == DesktopObjectMemory::KNOWLEDGE_TEMPORARY
        || entity.stability == DesktopObjectMemory::STABILITY_EPHEMERAL
        || role == DesktopSemanticObject::ROLE_UTILITY
    {
        return DesktopSemanticObject::IMPORTANCE_EPHEMERAL;
    }
    DesktopSemanticObject::IMPORTANCE_ROUTINE
}

fn assign_role(
    entity: &DesktopObjectMemory,
    focus: i32,
    span_seconds: i32,
    outbound: i32,
    inbound: i32,
    alternates: bool,
    companion: bool,
    in_strong_group: bool,
    behaviour: &DesktopBehaviourTimeline,
) -> (&'static str, i32) {
    let is_dominant = behaviour
        .sessions
        .iter()
        .filter(|session| session.kind == "active" || session.kind == "returning")
        .any(|session| {
            session
                .dominant_focus
                .as_ref()
                .and_then(|window| window.stable_window_id.as_deref())
                .is_some_and(|id| id == entity.stable_window_id)
        });

    if entity.knowledge == DesktopObjectMemory::KNOWLEDGE_INTERRUPTED
        || entity.knowledge == DesktopObjectMemory::KNOWLEDGE_FADING
    {
        return (
            DesktopSemanticObject::ROLE_INTERRUPTED,
            2 + entity.sample_presence_count.min(4),
        );
    }
    if entity.presence == DesktopObjectMemory::PRESENCE_RETURNING
        || entity.knowledge == DesktopObjectMemory::KNOWLEDGE_RETURNING
    {
        return (
            DesktopSemanticObject::ROLE_RETURNING,
            3 + entity.recurrence_count.min(4),
        );
    }
    if entity.knowledge == DesktopObjectMemory::KNOWLEDGE_RISING {
        return (
            DesktopSemanticObject::ROLE_EMERGING,
            3 + focus.min(4) + entity.recurrence_count.min(2),
        );
    }

    let working_score = focus.saturating_mul(2) + (span_seconds / 60).min(6) + if is_dominant { 3 } else { 0 };
    if working_score >= 5 || (focus >= 2 && is_dominant) {
        return (DesktopSemanticObject::ROLE_WORKING, working_score.max(3));
    }
    if alternates && (outbound + inbound) >= 2 {
        return (
            DesktopSemanticObject::ROLE_ALTERNATING,
            2 + outbound + inbound,
        );
    }
    if companion {
        return (
            DesktopSemanticObject::ROLE_COMPANION,
            2 + entity.sample_presence_count.min(4),
        );
    }
    if entity.knowledge == DesktopObjectMemory::KNOWLEDGE_TEMPORARY
        || (entity.stability == DesktopObjectMemory::STABILITY_EPHEMERAL
            && (entity.opened_count + entity.closed_count) >= 2)
    {
        return (
            DesktopSemanticObject::ROLE_UTILITY,
            2 + entity.opened_count + entity.closed_count,
        );
    }
    if entity.stability == DesktopObjectMemory::STABILITY_PERSISTENT
        || entity.stability == DesktopObjectMemory::STABILITY_STABLE
    {
        if focus <= 1 {
            return (
                DesktopSemanticObject::ROLE_BACKGROUND,
                2 + entity.sample_presence_count.min(5),
            );
        }
    }
    if in_strong_group {
        return (
            DesktopSemanticObject::ROLE_CLUSTER,
            2 + entity.session_presence_count.min(4),
        );
    }
    if companion || entity.sample_presence_count >= 2 {
        (
            DesktopSemanticObject::ROLE_COMPANION,
            1 + entity.sample_presence_count.min(3),
        )
    } else {
        (
            DesktopSemanticObject::ROLE_UTILITY,
            1 + entity.sample_presence_count.min(2),
        )
    }
}

fn project_semantic_relationships(
    behaviour: &DesktopBehaviourTimeline,
    groups: &[DesktopWindowGroup],
    objects: &[DesktopSemanticObject],
) -> Vec<DesktopSemanticRelationship> {
    let mut relationships = Vec::new();
    let role_by_id: HashMap<&str, &str> = objects
        .iter()
        .map(|object| (object.stable_window_id.as_str(), object.role.as_str()))
        .collect();

    let mut alternate_keys: BTreeSet<(String, String)> = BTreeSet::new();
    for follow in &behaviour.focus_follows {
        let Some(from) = stable_of(&follow.from.stable_window_id, &follow.from.hwnd) else {
            continue;
        };
        let Some(to) = stable_of(&follow.to.stable_window_id, &follow.to.hwnd) else {
            continue;
        };
        let reverse = behaviour.focus_follows.iter().any(|other| {
            stable_of(&other.from.stable_window_id, &other.from.hwnd).as_deref() == Some(to.as_str())
                && stable_of(&other.to.stable_window_id, &other.to.hwnd).as_deref()
                    == Some(from.as_str())
        });
        if reverse {
            let key = ordered_pair(&from, &to);
            if alternate_keys.insert(key.clone()) {
                let reverse_count = behaviour
                    .focus_follows
                    .iter()
                    .find(|other| {
                        stable_of(&other.from.stable_window_id, &other.from.hwnd).as_deref()
                            == Some(to.as_str())
                            && stable_of(&other.to.stable_window_id, &other.to.hwnd).as_deref()
                                == Some(from.as_str())
                    })
                    .map(|other| other.transition_count)
                    .unwrap_or(0);
                let evidence = follow.transition_count + reverse_count;
                let sessions = follow.session_count.max(
                    behaviour
                        .focus_follows
                        .iter()
                        .find(|other| {
                            stable_of(&other.from.stable_window_id, &other.from.hwnd).as_deref()
                                == Some(to.as_str())
                                && stable_of(&other.to.stable_window_id, &other.to.hwnd).as_deref()
                                    == Some(from.as_str())
                        })
                        .map(|other| other.session_count)
                        .unwrap_or(0),
                );
                relationships.push(DesktopSemanticRelationship {
                    from_stable_window_id: key.0.clone(),
                    to_stable_window_id: key.1.clone(),
                    kind: DesktopSemanticRelationship::KIND_FREQUENTLY_ALTERNATES.into(),
                    evidence_count: evidence,
                    session_count: sessions,
                    confidence: DesktopWindowGroup::confidence_for_evidence(
                        evidence.saturating_add(sessions.saturating_sub(1)),
                    )
                    .into(),
                    authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
                });
            }
            continue;
        }
        let from_is_working = role_by_id.get(from.as_str()) == Some(&DesktopSemanticObject::ROLE_WORKING);
        let kind = if from_is_working {
            DesktopSemanticRelationship::KIND_PRECEDES
        } else {
            DesktopSemanticRelationship::KIND_FOLLOWS
        };
        relationships.push(DesktopSemanticRelationship {
            from_stable_window_id: from,
            to_stable_window_id: to,
            kind: kind.into(),
            evidence_count: follow.transition_count,
            session_count: follow.session_count,
            confidence: follow.confidence.clone(),
            authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
        });
    }

    for pair in &behaviour.co_presence {
        let Some(left) = stable_of(&pair.left.stable_window_id, &pair.left.hwnd) else {
            continue;
        };
        let Some(right) = stable_of(&pair.right.stable_window_id, &pair.right.hwnd) else {
            continue;
        };
        let kind = if pair.session_count >= 2 || pair.sample_count >= 4 {
            DesktopSemanticRelationship::KIND_WORKS_WITH
        } else {
            DesktopSemanticRelationship::KIND_COMMONLY_ACCOMPANIES
        };
        relationships.push(DesktopSemanticRelationship {
            from_stable_window_id: left.clone(),
            to_stable_window_id: right.clone(),
            kind: kind.into(),
            evidence_count: pair.sample_count,
            session_count: pair.session_count,
            confidence: pair.confidence.clone(),
            authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
        });
        let left_role = role_by_id.get(left.as_str()).copied();
        let right_role = role_by_id.get(right.as_str()).copied();
        if left_role == Some(DesktopSemanticObject::ROLE_COMPANION)
            && right_role == Some(DesktopSemanticObject::ROLE_WORKING)
        {
            relationships.push(DesktopSemanticRelationship {
                from_stable_window_id: left,
                to_stable_window_id: right,
                kind: DesktopSemanticRelationship::KIND_SUPPORTS.into(),
                evidence_count: pair.sample_count,
                session_count: pair.session_count,
                confidence: pair.confidence.clone(),
                authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
            });
        } else if right_role == Some(DesktopSemanticObject::ROLE_COMPANION)
            && left_role == Some(DesktopSemanticObject::ROLE_WORKING)
        {
            relationships.push(DesktopSemanticRelationship {
                from_stable_window_id: right,
                to_stable_window_id: left,
                kind: DesktopSemanticRelationship::KIND_SUPPORTS.into(),
                evidence_count: pair.sample_count,
                session_count: pair.session_count,
                confidence: pair.confidence.clone(),
                authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
            });
        }
    }

    for group in groups {
        if group.member_ids.len() < 2 {
            continue;
        }
        if group.confidence == DesktopWindowGroup::CONFIDENCE_STRUCTURAL
            && group.evidence_count <= 1
        {
            continue;
        }
        for member in &group.member_ids {
            relationships.push(DesktopSemanticRelationship {
                from_stable_window_id: member.clone(),
                to_stable_window_id: group.id.clone(),
                kind: DesktopSemanticRelationship::KIND_BELONGS_INSIDE.into(),
                evidence_count: group.evidence_count,
                session_count: 0,
                confidence: group.confidence.clone(),
                authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
            });
        }
    }

    relationships.sort_by(|a, b| {
        b.evidence_count
            .cmp(&a.evidence_count)
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.from_stable_window_id.cmp(&b.from_stable_window_id))
            .then_with(|| a.to_stable_window_id.cmp(&b.to_stable_window_id))
    });
    relationships.truncate(DESKTOP_SEMANTIC_RELATIONSHIP_LIMIT);
    relationships
}

fn project_semantic_activities(
    behaviour: &DesktopBehaviourTimeline,
    memory: &DesktopRuntimeMemory,
    objects: &[DesktopSemanticObject],
    relationships: &[DesktopSemanticRelationship],
) -> Vec<DesktopSemanticActivity> {
    let mut activities = Vec::new();
    if behaviour.sessions.is_empty() {
        if behaviour.sample_count > 0 {
            activities.push(infer_activity_from_window(
                "activity:coverage",
                None,
                behaviour,
                memory,
                objects,
                relationships,
            ));
        }
        activities.truncate(DESKTOP_SEMANTIC_ACTIVITY_LIMIT);
        return activities;
    }

    for session in &behaviour.sessions {
        let activity = infer_activity_from_window(
            &format!("activity:{}", session.id),
            Some(session.id.as_str()),
            behaviour,
            memory,
            objects,
            relationships,
        );
        // Re-score using session-local transition density.
        let mut activity = activity;
        activity.session_id = Some(session.id.clone());
        if session.kind == "returning" {
            activity.kind = DesktopSemanticActivity::KIND_RETURNING_WORK.into();
            activity.evidence_score = activity.evidence_score.saturating_add(2);
        } else if session.kind == "completed"
            && memory
                .entities
                .iter()
                .any(|entity| entity.knowledge == DesktopObjectMemory::KNOWLEDGE_INTERRUPTED)
        {
            activity.kind = DesktopSemanticActivity::KIND_INTERRUPTED_WORK.into();
        }
        activity.confidence =
            DesktopWindowGroup::confidence_for_evidence(activity.evidence_score).into();
        activities.push(activity);
    }

    activities.truncate(DESKTOP_SEMANTIC_ACTIVITY_LIMIT);
    activities
}

fn infer_activity_from_window(
    id: &str,
    session_id: Option<&str>,
    behaviour: &DesktopBehaviourTimeline,
    _memory: &DesktopRuntimeMemory,
    objects: &[DesktopSemanticObject],
    relationships: &[DesktopSemanticRelationship],
) -> DesktopSemanticActivity {
    let working_ids: Vec<String> = objects
        .iter()
        .filter(|object| {
            object.role == DesktopSemanticObject::ROLE_WORKING
                || object.role == DesktopSemanticObject::ROLE_EMERGING
                || object.role == DesktopSemanticObject::ROLE_ALTERNATING
        })
        .map(|object| object.stable_window_id.clone())
        .collect();

    let transition_count = if let Some(session_id) = session_id {
        behaviour
            .sessions
            .iter()
            .find(|session| session.id == session_id)
            .map(|session| session.focus_transition_count)
            .unwrap_or(behaviour.focus_transitions.len() as i32)
    } else {
        behaviour.focus_transitions.len() as i32
    };
    let sample_count = if let Some(session_id) = session_id {
        behaviour
            .sessions
            .iter()
            .find(|session| session.id == session_id)
            .map(|session| session.sample_count)
            .unwrap_or(behaviour.sample_count)
    } else {
        behaviour.sample_count
    };
    let alternates = relationships
        .iter()
        .filter(|rel| rel.kind == DesktopSemanticRelationship::KIND_FREQUENTLY_ALTERNATES)
        .count();
    let background_count = objects
        .iter()
        .filter(|object| object.role == DesktopSemanticObject::ROLE_BACKGROUND)
        .count();

    let (kind, score) = if alternates >= 1 && transition_count >= 2 {
        (
            DesktopSemanticActivity::KIND_COMPARING,
            3 + alternates as i32 + transition_count.min(4),
        )
    } else if transition_count >= 4 || (sample_count > 0 && transition_count * 2 >= sample_count) {
        (
            DesktopSemanticActivity::KIND_TASK_SWITCHING,
            3 + transition_count.min(6),
        )
    } else if working_ids.len() == 1 && transition_count <= 2 {
        (
            DesktopSemanticActivity::KIND_FOCUSED_WORK,
            3 + objects
                .iter()
                .find(|object| object.role == DesktopSemanticObject::ROLE_WORKING)
                .map(|object| object.evidence_score.min(4))
                .unwrap_or(1),
        )
    } else if background_count >= 2 && transition_count <= 1 {
        (
            DesktopSemanticActivity::KIND_MONITORING,
            2 + background_count as i32,
        )
    } else if working_ids.is_empty() {
        (DesktopSemanticActivity::KIND_AMBIENT, 1 + sample_count.min(3))
    } else {
        (
            DesktopSemanticActivity::KIND_TASK_SWITCHING,
            2 + transition_count.min(4),
        )
    };

    DesktopSemanticActivity {
        id: id.into(),
        session_id: session_id.map(str::to_string),
        kind: kind.into(),
        confidence: DesktopWindowGroup::confidence_for_evidence(score).into(),
        evidence_score: score,
        member_ids: working_ids,
        authority_effect: DesktopSemanticActivity::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn project_semantic_graph(
    objects: &[DesktopSemanticObject],
    relationships: &[DesktopSemanticRelationship],
    activities: &[DesktopSemanticActivity],
    groups: &[DesktopWindowGroup],
    behaviour: &DesktopBehaviourTimeline,
) -> DesktopSemanticGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for object in objects {
        nodes.push(DesktopSemanticGraphNode {
            id: format!("object:{}", object.stable_window_id),
            kind: "object".into(),
            label: format!("{} ({})", object.title, object.role),
            authority_effect: DesktopSemanticObject::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    for group in groups {
        nodes.push(DesktopSemanticGraphNode {
            id: format!("group:{}", group.id),
            kind: "group".into(),
            label: group.label.clone(),
            authority_effect: DesktopWindowGroup::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    for session in &behaviour.sessions {
        nodes.push(DesktopSemanticGraphNode {
            id: format!("session:{}", session.id),
            kind: "session".into(),
            label: format!("{} ({})", session.kind, session.confidence),
            authority_effect: DesktopSemanticObject::AUTHORITY_EFFECT_NONE.into(),
        });
    }
    for activity in activities {
        nodes.push(DesktopSemanticGraphNode {
            id: activity.id.clone(),
            kind: "activity".into(),
            label: activity.kind.clone(),
            authority_effect: DesktopSemanticActivity::AUTHORITY_EFFECT_NONE.into(),
        });
        if let Some(session_id) = &activity.session_id {
            edges.push(DesktopSemanticGraphEdge {
                id: format!("edge:activity-session:{}:{}", activity.id, session_id),
                from_id: activity.id.clone(),
                to_id: format!("session:{session_id}"),
                kind: "occurs_in".into(),
                confidence: activity.confidence.clone(),
                authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        for member in &activity.member_ids {
            edges.push(DesktopSemanticGraphEdge {
                id: format!("edge:activity-member:{}:{}", activity.id, member),
                from_id: activity.id.clone(),
                to_id: format!("object:{member}"),
                kind: "involves".into(),
                confidence: activity.confidence.clone(),
                authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
            });
        }
    }

    for rel in relationships {
        let from_id = if rel.kind == DesktopSemanticRelationship::KIND_BELONGS_INSIDE {
            format!("object:{}", rel.from_stable_window_id)
        } else {
            format!("object:{}", rel.from_stable_window_id)
        };
        let to_id = if rel.kind == DesktopSemanticRelationship::KIND_BELONGS_INSIDE {
            format!("group:{}", rel.to_stable_window_id)
        } else {
            format!("object:{}", rel.to_stable_window_id)
        };
        edges.push(DesktopSemanticGraphEdge {
            id: format!(
                "edge:{}:{}:{}",
                rel.kind, rel.from_stable_window_id, rel.to_stable_window_id
            ),
            from_id,
            to_id,
            kind: rel.kind.clone(),
            confidence: rel.confidence.clone(),
            authority_effect: DesktopSemanticRelationship::AUTHORITY_EFFECT_NONE.into(),
        });
    }

    edges.truncate(DESKTOP_SEMANTIC_GRAPH_EDGE_LIMIT);
    DesktopSemanticGraph {
        nodes,
        edges,
        authority_effect: DesktopSemanticGraph::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn focus_counts(behaviour: &DesktopBehaviourTimeline) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for revisit in &behaviour.window_revisits {
        if let Some(id) = stable_of(&revisit.window.stable_window_id, &revisit.window.hwnd) {
            map.insert(id, revisit.focus_count);
        }
    }
    map
}

fn span_seconds(behaviour: &DesktopBehaviourTimeline) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for span in &behaviour.recent_focus_spans {
        if let Some(id) = stable_of(&span.window.stable_window_id, &span.window.hwnd) {
            let seconds = span.sample_span_seconds.unwrap_or(0) as i32;
            *map.entry(id).or_insert(0) += seconds;
        }
    }
    if let (Some(current), Some(seconds)) = (
        behaviour.current_focus.as_ref(),
        behaviour.current_focus_sample_span_seconds,
    ) {
        if let Some(id) = stable_of(&current.stable_window_id, &current.hwnd) {
            *map.entry(id).or_insert(0) += seconds as i32;
        }
    }
    map
}

fn follow_counts(behaviour: &DesktopBehaviourTimeline, outbound: bool) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for follow in &behaviour.focus_follows {
        let key = if outbound {
            stable_of(&follow.from.stable_window_id, &follow.from.hwnd)
        } else {
            stable_of(&follow.to.stable_window_id, &follow.to.hwnd)
        };
        if let Some(id) = key {
            *map.entry(id).or_insert(0) += follow.transition_count;
        }
    }
    map
}

fn alternating_pairs(behaviour: &DesktopBehaviourTimeline) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for follow in &behaviour.focus_follows {
        let Some(from) = stable_of(&follow.from.stable_window_id, &follow.from.hwnd) else {
            continue;
        };
        let Some(to) = stable_of(&follow.to.stable_window_id, &follow.to.hwnd) else {
            continue;
        };
        let reverse = behaviour.focus_follows.iter().any(|other| {
            stable_of(&other.from.stable_window_id, &other.from.hwnd).as_deref() == Some(to.as_str())
                && stable_of(&other.to.stable_window_id, &other.to.hwnd).as_deref()
                    == Some(from.as_str())
        });
        if reverse {
            ids.insert(from);
            ids.insert(to);
        }
    }
    ids
}

fn companion_candidates(
    behaviour: &DesktopBehaviourTimeline,
    focus_by_id: &HashMap<String, i32>,
) -> BTreeSet<String> {
    let hub = focus_by_id
        .iter()
        .max_by(|a, b| a.1.cmp(b.1).then_with(|| a.0.cmp(b.0)))
        .map(|(id, _)| id.clone());
    let Some(hub) = hub else {
        return BTreeSet::new();
    };
    let mut companions = BTreeSet::new();
    for pair in &behaviour.co_presence {
        let Some(left) = stable_of(&pair.left.stable_window_id, &pair.left.hwnd) else {
            continue;
        };
        let Some(right) = stable_of(&pair.right.stable_window_id, &pair.right.hwnd) else {
            continue;
        };
        if pair.sample_count < 2 {
            continue;
        }
        if left == hub {
            companions.insert(right);
        } else if right == hub {
            companions.insert(left);
        }
    }
    companions
}

fn strong_group_member_ids(groups: &[DesktopWindowGroup]) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for group in groups {
        if group.confidence == DesktopWindowGroup::CONFIDENCE_RECURRING
            || group.confidence == DesktopWindowGroup::CONFIDENCE_STRONG
            || group.evidence_count >= 4
        {
            ids.extend(group.member_ids.iter().cloned());
        }
    }
    ids
}

fn stable_of(stable: &Option<String>, hwnd: &str) -> Option<String> {
    stable
        .as_ref()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            let hwnd = hwnd.trim();
            if hwnd.is_empty() {
                None
            } else {
                Some(hwnd.to_string())
            }
        })
}

fn ordered_pair(a: &str, b: &str) -> (String, String) {
    if a <= b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop_behaviour::{project_desktop_behaviour, DesktopFocusFollow};
    use crate::workspace_observation::{
        ObservedMonitor, ObservedWindow, WindowIdentityConfidence, ObservationWindowIdentity,
        WorkspaceObservationPass, WorkspaceObservationSnapshot,
    };
    use crate::workspace_observation_delta::ObservationWindowRef;
    use crate::desktop_runtime_memory::project_desktop_runtime_memory;

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
    fn projects_working_and_companion_roles_without_app_lookups() {
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

        let a = semantics
            .objects
            .iter()
            .find(|object| object.stable_window_id == "stable-a")
            .expect("stable-a");
        assert!(
            a.role == DesktopSemanticObject::ROLE_WORKING
                || a.role == DesktopSemanticObject::ROLE_EMERGING
                || a.role == DesktopSemanticObject::ROLE_ALTERNATING,
            "unexpected role {}",
            a.role
        );
        assert!(!semantics.relationships.is_empty());
        assert!(!semantics.activities.is_empty());
        assert!(!semantics.graph.nodes.is_empty());
        assert!(!semantics.graph.edges.is_empty());
        // Never depends on process_name taxonomy.
        assert!(semantics.objects.iter().all(|object| {
            ![
                "browser", "ide", "mail", "media", "chat", "communication", "development",
            ]
            .contains(&object.role.as_str())
        }));
    }

    #[test]
    fn marks_returning_role_from_memory_presence() {
        let mut memory = DesktopRuntimeMemory::empty();
        memory.entities.push(DesktopObjectMemory {
            stable_window_id: "stable-r".into(),
            hwnd: "0xr".into(),
            title: "Returning".into(),
            process_id: 1,
            process_name: Some("x.exe".into()),
            first_observed_at: "2026-07-30T09:00:00Z".into(),
            last_observed_at: "2026-07-30T11:00:00Z".into(),
            identity_confidence: "high".into(),
            presence: DesktopObjectMemory::PRESENCE_RETURNING.into(),
            sample_presence_count: 3,
            session_presence_count: 2,
            focus_count: 1,
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
        assert_eq!(semantics.objects[0].role, DesktopSemanticObject::ROLE_RETURNING);
    }

    #[test]
    fn projects_alternate_relationship_from_bidirectional_follows() {
        let mut behaviour = DesktopBehaviourTimeline::empty();
        behaviour.sample_count = 4;
        behaviour.focus_follows = vec![
            DesktopFocusFollow {
                from: ObservationWindowRef {
                    stable_window_id: Some("a".into()),
                    hwnd: "1".into(),
                    title: "A".into(),
                    process_id: 1,
                },
                to: ObservationWindowRef {
                    stable_window_id: Some("b".into()),
                    hwnd: "2".into(),
                    title: "B".into(),
                    process_id: 2,
                },
                transition_count: 2,
                last_at: "2026-07-30T10:00:00Z".into(),
                confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                session_count: 1,
            },
            DesktopFocusFollow {
                from: ObservationWindowRef {
                    stable_window_id: Some("b".into()),
                    hwnd: "2".into(),
                    title: "B".into(),
                    process_id: 2,
                },
                to: ObservationWindowRef {
                    stable_window_id: Some("a".into()),
                    hwnd: "1".into(),
                    title: "A".into(),
                    process_id: 1,
                },
                transition_count: 2,
                last_at: "2026-07-30T10:01:00Z".into(),
                confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                session_count: 1,
            },
        ];
        let mut memory = DesktopRuntimeMemory::empty();
        for (id, hwnd, title) in [("a", "1", "A"), ("b", "2", "B")] {
            memory.entities.push(DesktopObjectMemory {
                stable_window_id: id.into(),
                hwnd: hwnd.into(),
                title: title.into(),
                process_id: 1,
                process_name: None,
                first_observed_at: "2026-07-30T10:00:00Z".into(),
                last_observed_at: "2026-07-30T10:01:00Z".into(),
                identity_confidence: "high".into(),
                presence: DesktopObjectMemory::PRESENCE_PRESENT.into(),
                sample_presence_count: 2,
                session_presence_count: 1,
                focus_count: 2,
                opened_count: 0,
                closed_count: 0,
                recurrence_count: 0,
                stability: DesktopObjectMemory::STABILITY_STABLE.into(),
                continuity_confidence: DesktopWindowGroup::CONFIDENCE_EMERGING.into(),
                knowledge: DesktopObjectMemory::KNOWLEDGE_ESTABLISHED.into(),
                authority_effect: DesktopObjectMemory::AUTHORITY_EFFECT_NONE.into(),
            });
        }
        let semantics = project_desktop_semantics(&behaviour, &memory, &[]);
        assert!(semantics.relationships.iter().any(|rel| {
            rel.kind == DesktopSemanticRelationship::KIND_FREQUENTLY_ALTERNATES
        }));
    }

    #[test]
    fn demotes_working_role_when_temporary_and_unfocused() {
        let mut memory = DesktopRuntimeMemory::empty();
        memory.entities.push(DesktopObjectMemory {
            stable_window_id: "stable-t".into(),
            hwnd: "0xt".into(),
            title: "Temp".into(),
            process_id: 9,
            process_name: None,
            first_observed_at: "2026-07-30T10:00:00Z".into(),
            last_observed_at: "2026-07-30T10:00:00Z".into(),
            identity_confidence: "low".into(),
            presence: DesktopObjectMemory::PRESENCE_PRESENT.into(),
            sample_presence_count: 1,
            session_presence_count: 1,
            focus_count: 0,
            opened_count: 1,
            closed_count: 1,
            recurrence_count: 0,
            stability: DesktopObjectMemory::STABILITY_EPHEMERAL.into(),
            continuity_confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
            knowledge: DesktopObjectMemory::KNOWLEDGE_TEMPORARY.into(),
            authority_effect: DesktopObjectMemory::AUTHORITY_EFFECT_NONE.into(),
        });
        let mut behaviour = DesktopBehaviourTimeline::empty();
        behaviour.sample_count = 1;
        behaviour.sessions = vec![crate::desktop_behaviour::DesktopObservationSession {
            id: "session:p1:p1".into(),
            started_at: "2026-07-30T10:00:00Z".into(),
            ended_at: "2026-07-30T10:00:00Z".into(),
            start_pass_id: "p1".into(),
            end_pass_id: "p1".into(),
            sample_count: 1,
            focus_transition_count: 0,
            dominant_focus: Some(ObservationWindowRef {
                stable_window_id: Some("stable-t".into()),
                hwnd: "0xt".into(),
                title: "Temp".into(),
                process_id: 9,
            }),
            kind: "active".into(),
            confidence: DesktopWindowGroup::CONFIDENCE_STRUCTURAL.into(),
        }];
        let semantics = project_desktop_semantics(&behaviour, &memory, &[]);
        let object = &semantics.objects[0];
        assert_ne!(object.role, DesktopSemanticObject::ROLE_WORKING);
        assert!(
            object.role == DesktopSemanticObject::ROLE_UTILITY
                || object.role == DesktopSemanticObject::ROLE_INTERRUPTED
                || object.role == DesktopSemanticObject::ROLE_EMERGING
                || object.role == DesktopSemanticObject::ROLE_COMPANION
                || object.role == DesktopSemanticObject::ROLE_BACKGROUND
        );
    }
}
