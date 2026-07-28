//! Workspace Cognitive Orchestration — Programme II Batch 5.
//!
//! Coordination layer above Cognitive Model / Planning / Reasoning / Graph.
//! Owns orchestration state, refresh ordering, dependency scheduling, and
//! orchestration evidence only. Permanently non-executing.
//! Never owns Intent/Task/RE/DE/Execution/Planning/Reasoning/Graph lifecycles.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceCognitiveOrchestrationError {
    #[error("invalid orchestration status: {0}")]
    InvalidStatus(String),

    #[error("invalid artefact kind: {0}")]
    InvalidArtefactKind(String),

    #[error("confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("orchestration artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("orchestration snapshot not found")]
    SnapshotNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspaceCognitiveOrchestrationError> {
    if value > 100 {
        Err(WorkspaceCognitiveOrchestrationError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrchestrationStatus {
    Current,
    Superseded,
    Archived,
}

impl OrchestrationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Superseded => "superseded",
            Self::Archived => "archived",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveOrchestrationError> {
        match value {
            "current" => Ok(Self::Current),
            "superseded" => Ok(Self::Superseded),
            "archived" => Ok(Self::Archived),
            other => Err(WorkspaceCognitiveOrchestrationError::InvalidStatus(
                other.into(),
            )),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Archived)
    }
}

/// Cognitive artefact kinds that orchestration may schedule for refresh consideration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrchestrationArtefactKind {
    CognitiveModel,
    Planning,
    Reasoning,
    CognitiveGraph,
}

impl OrchestrationArtefactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CognitiveModel => "cognitive_model",
            Self::Planning => "planning",
            Self::Reasoning => "reasoning",
            Self::CognitiveGraph => "cognitive_graph",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspaceCognitiveOrchestrationError> {
        match value {
            "cognitive_model" => Ok(Self::CognitiveModel),
            "planning" => Ok(Self::Planning),
            "reasoning" => Ok(Self::Reasoning),
            "cognitive_graph" => Ok(Self::CognitiveGraph),
            other => Err(WorkspaceCognitiveOrchestrationError::InvalidArtefactKind(
                other.into(),
            )),
        }
    }
}

/// Directed dependency: `from` should refresh before `to` (from → to).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationDependency {
    pub from: OrchestrationArtefactKind,
    pub to: OrchestrationArtefactKind,
    pub reason: String,
}

/// Ordered refresh stage — descriptive only, never executed by orchestration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationRefreshStage {
    pub stage: u32,
    pub artefact: OrchestrationArtefactKind,
    pub action: String,
    pub rationale: String,
}

/// Observable staleness / block / skip finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationObservation {
    pub kind: String,
    pub artefact: Option<OrchestrationArtefactKind>,
    pub statement: String,
    pub evidence_refs: Vec<String>,
}

/// Cycle evidence — never invents a resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationCycleDetected {
    pub cycle: Vec<OrchestrationArtefactKind>,
    pub evidence_refs: Vec<String>,
}

/// Reference links to foreign durable artefacts (never payloads).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationEvidenceLink {
    pub external_ref: String,
    pub kind: String,
}

/// Durable orchestration snapshot root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOrchestrationMeta {
    pub orchestration_id: String,
    pub workspace_id: String,
    pub status: OrchestrationStatus,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub current_generation: u64,
    pub uncertainty: u8,
    pub rationale: String,
    pub authority_effect: String,
    pub terminal: bool,
    pub actionable: bool,
}

impl WorkspaceOrchestrationMeta {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "orchestration:";

    pub fn new(
        workspace_id: impl Into<String>,
        created_at: impl Into<String>,
        current_generation: u64,
        uncertainty: u8,
        rationale: impl Into<String>,
    ) -> Result<Self, WorkspaceCognitiveOrchestrationError> {
        Ok(Self {
            orchestration_id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            status: OrchestrationStatus::Current,
            created_at: created_at.into(),
            superseded_at: None,
            current_generation,
            uncertainty: clamp_score(uncertainty)?,
            rationale: rationale.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
            terminal: false,
            actionable: false,
        })
    }

    pub fn mark_superseded(&mut self, at: impl Into<String>) {
        self.status = OrchestrationStatus::Superseded;
        let at = at.into();
        self.superseded_at = Some(at);
        self.terminal = true;
        self.actionable = false;
    }

