//! Workspace Planning Engine — Programme II Batch 2.
//!
//! Higher-order, durable, permanently non-executing planning artefacts.
//! Composes Cognitive Model, Intent, Task Graph, RE/DE, Attention, Purpose,
//! and Memory references. Never owns those systems' lifecycles.
//!
//! Planning owns sequencing, decomposition, ordering, rationale, assumptions,
//! and uncertainty — not Intent, Task, Execution, Permissions, RE, or DE.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspacePlanningError {
    #[error("title must not be empty")]
    EmptyTitle,

    #[error("invalid plan status: {0}")]
    InvalidStatus(String),

    #[error("confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("planning artefacts must have authority_effect none")]
    AuthorityEffectMustBeNone,

    #[error("plan not found")]
    PlanNotFound,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, WorkspacePlanningError> {
    if value > 100 {
        Err(WorkspacePlanningError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

fn normalize_title(title: impl Into<String>) -> Result<String, WorkspacePlanningError> {
    let trimmed = title.into().trim().to_string();
    if trimmed.is_empty() {
        Err(WorkspacePlanningError::EmptyTitle)
    } else {
        Ok(trimmed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanningPlanStatus {
    Active,
    Superseded,
    Abandoned,
}

impl PlanningPlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Superseded => "superseded",
            Self::Abandoned => "abandoned",
        }
    }

    pub fn parse(value: &str) -> Result<Self, WorkspacePlanningError> {
        match value {
            "active" => Ok(Self::Active),
            "superseded" => Ok(Self::Superseded),
            "abandoned" => Ok(Self::Abandoned),
            other => Err(WorkspacePlanningError::InvalidStatus(other.into())),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Superseded | Self::Abandoned)
    }
}

/// Durable planning plan (sequencing artefact — never executable).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningPlan {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub summary: String,
    pub status: PlanningPlanStatus,
    pub confidence: u8,
    pub uncertainty: u8,
    pub generated_at: String,
    pub superseded_at: Option<String>,
    pub authority_effect: String,
}

impl PlanningPlan {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_plan:";

    pub fn new(
        workspace_id: impl Into<String>,
        title: impl Into<String>,
        summary: impl Into<String>,
        confidence: u8,
        uncertainty: u8,
        generated_at: impl Into<String>,
    ) -> Result<Self, WorkspacePlanningError> {
        let title = normalize_title(title)?;
        let confidence = clamp_score(confidence)?;
        let uncertainty = clamp_score(uncertainty)?;
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            title,
            summary: summary.into().trim().to_string(),
            status: PlanningPlanStatus::Active,
            confidence,
            uncertainty,
            generated_at: generated_at.into(),
            superseded_at: None,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate(&self) -> Result<(), WorkspacePlanningError> {
        normalize_title(&self.title)?;
        clamp_score(self.confidence)?;
        clamp_score(self.uncertainty)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(WorkspacePlanningError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningStep {
    pub id: String,
    pub plan_id: String,
    pub workspace_id: String,
    pub ordinal: u32,
    pub title: String,
    pub rationale: String,
    /// Reference-only links: `cognitive:…`, `work_goal:…`, `task:…`, etc.
    pub evidence_refs: Vec<String>,
    pub confidence: u8,
    pub uncertainty: u8,
    pub authority_effect: String,
}

impl PlanningStep {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_step:";

    pub fn new(
        plan_id: impl Into<String>,
        workspace_id: impl Into<String>,
        ordinal: u32,
        title: impl Into<String>,
        rationale: impl Into<String>,
        evidence_refs: Vec<String>,
        confidence: u8,
        uncertainty: u8,
    ) -> Result<Self, WorkspacePlanningError> {
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            plan_id: plan_id.into(),
            workspace_id: workspace_id.into(),
            ordinal,
            title: normalize_title(title)?,
            rationale: rationale.into(),
            evidence_refs,
            confidence: clamp_score(confidence)?,
            uncertainty: clamp_score(uncertainty)?,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningAssumption {
    pub id: String,
    pub plan_id: String,
    pub workspace_id: String,
    pub statement: String,
    pub confidence: u8,
    pub evidence_refs: Vec<String>,
    pub authority_effect: String,
}

impl PlanningAssumption {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_assumption:";

    pub fn new(
        plan_id: impl Into<String>,
        workspace_id: impl Into<String>,
        statement: impl Into<String>,
        confidence: u8,
        evidence_refs: Vec<String>,
    ) -> Result<Self, WorkspacePlanningError> {
        let statement = normalize_title(statement)?;
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            plan_id: plan_id.into(),
            workspace_id: workspace_id.into(),
            statement,
            confidence: clamp_score(confidence)?,
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningRisk {
    pub id: String,
    pub plan_id: String,
    pub workspace_id: String,
    pub statement: String,
    pub uncertainty: u8,
    pub evidence_refs: Vec<String>,
    pub authority_effect: String,
}

impl PlanningRisk {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_risk:";

    pub fn new(
        plan_id: impl Into<String>,
        workspace_id: impl Into<String>,
        statement: impl Into<String>,
        uncertainty: u8,
        evidence_refs: Vec<String>,
    ) -> Result<Self, WorkspacePlanningError> {
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            plan_id: plan_id.into(),
            workspace_id: workspace_id.into(),
            statement: normalize_title(statement)?,
            uncertainty: clamp_score(uncertainty)?,
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningGap {
    pub id: String,
    pub plan_id: String,
    pub workspace_id: String,
    pub statement: String,
    pub evidence_refs: Vec<String>,
    pub authority_effect: String,
}

impl PlanningGap {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_gap:";

    pub fn new(
        plan_id: impl Into<String>,
        workspace_id: impl Into<String>,
        statement: impl Into<String>,
        evidence_refs: Vec<String>,
    ) -> Result<Self, WorkspacePlanningError> {
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            plan_id: plan_id.into(),
            workspace_id: workspace_id.into(),
            statement: normalize_title(statement)?,
            evidence_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningDependency {
    pub id: String,
    pub plan_id: String,
    pub workspace_id: String,
    pub from_step_id: String,
    pub to_step_id: String,
    pub kind: String,
    pub authority_effect: String,
}

impl PlanningDependency {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_dep:";

    pub fn new(
        plan_id: impl Into<String>,
        workspace_id: impl Into<String>,
        from_step_id: impl Into<String>,
        to_step_id: impl Into<String>,
        kind: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            plan_id: plan_id.into(),
            workspace_id: workspace_id.into(),
            from_step_id: from_step_id.into(),
            to_step_id: to_step_id.into(),
            kind: kind.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningExplanation {
    pub summary: String,
    pub why: String,
    pub next: String,
}

/// Reference to a constraint owned elsewhere (cognitive constraint, policy, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningConstraintReference {
    pub external_ref: String,
    pub note: String,
}

/// Typed evidence pointer — never duplicates canonical payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningEvidenceReference {
    pub external_ref: String,
    pub kind: String,
}

/// Optional alternate sequencing (advisory only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningAlternative {
    pub id: String,
    pub title: String,
    pub rationale: String,
    pub step_refs: Vec<String>,
    pub authority_effect: String,
}

impl PlanningAlternative {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_alt:";

    pub fn new(
        title: impl Into<String>,
        rationale: impl Into<String>,
        step_refs: Vec<String>,
    ) -> Result<Self, WorkspacePlanningError> {
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            title: normalize_title(title)?,
            rationale: rationale.into(),
            step_refs,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Logical grouping of steps within a plan (presentation / rationale only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningSection {
    pub id: String,
    pub title: String,
    pub step_ids: Vec<String>,
    pub authority_effect: String,
}

impl PlanningSection {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "planning_section:";

    pub fn new(title: impl Into<String>, step_ids: Vec<String>) -> Result<Self, WorkspacePlanningError> {
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            title: normalize_title(title)?,
            step_ids,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }
}

/// Aggregate confidence / uncertainty for a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningConfidence {
    pub confidence: u8,
    pub uncertainty: u8,
}

impl PlanningConfidence {
    /// Propagate from step scores: confidence = mean, uncertainty = max.
    pub fn from_steps(steps: &[PlanningStep]) -> Self {
        if steps.is_empty() {
            return Self {
                confidence: 50,
                uncertainty: 50,
            };
        }
        let conf_sum: u32 = steps.iter().map(|s| u32::from(s.confidence)).sum();
        let confidence = (conf_sum / steps.len() as u32) as u8;
        let uncertainty = steps.iter().map(|s| s.uncertainty).max().unwrap_or(50);
        Self {
            confidence,
            uncertainty,
        }
    }
}

/// Returns true when planning dependencies form a cycle.
pub fn planning_dependency_has_cycle(deps: &[PlanningDependency]) -> bool {
    use std::collections::{HashMap, HashSet};
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for d in deps {
        adj.entry(d.from_step_id.as_str())
            .or_default()
            .push(d.to_step_id.as_str());
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    fn dfs<'a>(
        node: &'a str,
        adj: &HashMap<&str, Vec<&'a str>>,
        visiting: &mut HashSet<&'a str>,
        visited: &mut HashSet<&'a str>,
    ) -> bool {
        if visited.contains(node) {
            return false;
        }
        if !visiting.insert(node) {
            return true;
        }
        if let Some(nexts) = adj.get(node) {
            for n in nexts {
                if dfs(n, adj, visiting, visited) {
                    return true;
                }
            }
        }
        visiting.remove(node);
        visited.insert(node);
        false
    }
    for node in adj.keys() {
        if dfs(node, &adj, &mut visiting, &mut visited) {
            return true;
        }
    }
    false
}

/// Terminal planning evidence — never actionable / never executable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningHistoryEntry {
    pub plan_id: String,
    pub title: String,
    pub status: String,
    pub confidence: u8,
    pub uncertainty: u8,
    pub generated_at: String,
    pub superseded_at: Option<String>,
    pub terminal: bool,
    pub actionable: bool,
    pub authority_effect: String,
}

impl PlanningHistoryEntry {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn from_plan(plan: &PlanningPlan) -> Option<Self> {
        if !plan.status.is_terminal() {
            return None;
        }
        Some(Self {
            plan_id: plan.id.clone(),
            title: plan.title.clone(),
            status: plan.status.as_str().into(),
            confidence: plan.confidence,
            uncertainty: plan.uncertainty,
            generated_at: plan.generated_at.clone(),
            superseded_at: plan.superseded_at.clone(),
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

/// Actionable current plan view (still non-executing — no command fields).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningProposal {
    pub plan: PlanningPlan,
    pub steps: Vec<PlanningStep>,
    pub sections: Vec<PlanningSection>,
    pub assumptions: Vec<PlanningAssumption>,
    pub risks: Vec<PlanningRisk>,
    pub gaps: Vec<PlanningGap>,
    pub dependencies: Vec<PlanningDependency>,
    pub alternatives: Vec<PlanningAlternative>,
    pub constraint_refs: Vec<PlanningConstraintReference>,
    pub evidence_refs: Vec<PlanningEvidenceReference>,
    pub explanation: PlanningExplanation,
    pub authority_effect: String,
}

impl PlanningProposal {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn is_non_executing(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self.plan.authority_effect == PlanningPlan::AUTHORITY_EFFECT_NONE
            && self
                .steps
                .iter()
                .all(|s| s.authority_effect == PlanningStep::AUTHORITY_EFFECT_NONE)
            && !planning_dependency_has_cycle(&self.dependencies)
    }
}

/// Dual-channel planning projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningSnapshot {
    pub workspace_id: String,
    pub generated_at: String,
    /// Current active proposal (at most one).
    pub current: Option<PlanningProposal>,
    pub history: Vec<PlanningHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

impl PlanningSnapshot {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        current: Option<PlanningProposal>,
        history: Vec<PlanningHistoryEntry>,
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

    pub fn summary(&self, history_limit: usize) -> PlanningSummary {
        let history: Vec<_> = self.history.iter().take(history_limit).cloned().collect();
        PlanningSummary {
            workspace_id: self.workspace_id.clone(),
            generated_at: self.generated_at.clone(),
            has_active_plan: self.current.is_some(),
            active_plan_id: self.current.as_ref().map(|c| c.plan.id.clone()),
            active_title: self.current.as_ref().map(|c| c.plan.title.clone()),
            step_count: self
                .current
                .as_ref()
                .map(|c| c.steps.len())
                .unwrap_or(0),
            history,
            history_count: self.history_count,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }
}

/// Compact projection summary for UI / IPC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanningSummary {
    pub workspace_id: String,
    pub generated_at: String,
    pub has_active_plan: bool,
    pub active_plan_id: Option<String>,
    pub active_title: Option<String>,
    pub step_count: usize,
    pub history: Vec<PlanningHistoryEntry>,
    pub history_count: usize,
    pub authority_effect: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_from_active_plan_is_none() {
        let plan = PlanningPlan::new("ws", "Plan", "summary", 70, 30, "t0").unwrap();
        assert!(PlanningHistoryEntry::from_plan(&plan).is_none());
    }

    #[test]
    fn superseded_plan_is_non_actionable_history() {
        let mut plan = PlanningPlan::new("ws", "Plan", "summary", 70, 30, "t0").unwrap();
        plan.status = PlanningPlanStatus::Superseded;
        plan.superseded_at = Some("t1".into());
        let entry = PlanningHistoryEntry::from_plan(&plan).unwrap();
        assert!(entry.is_non_actionable());
        assert!(!entry.actionable);
    }

    #[test]
    fn snapshot_rejects_commandability() {
        let plan = PlanningPlan::new("ws", "Plan", "Do X then Y", 80, 20, "t0").unwrap();
        let step = PlanningStep::new(
            &plan.id,
            "ws",
            0,
            "Step 1",
            "Because objective is focused",
            vec!["cognitive:obj".into()],
            75,
            25,
        )
        .unwrap();
        let proposal = PlanningProposal {
            plan,
            steps: vec![step],
            sections: vec![],
            assumptions: vec![],
            risks: vec![],
            gaps: vec![],
            dependencies: vec![],
            alternatives: vec![],
            constraint_refs: vec![],
            evidence_refs: vec![],
            explanation: PlanningExplanation {
                summary: "Ordered work from cognitive focus".into(),
                why: "Focus + objectives drive sequencing".into(),
                next: "Review proposal; execution starts elsewhere".into(),
            },
            authority_effect: "none".into(),
        };
        let snap = PlanningSnapshot::assemble("ws", Some(proposal), vec![], 0, "t0");
        assert!(snap.is_non_commandable());
        assert!(crate::projection_contract::history_count_is_authoritative(
            snap.history.len(),
            snap.history_count
        ));
    }

    #[test]
    fn confidence_propagates_from_steps() {
        let plan = PlanningPlan::new("ws", "Plan", "s", 50, 50, "t0").unwrap();
        let a = PlanningStep::new(&plan.id, "ws", 0, "A", "r", vec![], 80, 20).unwrap();
        let b = PlanningStep::new(&plan.id, "ws", 1, "B", "r", vec![], 60, 40).unwrap();
        let conf = PlanningConfidence::from_steps(&[a, b]);
        assert_eq!(conf.confidence, 70);
        assert_eq!(conf.uncertainty, 40);
    }

    #[test]
    fn dependency_cycle_detected() {
        let d1 = PlanningDependency::new("p", "ws", "s1", "s2", "before");
        let d2 = PlanningDependency::new("p", "ws", "s2", "s1", "before");
        assert!(planning_dependency_has_cycle(&[d1, d2]));
        let linear = PlanningDependency::new("p", "ws", "s1", "s2", "before");
        assert!(!planning_dependency_has_cycle(&[linear]));
    }

    #[test]
    fn summary_preserves_history_count_under_window() {
        let mut plan = PlanningPlan::new("ws", "Old", "s", 50, 50, "t0").unwrap();
        plan.status = PlanningPlanStatus::Superseded;
        plan.superseded_at = Some("t1".into());
        let entry = PlanningHistoryEntry::from_plan(&plan).unwrap();
        let snap = PlanningSnapshot::assemble("ws", None, vec![entry.clone(), entry], 2, "t2");
        let summary = snap.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 2);
        assert!(!summary.has_active_plan);
    }
}
