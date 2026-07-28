//! Workspace Cognitive Graph — Programme II Batch 4.
//!
//! Cross-domain integration topology. Nodes and edges are projections over
//! existing identities only. Never owns lifecycle, execution, planning,
//! reasoning, recommendations, decisions, permissions, or task state.
//! Permanently non-executing. Never invents missing relationships.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceCognitiveGraphError {
    #[error("title must not be empty")]
    EmptyTitle,

    #[error("external_ref must not be empty")]
    EmptyExternalRef,

    #[error("invalid graph status: {0}")]
    InvalidStatus(String),

    #[error("invalid node kind: {0}")]
    InvalidNodeKind(String),

    #[error("invalid edge kind: {0}")]
    InvalidEdgeKind(String),

    #[error("confidence must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("graph artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("self-edges are forbidden")]
    SelfEdge,

    #[error("graph snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspaceCognitiveGraphError> {
    if value > 100 {
        Err(WorkspaceCognitiveGraphError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

fn normalize_nonempty(
    value: impl Into<String>,
    empty: WorkspaceCognitiveGraphError,
) -> Result<String, WorkspaceCognitiveGraphError> {
    let trimmed = value.into().trim().to_string();
    if trimmed.is_empty() {
        Err(empty)
    } else {
        Ok(trimmed)
    }
}

/// Informational snapshot status — no execution semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveGraphStatus {
    Current,
    Superseded,
    Archived,
}

impl CognitiveGraphStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveGraphError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceCognitiveGraphError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Node kinds — projections of existing domain objects only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveGraphNodeKind {
    Goal,
    Objective,
    Initiative,
    Milestone,
    Task,
    Recommendation,
    Decision,
    Plan,
    Constraint,
    Risk,
    Opportunity,
    Memory,
    Purpose,
    Attention,
    Execution,
    Reasoning,
    Context,
    WorkingSet,
    BrokenReference,
}

impl CognitiveGraphNodeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Goal => "goal",
            Self::Objective => "objective",
            Self::Initiative => "initiative",
            Self::Milestone => "milestone",
            Self::Task => "task",
            Self::Recommendation => "recommendation",
            Self::Decision => "decision",
            Self::Plan => "plan",
            Self::Constraint => "constraint",
            Self::Risk => "risk",
            Self::Opportunity => "opportunity",
            Self::Memory => "memory",
            Self::Purpose => "purpose",
            Self::Attention => "attention",
            Self::Execution => "execution",
            Self::Reasoning => "reasoning",
            Self::Context => "context",
            Self::WorkingSet => "working_set",
            Self::BrokenReference => "broken_reference",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveGraphError> {
        match value {
            "goal" => Ok(Self::Goal),
            "objective" => Ok(Self::Objective),
            "initiative" => Ok(Self::Initiative),
            "milestone" => Ok(Self::Milestone),
            "task" => Ok(Self::Task),
            "recommendation" => Ok(Self::Recommendation),
            "decision" => Ok(Self::Decision),
            "plan" => Ok(Self::Plan),
            "constraint" => Ok(Self::Constraint),
            "risk" => Ok(Self::Risk),
            "opportunity" => Ok(Self::Opportunity),
            "memory" => Ok(Self::Memory),
            "purpose" => Ok(Self::Purpose),
            "attention" => Ok(Self::Attention),
            "execution" => Ok(Self::Execution),
            "reasoning" => Ok(Self::Reasoning),
            "context" => Ok(Self::Context),
            "working_set" => Ok(Self::WorkingSet),
            "broken_reference" => Ok(Self::BrokenReference),
            other => Err(WorkspaceCognitiveGraphError::InvalidNodeKind(other.into())),
        }
    }
}

/// Typed edge kinds — meaning only, never authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveGraphEdgeKind {
    DependsOn,
    Supports,
    Blocks,
    Contradicts,
    DerivedFrom,
    InformedBy,
    Supersedes,
    References,
    Satisfies,
    CausedBy,
    EvidenceFor,
    RelatesTo,
    Constrains,
    PartOf,
    Focuses,
    Mitigates,
    Enables,
}

