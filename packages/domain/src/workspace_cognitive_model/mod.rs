//! Workspace Cognitive Model — Programme II Batch 1.
//!
//! Durable semantic layer for objectives, relationships, importance, confidence,
//! uncertainty, and focus. Not a projection aggregator. Never executes.
//!
//! WorkGoal / Project / Task payloads remain owned by WorkspaceIntentService.
//! Goal nodes *reference* WorkGoals via `external_ref`.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::errors::DomainError;

/// Cognitive model validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CognitiveModelError {
    #[error("title must not be empty")]
    EmptyTitle,

    #[error("invalid node kind: {0}")]
    InvalidNodeKind(String),

    #[error("invalid relation kind: {0}")]
    InvalidRelationKind(String),

    #[error("invalid status: {0}")]
    InvalidStatus(String),

    #[error("importance/confidence/uncertainty must be 0..=100 (got {0})")]
    InvalidScore(u8),

    #[error("goal nodes must reference a work_goal via external_ref")]
    GoalRequiresWorkGoalRef,

    #[error("relation endpoints must not be empty")]
    EmptyRelationEndpoint,

    #[error("self-relations are forbidden")]
    SelfRelation,

    #[error("authority_effect must be none")]
    AuthorityEffectMustBeNone,

    #[error(transparent)]
    Domain(#[from] DomainError),
}

fn clamp_score(value: u8) -> Result<u8, CognitiveModelError> {
    if value > 100 {
        Err(CognitiveModelError::InvalidScore(value))
    } else {
        Ok(value)
    }
}

fn normalize_title(title: impl Into<String>) -> Result<String, CognitiveModelError> {
    let trimmed = title.into().trim().to_string();
    if trimmed.is_empty() {
        Err(CognitiveModelError::EmptyTitle)
    } else {
        Ok(trimmed)
    }
}

/// Semantic node kinds in the cognitive layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveNodeKind {
    Goal,
    Objective,
    Initiative,
    Milestone,
    Context,
    WorkingSet,
    Constraint,
    Risk,
    Opportunity,
}

impl CognitiveNodeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Goal => "goal",
            Self::Objective => "objective",
            Self::Initiative => "initiative",
            Self::Milestone => "milestone",
            Self::Context => "context",
            Self::WorkingSet => "working_set",
            Self::Constraint => "constraint",
            Self::Risk => "risk",
            Self::Opportunity => "opportunity",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CognitiveModelError> {
        match value {
            "goal" => Ok(Self::Goal),
            "objective" => Ok(Self::Objective),
            "initiative" => Ok(Self::Initiative),
            "milestone" => Ok(Self::Milestone),
            "context" => Ok(Self::Context),
            "working_set" => Ok(Self::WorkingSet),
            "constraint" => Ok(Self::Constraint),
            "risk" => Ok(Self::Risk),
            "opportunity" => Ok(Self::Opportunity),
            other => Err(CognitiveModelError::InvalidNodeKind(other.into())),
        }
    }
}

/// Lifecycle of a cognitive node (semantic understanding — not execution).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveNodeStatus {
    Active,
    Paused,
    Completed,
    Abandoned,
}

impl CognitiveNodeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Abandoned => "abandoned",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CognitiveModelError> {
        match value {
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "completed" => Ok(Self::Completed),
            "abandoned" => Ok(Self::Abandoned),
            other => Err(CognitiveModelError::InvalidStatus(other.into())),
        }
    }
}

/// Relation kinds between cognitive nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CognitiveRelationKind {
    DependsOn,
    Blocks,
    Supports,
    Constrains,
    Mitigates,
    Enables,
    PartOf,
    Focuses,
}