    pub fn validate(&self) -> Result<(), WorkspaceCognitiveOrchestrationError> {
        clamp_score(self.uncertainty)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspaceCognitiveOrchestrationError::AuthorityEffectMustBeNone);
        }
        if self.actionable {
            return Err(WorkspaceCognitiveOrchestrationError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

/// Full current orchestration view (non-executing).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOrchestrationView {
    pub meta: WorkspaceOrchestrationMeta,
    pub refresh_plan: Vec<OrchestrationRefreshStage>,
    pub dependency_order: Vec<OrchestrationArtefactKind>,
    pub dependencies: Vec<OrchestrationDependency>,
    pub blocked_items: Vec<OrchestrationObservation>,
    pub stale_items: Vec<OrchestrationObservation>,
    pub skipped_items: Vec<OrchestrationObservation>,
    pub cycles: Vec<OrchestrationCycleDetected>,
    pub evidence_links: Vec<OrchestrationEvidenceLink>,
    pub graph_links: Vec<OrchestrationEvidenceLink>,
    pub planning_links: Vec<OrchestrationEvidenceLink>,
    pub reasoning_links: Vec<OrchestrationEvidenceLink>,
    pub execution_links: Vec<OrchestrationEvidenceLink>,
    pub authority_effect: String,
}

impl WorkspaceOrchestrationView {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.meta.authority_effect == WorkspaceOrchestrationMeta::AUTHORITY_EFFECT_NONE
            && !self.meta.actionable
            && !self.meta.terminal
    }
}