impl CognitiveGraphEdgeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DependsOn => "depends_on",
            Self::Supports => "supports",
            Self::Blocks => "blocks",
            Self::Contradicts => "contradicts",
            Self::DerivedFrom => "derived_from",
            Self::InformedBy => "informed_by",
            Self::Supersedes => "supersedes",
            Self::References => "references",
            Self::Satisfies => "satisfies",
            Self::CausedBy => "caused_by",
            Self::EvidenceFor => "evidence_for",
            Self::RelatesTo => "relates_to",
            Self::Constrains => "constrains",
            Self::PartOf => "part_of",
            Self::Focuses => "focuses",
            Self::Mitigates => "mitigates",
            Self::Enables => "enables",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveGraphError> {
        match value {
            "depends_on" => Ok(Self::DependsOn),
            "supports" => Ok(Self::Supports),
            "blocks" => Ok(Self::Blocks),
            "contradicts" => Ok(Self::Contradicts),
            "derived_from" => Ok(Self::DerivedFrom),
            "informed_by" => Ok(Self::InformedBy),
            "supersedes" => Ok(Self::Supersedes),
            "references" => Ok(Self::References),
            "satisfies" => Ok(Self::Satisfies),
            "caused_by" => Ok(Self::CausedBy),
            "evidence_for" => Ok(Self::EvidenceFor),
            "relates_to" => Ok(Self::RelatesTo),
            "constrains" => Ok(Self::Constrains),
            "part_of" => Ok(Self::PartOf),
            "focuses" => Ok(Self::Focuses),
            "mitigates" => Ok(Self::Mitigates),
            "enables" => Ok(Self::Enables),
            other => Err(WorkspaceCognitiveGraphError::InvalidEdgeKind(other.into())),
        }
    }
}

/// Projection of an existing domain object into the graph.
/// `external_ref` is the canonical identity — never replaced by a graph-owned id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphNode {
    pub external_ref: String,
    pub workspace_id: String,
    pub kind: CognitiveGraphNodeKind,
    pub title: String,
    /// True when the referenced identity could not be resolved in source systems.
    pub broken: bool,
    pub authority_effect: String,
}

impl CognitiveGraphNode {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn new(
        workspace_id: impl Into<String>,
        kind: CognitiveGraphNodeKind,
        external_ref: impl Into<String>,
        title: impl Into<String>,
        broken: bool,
    ) -> Result<Self, WorkspaceCognitiveGraphError> {
        Ok(Self {
            external_ref: normalize_nonempty(
                external_ref,
                WorkspaceCognitiveGraphError::EmptyExternalRef,
            )?,
            workspace_id: workspace_id.into(),
            kind,
            title: normalize_nonempty(title, WorkspaceCognitiveGraphError::EmptyTitle)?,
            broken,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveGraphError> {
        normalize_nonempty(
            &self.external_ref,
            WorkspaceCognitiveGraphError::EmptyExternalRef,
        )?;
        normalize_nonempty(&self.title, WorkspaceCognitiveGraphError::EmptyTitle)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceCognitiveGraphError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

/// Typed semantic edge — meaning only, never authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphEdge {
    pub id: String,
    pub workspace_id: String,
    pub from_ref: String,
    pub to_ref: String,
    pub kind: CognitiveGraphEdgeKind,
    pub explanation: String,
    pub confidence: u8,
    /// Evidence supporting this connection (reference ids only).
    pub evidence_refs: Vec<String>,
    /// True when either endpoint was unresolved at generation time.
    pub broken: bool,
    pub authority_effect: String,
}

impl CognitiveGraphEdge {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive_edge:";

    pub fn new(
        workspace_id: impl Into<String>,
        from_ref: impl Into<String>,
        to_ref: impl Into<String>,
        kind: CognitiveGraphEdgeKind,
        explanation: impl Into<String>,
        confidence: u8,
        evidence_refs: Vec<String>,
        broken: bool,
    ) -> Result<Self, WorkspaceCognitiveGraphError> {
        let from_ref =
            normalize_nonempty(from_ref, WorkspaceCognitiveGraphError::EmptyExternalRef)?;
        let to_ref = normalize_nonempty(to_ref, WorkspaceCognitiveGraphError::EmptyExternalRef)?;
        if from_ref == to_ref {
            return Err(WorkspaceCognitiveGraphError::SelfEdge);
        }
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            from_ref,
            to_ref,
            kind,
            explanation: explanation.into(),
            confidence: clamp_score(confidence)?,
            evidence_refs,
            broken,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveGraphError> {
        if self.from_ref == self.to_ref {
            return Err(WorkspaceCognitiveGraphError::SelfEdge);
        }
        clamp_score(self.confidence)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceCognitiveGraphError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }

    /// Dedup key: endpoints + kind (prevents duplicate edges in a snapshot).
    pub fn dedup_key(&self) -> String {
        format!("{}|{}|{}", self.from_ref, self.kind.as_str(), self.to_ref)
    }
}

/// Durable graph snapshot metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphMeta {
    pub id: String,
    pub workspace_id: String,
    pub status: CognitiveGraphStatus,
    pub generated_at: String,
    pub superseded_at: Option<String>,
    pub node_count: usize,
    pub edge_count: usize,
    pub broken_node_count: usize,
    pub broken_edge_count: usize,
    pub authority_effect: String,
}

impl CognitiveGraphMeta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive_graph:";

    pub fn new(
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        node_count: usize,
        edge_count: usize,
        broken_node_count: usize,
        broken_edge_count: usize,
    ) -> Self {
        Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            status: CognitiveGraphStatus::Current,
            generated_at: generated_at.into(),
            superseded_at: None,
            node_count,
            edge_count,
            broken_node_count,
            broken_edge_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = CognitiveGraphStatus::Superseded;
        let at = at.into();
        self.superseded_at = Some(at);
    }
}

/// Full current graph proposal (read-only projection).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphView {
    pub meta: CognitiveGraphMeta,
    pub nodes: Vec<CognitiveGraphNode>,
    pub edges: Vec<CognitiveGraphEdge>,
    pub authority_effect: String,
}

impl CognitiveGraphView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.meta.authority_effect == CognitiveGraphMeta::AUTHORITY_EFFECT_NONE
            && self
                .nodes
                .iter()
                .all(|n| n.authority_effect == CognitiveGraphNode::AUTHORITY_EFFECT_NONE)
            && self
                .edges
                .iter()
                .all(|e| e.authority_effect == CognitiveGraphEdge::AUTHORITY_EFFECT_NONE)
    }
}