impl CognitiveRelationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DependsOn => "depends_on",
            Self::Blocks => "blocks",
            Self::Supports => "supports",
            Self::Constrains => "constrains",
            Self::Mitigates => "mitigates",
            Self::Enables => "enables",
            Self::PartOf => "part_of",
            Self::Focuses => "focuses",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CognitiveModelError> {
        match value {
            "depends_on" => Ok(Self::DependsOn),
            "blocks" => Ok(Self::Blocks),
            "supports" => Ok(Self::Supports),
            "constrains" => Ok(Self::Constrains),
            "mitigates" => Ok(Self::Mitigates),
            "enables" => Ok(Self::Enables),
            "part_of" => Ok(Self::PartOf),
            "focuses" => Ok(Self::Focuses),
            other => Err(CognitiveModelError::InvalidRelationKind(other.into())),
        }
    }
}

/// Durable cognitive semantic node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveNode {
    pub id: String,
    pub workspace_id: String,
    pub kind: CognitiveNodeKind,
    pub title: String,
    pub description: Option<String>,
    pub status: CognitiveNodeStatus,
    pub importance: u8,
    pub confidence: u8,
    pub uncertainty: u8,
    pub is_current_focus: bool,
    /// External SoT reference, e.g. `work_goal:<id>`, `project:<id>`, `task_graph:<id>`.
    pub external_ref: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub authority_effect: String,
}

impl CognitiveNode {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive:";