/// Terminal orchestration evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestrationHistoryEntry {
    pub orchestration_id: String,
    pub status: String,
    pub created_at: String,
    pub superseded_at: Option<String>,
    pub current_generation: u64,
    pub uncertainty: u8,
    pub rationale_excerpt: String,
    pub stage_count: usize,
    pub cycle_count: usize,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl OrchestrationHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_view(view: &WorkspaceOrchestrationView) -> Option<Self> {
        if !view.meta.status.is_terminal() {
            return None;
        }
        let excerpt: String = view.meta.rationale.chars().take(200).collect();
        Some(Self {
            orchestration_id: view.meta.orchestration_id.clone(),
            status: view.meta.status.as_str().into(),
            created_at: view.meta.created_at.clone(),
            superseded_at: view.meta.superseded_at.clone(),
            current_generation: view.meta.current_generation,
            uncertainty: view.meta.uncertainty,
            rationale_excerpt: excerpt,
            stage_count: view.refresh_plan.len(),
            cycle_count: view.cycles.len(),
            terminal: true,
            actionable: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn from_meta(
        meta: &WorkspaceOrchestrationMeta,
        stage_count: usize,
        cycle_count: usize,
    ) -> Option<Self> {
        if !meta.status.is_terminal() {
            return None;
        }
        let excerpt: String = meta.rationale.chars().take(200).collect();
        Some(Self {
            orchestration_id: meta.orchestration_id.clone(),
            status: meta.status.as_str().into(),
            created_at: meta.created_at.clone(),
            superseded_at: meta.superseded_at.clone(),
            current_generation: meta.current_generation,
            uncertainty: meta.uncertainty,
            rationale_excerpt: excerpt,
            stage_count,
            cycle_count,
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

/// Dual-channel orchestration projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOrchestrationSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    pub current: Option<WorkspaceOrchestrationView>,
    pub history: Vec<OrchestrationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl WorkspaceOrchestrationSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<WorkspaceOrchestrationView>,
        history: Vec<OrchestrationHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> WorkspaceOrchestrationSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        WorkspaceOrchestrationSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_current: self.current.is_some(),
            orchestration_id: self
                .current
                .as_ref()
                .map(|c| c.meta.orchestration_id.clone()),
            current_generation: self.current.as_ref().map(|c| c.meta.current_generation),
            stage_count: self
                .current
                .as_ref()
                .map(|c| c.refresh_plan.len())
                .unwrap_or(0),
            stale_count: self
                .current
                .as_ref()
                .map(|c| c.stale_items.len())
                .unwrap_or(0),
            blocked_count: self
                .current
                .as_ref()
                .map(|c| c.blocked_items.len())
                .unwrap_or(0),
            cycle_count: self
                .current
                .as_ref()
                .map(|c| c.cycles.len())
                .unwrap_or(0),
            uncertainty: self.current.as_ref().map(|c| c.meta.uncertainty),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOrchestrationSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_current: bool,
    pub orchestration_id: Option<String>,
    pub current_generation: Option<u64>,
    pub stage_count: usize,
    pub stale_count: usize,
    pub blocked_count: usize,
    pub cycle_count: usize,
    pub uncertainty: Option<u8>,
    pub history: Vec<OrchestrationHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

/// Result of topological ordering over orchestration dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrchestrationOrderResult {
    pub order: Vec<OrchestrationArtefactKind>,
    pub cycles: Vec<OrchestrationCycleDetected>,
}

/// Topological sort. On cycles: do not invent a resolution — emit CycleDetected evidence.
pub fn order_dependencies(
    nodes: &[OrchestrationArtefactKind],
    deps: &[OrchestrationDependency],
) -> OrchestrationOrderResult {
    let node_set: HashSet<_> = nodes.iter().copied().collect();
    let mut indegree: HashMap<OrchestrationArtefactKind, usize> =
        nodes.iter().map(|n| (*n, 0usize)).collect();
    let mut adj: HashMap<OrchestrationArtefactKind, Vec<OrchestrationArtefactKind>> =
        HashMap::new();

    for d in deps {
        if !node_set.contains(&d.from) || !node_set.contains(&d.to) {
            continue;
        }
        if d.from == d.to {
            continue;
        }
        adj.entry(d.from).or_default().push(d.to);
        *indegree.entry(d.to).or_insert(0) += 1;
    }

    let mut queue: VecDeque<_> = indegree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(k, _)| *k)
        .collect();
    // Deterministic: sort queue by artefact name
    let mut qvec: Vec<_> = queue.drain(..).collect();
    qvec.sort_by_key(|k| k.as_str());
    queue.extend(qvec);

    let mut order = Vec::new();
    while let Some(n) = queue.pop_front() {
        order.push(n);
        if let Some(nexts) = adj.get(&n) {
            let mut nexts = nexts.clone();
            nexts.sort_by_key(|k| k.as_str());
            for m in nexts {
                if let Some(deg) = indegree.get_mut(&m) {
                    *deg = deg.saturating_sub(1);
                    if *deg == 0 {
                        queue.push_back(m);
                    }
                }
            }
        }
    }

    let mut cycles = Vec::new();
    if order.len() != nodes.len() {
        let remaining: Vec<_> = nodes
            .iter()
            .copied()
            .filter(|n| !order.contains(n))
            .collect();
        // Emit one CycleDetected observation with remaining nodes (no invented break).
        cycles.push(OrchestrationCycleDetected {
            cycle: remaining,
            evidence_refs: deps
                .iter()
                .map(|d| format!("{}->{}", d.from.as_str(), d.to.as_str()))
                .collect(),
        });
    }

    OrchestrationOrderResult { order, cycles }
}

/// Build default refresh stages from a dependency order (descriptive only).
pub fn refresh_stages_from_order(
    order: &[OrchestrationArtefactKind],
) -> Vec<OrchestrationRefreshStage> {
    order
        .iter()
        .enumerate()
        .map(|(i, artefact)| OrchestrationRefreshStage {
            stage: (i as u32) + 1,
            artefact: *artefact,
            action: format!("Refresh {}", artefact.as_str()),
            rationale: format!(
                "Stage {} considers regenerating {} based on dependency order — orchestration does not execute this refresh.",
                i + 1,
                artefact.as_str()
            ),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_ordering_linear() {
        let nodes = [
            OrchestrationArtefactKind::CognitiveModel,
            OrchestrationArtefactKind::Planning,
            OrchestrationArtefactKind::Reasoning,
            OrchestrationArtefactKind::CognitiveGraph,
        ];
        let deps = vec![
            OrchestrationDependency {
                from: OrchestrationArtefactKind::CognitiveModel,
                to: OrchestrationArtefactKind::Planning,
                reason: "planning consumes cognitive model".into(),
            },
            OrchestrationDependency {
                from: OrchestrationArtefactKind::Planning,
                to: OrchestrationArtefactKind::Reasoning,
                reason: "reasoning cites planning".into(),
            },
            OrchestrationDependency {
                from: OrchestrationArtefactKind::Reasoning,
                to: OrchestrationArtefactKind::CognitiveGraph,
                reason: "graph projects reasoning".into(),
            },
        ];
        let result = order_dependencies(&nodes, &deps);
        assert!(result.cycles.is_empty());
        assert_eq!(
            result.order,
            vec![
                OrchestrationArtefactKind::CognitiveModel,
                OrchestrationArtefactKind::Planning,
                OrchestrationArtefactKind::Reasoning,
                OrchestrationArtefactKind::CognitiveGraph,
            ]
        );
        let stages = refresh_stages_from_order(&result.order);
        assert_eq!(stages.len(), 4);
        assert_eq!(stages[0].stage, 1);
    }

    #[test]
    fn cycle_detection_emits_evidence_without_resolution() {
        let nodes = [
            OrchestrationArtefactKind::Planning,
            OrchestrationArtefactKind::Reasoning,
        ];
        let deps = vec![
            OrchestrationDependency {
                from: OrchestrationArtefactKind::Planning,
                to: OrchestrationArtefactKind::Reasoning,
                reason: "a".into(),
            },
            OrchestrationDependency {
                from: OrchestrationArtefactKind::Reasoning,
                to: OrchestrationArtefactKind::Planning,
                reason: "b".into(),
            },
        ];
        let result = order_dependencies(&nodes, &deps);
        assert!(!result.cycles.is_empty());
        assert!(result.order.len() < nodes.len());
        assert_eq!(result.cycles[0].cycle.len(), 2);
    }

    #[test]
    fn history_from_current_is_none() {
        let meta =
            WorkspaceOrchestrationMeta::new("ws", "t0", 1, 40, "coordinate refresh").unwrap();
        let view = WorkspaceOrchestrationView {
            meta,
            refresh_plan: vec![],
            dependency_order: vec![],
            dependencies: vec![],
            blocked_items: vec![],
            stale_items: vec![],
            skipped_items: vec![],
            cycles: vec![],
            evidence_links: vec![],
            graph_links: vec![],
            planning_links: vec![],
            reasoning_links: vec![],
            execution_links: vec![],
            authority_effect: "none".into(),
        };
        assert!(OrchestrationHistoryEntry::from_view(&view).is_none());
        assert!(view.is_non_executing());
    }

    #[test]
    fn superseded_is_non_actionable_history() {
        let mut meta =
            WorkspaceOrchestrationMeta::new("ws", "t0", 1, 40, "coordinate refresh").unwrap();
        meta.mark_superseded("t1");
        let entry = OrchestrationHistoryEntry::from_meta(&meta, 3, 0).unwrap();
        assert!(entry.is_non_actionable());
    }

    #[test]
    fn snapshot_rejects_commandability() {
        let meta =
            WorkspaceOrchestrationMeta::new("ws", "t0", 2, 35, "refresh ordering").unwrap();
        let view = WorkspaceOrchestrationView {
            meta,
            refresh_plan: refresh_stages_from_order(&[
                OrchestrationArtefactKind::CognitiveModel,
                OrchestrationArtefactKind::Planning,
            ]),
            dependency_order: vec![
                OrchestrationArtefactKind::CognitiveModel,
                OrchestrationArtefactKind::Planning,
            ],
            dependencies: vec![],
            blocked_items: vec![],
            stale_items: vec![],
            skipped_items: vec![],
            cycles: vec![],
            evidence_links: vec![],
            graph_links: vec![],
            planning_links: vec![],
            reasoning_links: vec![],
            execution_links: vec![],
            authority_effect: "none".into(),
        };
        let snap = WorkspaceOrchestrationSnapshot::assemble("ws", Some(view), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
        assert!(crate::projection_contract::history_count_is_authoritative(
            snap.history.len(),
            snap.history_count
        ));
    }

    #[test]
    fn summary_preserves_history_count() {
        let mut meta =
            WorkspaceOrchestrationMeta::new("ws", "t0", 1, 50, "old").unwrap();
        meta.mark_superseded("t1");
        let entry = OrchestrationHistoryEntry::from_meta(&meta, 2, 0).unwrap();
        let snap = WorkspaceOrchestrationSnapshot::assemble(
            "ws",
            None,
            vec![entry.clone(), entry],
            2,
            "t2",
        );
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
        assert!(!summary.has_current);
    }
}