/// Terminal graph evidence — never actionable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphHistoryEntry {
    pub snapshot_id: String,
    pub status: String,
    pub generated_at: String,
    pub superseded_at: Option<String>,
    pub node_count: usize,
    pub edge_count: usize,
    pub broken_node_count: usize,
    pub broken_edge_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl CognitiveGraphHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_meta(meta: &CognitiveGraphMeta) -> Option<Self> {
        if !meta.status.is_terminal() {
            return None;
        }
        Some(Self {
            snapshot_id: meta.id.clone(),
            status: meta.status.as_str().into(),
            generated_at: meta.generated_at.clone(),
            superseded_at: meta.superseded_at.clone(),
            node_count: meta.node_count,
            edge_count: meta.edge_count,
            broken_node_count: meta.broken_node_count,
            broken_edge_count: meta.broken_edge_count,
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn is_non_actionable(&self) -> bool {
        self.terminal
            && !self.actionable
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }
}

/// Dual-channel cognitive graph projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<CognitiveGraphView>,
    pub history: Vec<CognitiveGraphHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl CognitiveGraphSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<CognitiveGraphView>,
        history: Vec<CognitiveGraphHistoryEntry>,
        history_count: usize,
        generated_at: impl Into<String>,
    ) -> Self {
        Self {
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            current,
            history,
            history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn is_non_commandable(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.history.iter().all(|h| h.is_non_actionable())
            && self
                .current
                .as_ref()
                .map(|c| c.is_non_executing())
                .unwrap_or(true)
    }

    pub fn summary(&self, history_limit: usize) -> CognitiveGraphSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        CognitiveGraphSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            current_id: self.current.as_ref().map(|c| c.meta.id.clone()),
            node_count: self
                .current
                .as_ref()
                .map(|c| c.nodes.len())
                .unwrap_or(0),
            edge_count: self
                .current
                .as_ref()
                .map(|c| c.edges.len())
                .unwrap_or(0),
            broken_reference_count: self
                .current
                .as_ref()
                .map(|c| {
                    c.nodes.iter().filter(|n| n.broken).count()
                        + c.edges.iter().filter(|e| e.broken).count()
                })
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveGraphSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub current_id: Option<String>,
    pub node_count: usize,
    pub edge_count: usize,
    pub broken_reference_count: usize,
    pub history: Vec<CognitiveGraphHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Deduplicate nodes by external_ref (first wins; prefer non-broken).
pub fn dedupe_nodes(nodes: Vec<CognitiveGraphNode>) -> Vec<CognitiveGraphNode> {
    use std::collections::HashMap;
    let mut map: HashMap<String, CognitiveGraphNode> = HashMap::new();
    for node in nodes {
        match map.get(&node.external_ref) {
            Some(existing) if !existing.broken || node.broken => {}
            _ => {
                map.insert(node.external_ref.clone(), node);
            }
        }
    }
    let mut out: Vec<_> = map.into_values().collect();
    out.sort_by(|a, b| a.external_ref.cmp(&b.external_ref));
    out
}

/// Deduplicate edges by (from, kind, to).
pub fn dedupe_edges(edges: Vec<CognitiveGraphEdge>) -> Vec<CognitiveGraphEdge> {
    use std::collections::HashMap;
    let mut map: HashMap<String, CognitiveGraphEdge> = HashMap::new();
    for edge in edges {
        map.entry(edge.dedup_key()).or_insert(edge);
    }
    let mut out: Vec<_> = map.into_values().collect();
    out.sort_by(|a, b| a.dedup_key().cmp(&b.dedup_key()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_from_current_is_none() {
        let meta = CognitiveGraphMeta::new("ws", "t0", 1, 1, 0, 0);
        assert!(CognitiveGraphHistoryEntry::from_meta(&meta).is_none());
    }

    #[test]
    fn superseded_is_non_actionable_history() {
        let mut meta = CognitiveGraphMeta::new("ws", "t0", 2, 1, 0, 0);
        meta.mark_superseded("t1");
        let entry = CognitiveGraphHistoryEntry::from_meta(&meta).unwrap();
        assert!(entry.is_non_actionable());
    }

    #[test]
    fn snapshot_rejects_commandability() {
        let node = CognitiveGraphNode::new("ws", CognitiveGraphNodeKind::Goal, "cognitive:1", "G", false)
            .unwrap();
        let edge = CognitiveGraphEdge::new(
            "ws",
            "cognitive:1",
            "cognitive:2",
            CognitiveGraphEdgeKind::Supports,
            "explicit cognitive relation",
            70,
            vec!["cognitive_rel:1".into()],
            false,
        )
        .unwrap();
        let meta = CognitiveGraphMeta::new("ws", "t0", 1, 1, 0, 0);
        let view = CognitiveGraphView {
            meta,
            nodes: vec![node],
            edges: vec![edge],
            authority_effect: "none".into(),
        };
        let snap = CognitiveGraphSnapshot::assemble("ws", Some(view), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
        assert!(crate::projection_contract::history_count_is_authoritative(
            snap.history.len(),
            snap.history_count
        ));
    }

    #[test]
    fn duplicate_nodes_and_edges_are_collapsed() {
        let a = CognitiveGraphNode::new("ws", CognitiveGraphNodeKind::Task, "task:1", "T", false)
            .unwrap();
        let b = CognitiveGraphNode::new("ws", CognitiveGraphNodeKind::Task, "task:1", "T2", true)
            .unwrap();
        let nodes = dedupe_nodes(vec![a, b]);
        assert_eq!(nodes.len(), 1);
        assert!(!nodes[0].broken);

        let e1 = CognitiveGraphEdge::new(
            "ws",
            "a",
            "b",
            CognitiveGraphEdgeKind::DependsOn,
            "one",
            50,
            vec![],
            false,
        )
        .unwrap();
        let e2 = CognitiveGraphEdge::new(
            "ws",
            "a",
            "b",
            CognitiveGraphEdgeKind::DependsOn,
            "two",
            50,
            vec![],
            false,
        )
        .unwrap();
        assert_eq!(dedupe_edges(vec![e1, e2]).len(), 1);
    }

    #[test]
    fn self_edges_forbidden() {
        let err = CognitiveGraphEdge::new(
            "ws",
            "x",
            "x",
            CognitiveGraphEdgeKind::RelatesTo,
            "bad",
            50,
            vec![],
            false,
        );
        assert_eq!(err, Err(WorkspaceCognitiveGraphError::SelfEdge));
    }

    #[test]
    fn summary_preserves_history_count() {
        let mut meta = CognitiveGraphMeta::new("ws", "t0", 1, 0, 0, 0);
        meta.mark_superseded("t1");
        let entry = CognitiveGraphHistoryEntry::from_meta(&meta).unwrap();
        let snap = CognitiveGraphSnapshot::assemble("ws", None, vec![entry.clone(), entry], 2, "t2");
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
    }

    #[test]
    fn nodes_are_reference_only_external_refs() {
        let node = CognitiveGraphNode::new(
            "ws",
            CognitiveGraphNodeKind::Plan,
            "planning_plan:abc",
            "Plan",
            false,
        )
        .unwrap();
        assert!(node.external_ref.contains(':'));
        assert_eq!(node.authority_effect, "none");
    }

    #[test]
    fn broken_reference_nodes_are_observable_evidence() {
        let broken = CognitiveGraphNode::new(
            "ws",
            CognitiveGraphNodeKind::BrokenReference,
            "missing:xyz",
            "Unresolved reference missing:xyz",
            true,
        )
        .unwrap();
        assert!(broken.broken);
        assert_eq!(broken.kind, CognitiveGraphNodeKind::BrokenReference);
    }
}