    pub fn new(
        workspace_id: impl Into<String>,
        kind: CognitiveNodeKind,
        title: impl Into<String>,
        description: Option<String>,
        importance: u8,
        confidence: u8,
        uncertainty: u8,
        external_ref: Option<String>,
        parent_id: Option<String>,
        now: impl Into<String>,
    ) -> Result<Self, CognitiveModelError> {
        let title = normalize_title(title)?;
        let importance = clamp_score(importance)?;
        let confidence = clamp_score(confidence)?;
        let uncertainty = clamp_score(uncertainty)?;
        let external_ref = external_ref.and_then(|v| {
            let t = v.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        });
        if kind == CognitiveNodeKind::Goal {
            let valid = external_ref
                .as_deref()
                .is_some_and(|r| r.starts_with("work_goal:"));
            if !valid {
                return Err(CognitiveModelError::GoalRequiresWorkGoalRef);
            }
        }
        let now = now.into();
        let id = format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4());
        Ok(Self {
            id,
            workspace_id: workspace_id.into(),
            kind,
            title,
            description: description.and_then(|d| {
                let t = d.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            }),
            status: CognitiveNodeStatus::Active,
            importance,
            confidence,
            uncertainty,
            is_current_focus: false,
            external_ref,
            parent_id: parent_id.and_then(|p| {
                let t = p.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            }),
            created_at: now.clone(),
            updated_at: now,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate(&self) -> Result<(), CognitiveModelError> {
        normalize_title(&self.title)?;
        clamp_score(self.importance)?;
        clamp_score(self.confidence)?;
        clamp_score(self.uncertainty)?;
        if self.kind == CognitiveNodeKind::Goal {
            let valid = self
                .external_ref
                .as_deref()
                .is_some_and(|r| r.starts_with("work_goal:"));
            if !valid {
                return Err(CognitiveModelError::GoalRequiresWorkGoalRef);
            }
        }
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(CognitiveModelError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

/// Durable cognitive relation (dependency / support / constraint edge).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveRelation {
    pub id: String,
    pub workspace_id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: CognitiveRelationKind,
    pub confidence: u8,
    pub explanation: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub authority_effect: String,
}

impl CognitiveRelation {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";
    pub const ID_PREFIX: &'static str = "cognitive_rel:";

    pub fn new(
        workspace_id: impl Into<String>,
        from_id: impl Into<String>,
        to_id: impl Into<String>,
        kind: CognitiveRelationKind,
        confidence: u8,
        explanation: Option<String>,
        now: impl Into<String>,
    ) -> Result<Self, CognitiveModelError> {
        let from_id = from_id.into().trim().to_string();
        let to_id = to_id.into().trim().to_string();
        if from_id.is_empty() || to_id.is_empty() {
            return Err(CognitiveModelError::EmptyRelationEndpoint);
        }
        if from_id == to_id {
            return Err(CognitiveModelError::SelfRelation);
        }
        let confidence = clamp_score(confidence)?;
        let now = now.into();
        Ok(Self {
            id: format!("{}{}", Self::ID_PREFIX, uuid::Uuid::new_v4()),
            workspace_id: workspace_id.into(),
            from_id,
            to_id,
            kind,
            confidence,
            explanation: explanation.and_then(|e| {
                let t = e.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            }),
            created_at: now.clone(),
            updated_at: now,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        })
    }

    pub fn validate(&self) -> Result<(), CognitiveModelError> {
        if self.from_id.trim().is_empty() || self.to_id.trim().is_empty() {
            return Err(CognitiveModelError::EmptyRelationEndpoint);
        }
        if self.from_id == self.to_id {
            return Err(CognitiveModelError::SelfRelation);
        }
        clamp_score(self.confidence)?;
        if self.authority_effect != Self::AUTHORITY_EFFECT_NONE {
            return Err(CognitiveModelError::AuthorityEffectMustBeNone);
        }
        Ok(())
    }
}

/// Owner-assembled semantic snapshot for consumers (Attention, Purpose, Planner, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CognitiveModelState {
    pub workspace_id: String,
    pub nodes: Vec<CognitiveNode>,
    pub relations: Vec<CognitiveRelation>,
    pub current_focus_ids: Vec<String>,
    pub generated_at: String,
    pub authority_effect: String,
}

impl CognitiveModelState {
    pub const AUTHORITY_EFFECT_NONE: &'static str = "none";

    pub fn assemble(
        workspace_id: impl Into<String>,
        nodes: Vec<CognitiveNode>,
        relations: Vec<CognitiveRelation>,
        generated_at: impl Into<String>,
    ) -> Self {
        let current_focus_ids = nodes
            .iter()
            .filter(|n| n.is_current_focus)
            .map(|n| n.id.clone())
            .collect();
        Self {
            workspace_id: workspace_id.into(),
            nodes,
            relations,
            current_focus_ids,
            generated_at: generated_at.into(),
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    /// Snapshot never becomes a command surface.
    pub fn is_non_commandable(&self) -> bool {
        self.authority_effect == Self::AUTHORITY_EFFECT_NONE
            && self
                .nodes
                .iter()
                .all(|n| n.authority_effect == CognitiveNode::AUTHORITY_EFFECT_NONE)
            && self
                .relations
                .iter()
                .all(|r| r.authority_effect == CognitiveRelation::AUTHORITY_EFFECT_NONE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_requires_work_goal_ref() {
        let err = CognitiveNode::new(
            "ws",
            CognitiveNodeKind::Goal,
            "Ship Programme II",
            None,
            80,
            70,
            40,
            None,
            None,
            "t0",
        );
        assert_eq!(err, Err(CognitiveModelError::GoalRequiresWorkGoalRef));
    }

    #[test]
    fn objective_can_be_created_without_external_ref() {
        let node = CognitiveNode::new(
            "ws",
            CognitiveNodeKind::Objective,
            "Define cognitive ontology",
            Some("Batch 1".into()),
            90,
            80,
            20,
            None,
            None,
            "t0",
        )
        .unwrap();
        assert_eq!(node.kind, CognitiveNodeKind::Objective);
        assert_eq!(node.authority_effect, "none");
        node.validate().unwrap();
    }

    #[test]
    fn relation_rejects_self_loop() {
        let err = CognitiveRelation::new(
            "ws",
            "cognitive:a",
            "cognitive:a",
            CognitiveRelationKind::DependsOn,
            50,
            None,
            "t0",
        );
        assert_eq!(err, Err(CognitiveModelError::SelfRelation));
    }

    #[test]
    fn assembled_state_is_non_commandable() {
        let objective = CognitiveNode::new(
            "ws",
            CognitiveNodeKind::Objective,
            "Obj",
            None,
            50,
            50,
            50,
            None,
            None,
            "t0",
        )
        .unwrap();
        let state = CognitiveModelState::assemble("ws", vec![objective], vec![], "t1");
        assert!(state.is_non_commandable());
        assert!(state.current_focus_ids.is_empty());
    }
}
