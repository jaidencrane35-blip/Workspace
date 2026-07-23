//! Deterministic suggestion proposals — the "Suggest" boundary (Sprint 20).
//!
//! A [`Suggestion`] is a deterministic PROPOSAL object derived from a
//! [`WorkspaceContext`]. It is NOT an action: it never executes, mutates state,
//! or bypasses governance. The preserved future flow is
//! Context → Suggestion → (future) User Approval → Intent → Capability →
//! Permission → CommandPipeline → Execution.
//!
//! Generation is purely rule/threshold based — no AI, no LLM, no ML ranking, no
//! probability, no behaviour prediction. "Confidence" is deterministic metadata
//! only (a supporting count and a human-readable basis). Pure domain type: no
//! database, IO, or UI dependencies.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::context::WorkspaceContext;
use crate::resource::ResourceRef;

/// Deterministic thresholds that gate suggestion generation.
const RESOURCE_GROWTH_THRESHOLD: usize = 3;
const WORKSPACE_ACTIVITY_THRESHOLD: usize = 5;
const LAYOUT_ACTIVITY_THRESHOLD: usize = 2;

/// Kind of proposal, grounded in what `WorkspaceContext` deterministically
/// contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    /// The workspace has meaningful overall activity.
    WorkspaceActivity,
    /// Resources are being created at a notable rate.
    ResourceGrowth,
    /// Layout is being changed repeatedly.
    LayoutActivity,
}

/// Lifecycle status of a suggestion. Generation always yields `Pending`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}

/// Deterministic supporting evidence for a suggestion. NOT a probabilistic
/// score — just a count and a human-readable basis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestionConfidence {
    /// Number of supporting observations/events behind the proposal.
    pub supporting_observation_count: usize,
    /// Deterministic description of the rule that produced the suggestion.
    pub basis: String,
}

/// Suggestion-specific validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SuggestionError {
    #[error("Suggestion id must not be empty")]
    EmptyId,

    #[error("Suggestion title must not be empty")]
    EmptyTitle,

    #[error("Suggestion description must not be empty")]
    EmptyDescription,

    #[error("Suggestion source context must not be empty")]
    EmptySourceContext,

    #[error("Suggestion not found: {0}")]
    NotFound(String),

    #[error("Suggestion '{id}' cannot transition from {from:?} to {to:?}")]
    InvalidTransition {
        id: String,
        from: SuggestionStatus,
        to: SuggestionStatus,
    },
}

/// A deterministic, read-only proposal. Never an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Suggestion {
    /// Stable, deterministic identifier for this proposal.
    pub id: String,
    pub suggestion_type: SuggestionType,
    pub title: String,
    pub description: String,
    /// Short deterministic string describing which context inputs produced it
    /// (not the whole context).
    pub source_context: String,
    /// Canonical resource this proposal relates to, when applicable.
    pub related_resource_ref: Option<ResourceRef>,
    pub confidence: SuggestionConfidence,
    pub status: SuggestionStatus,
}

impl Suggestion {
    pub fn validate(&self) -> Result<(), SuggestionError> {
        if self.id.trim().is_empty() {
            return Err(SuggestionError::EmptyId);
        }
        if self.title.trim().is_empty() {
            return Err(SuggestionError::EmptyTitle);
        }
        if self.description.trim().is_empty() {
            return Err(SuggestionError::EmptyDescription);
        }
        if self.source_context.trim().is_empty() {
            return Err(SuggestionError::EmptySourceContext);
        }
        Ok(())
    }

    /// Marks a pending proposal as accepted. Decision only — never executes.
    pub fn accept(self) -> Result<Self, SuggestionError> {
        self.transition(SuggestionStatus::Accepted)
    }

    /// Marks a pending proposal as rejected. Decision only — never executes.
    pub fn reject(self) -> Result<Self, SuggestionError> {
        self.transition(SuggestionStatus::Rejected)
    }

    fn transition(mut self, to: SuggestionStatus) -> Result<Self, SuggestionError> {
        if self.status != SuggestionStatus::Pending {
            return Err(SuggestionError::InvalidTransition {
                id: self.id,
                from: self.status,
                to,
            });
        }
        self.status = to;
        Ok(self)
    }
}

/// Finds a currently derived pending suggestion by id.
pub fn find_pending_suggestion(
    suggestions: &[Suggestion],
    suggestion_id: &str,
) -> Result<Suggestion, SuggestionError> {
    suggestions
        .iter()
        .find(|suggestion| suggestion.id == suggestion_id)
        .cloned()
        .ok_or_else(|| SuggestionError::NotFound(suggestion_id.to_string()))
}

/// Deterministically derives proposals from a workspace context.
///
/// Rule/threshold based only. Given the same context, the output is identical.
/// All suggestions start as `Pending` and never trigger any action.
pub fn derive_suggestions(context: &WorkspaceContext) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();
    let workspace = &context.workspace;
    let workspace_id = workspace.canonical();
    let metrics = &context.metrics;

    if metrics.resource_creation_count >= RESOURCE_GROWTH_THRESHOLD {
        suggestions.push(Suggestion {
            id: format!("resource-growth:{workspace_id}"),
            suggestion_type: SuggestionType::ResourceGrowth,
            title: "Workspace is growing".into(),
            description: format!(
                "{} resources have been created recently in this workspace. \
                 You may want to organize them into zones or a layout.",
                metrics.resource_creation_count
            ),
            source_context: "metrics.resource_creation_count".into(),
            related_resource_ref: Some(workspace.clone()),
            confidence: SuggestionConfidence {
                supporting_observation_count: metrics.resource_creation_count,
                basis: format!(
                    "resource_creation_count >= {RESOURCE_GROWTH_THRESHOLD}"
                ),
            },
            status: SuggestionStatus::Pending,
        });
    }

    if metrics.layout_change_count >= LAYOUT_ACTIVITY_THRESHOLD {
        suggestions.push(Suggestion {
            id: format!("layout-activity:{workspace_id}"),
            suggestion_type: SuggestionType::LayoutActivity,
            title: "Frequent layout changes".into(),
            description: format!(
                "The layout of this workspace has changed {} times recently. \
                 You may want to save a preferred layout arrangement.",
                metrics.layout_change_count
            ),
            source_context: "metrics.layout_change_count".into(),
            related_resource_ref: Some(workspace.clone()),
            confidence: SuggestionConfidence {
                supporting_observation_count: metrics.layout_change_count,
                basis: format!("layout_change_count >= {LAYOUT_ACTIVITY_THRESHOLD}"),
            },
            status: SuggestionStatus::Pending,
        });
    }

    if metrics.observation_count >= WORKSPACE_ACTIVITY_THRESHOLD {
        suggestions.push(Suggestion {
            id: format!("workspace-activity:{workspace_id}"),
            suggestion_type: SuggestionType::WorkspaceActivity,
            title: "Active workspace".into(),
            description: format!(
                "This workspace has {} recent activity events. \
                 You may want to review its recent history.",
                metrics.observation_count
            ),
            source_context: "metrics.observation_count".into(),
            related_resource_ref: Some(workspace.clone()),
            confidence: SuggestionConfidence {
                supporting_observation_count: metrics.observation_count,
                basis: format!(
                    "observation_count >= {WORKSPACE_ACTIVITY_THRESHOLD}"
                ),
            },
            status: SuggestionStatus::Pending,
        });
    }

    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::ActorType;
    use crate::capability::Capability;
    use crate::discovery::CapabilityDiscovery;
    use crate::ids::ActorId;
    use crate::observation::{classify_event, neutral_summary, Observation};
    use crate::projection::WorkspaceSnapshot;
    use crate::resource::{ResourceId, ResourceKind, ResourceRef};
    use crate::analytics::WorkspaceMetrics;

    fn workspace_ref() -> ResourceRef {
        ResourceRef::new(ResourceKind::Workspace, ResourceId::new("ws-1").unwrap())
    }

    fn observation(source: &str) -> Observation {
        let (category, importance) = classify_event(source).unwrap();
        Observation {
            id: format!("obs-{source}"),
            occurred_at: "2026-07-24T00:00:00Z".into(),
            category,
            importance,
            source_event_type: source.into(),
            summary: neutral_summary(category, source),
            actor_type: ActorType::LocalUser,
            actor_id: Some("local-user".into()),
            resource_ref: None,
            metadata: None,
        }
    }

    fn context_with(observations: Vec<Observation>) -> WorkspaceContext {
        let workspace = workspace_ref();
        let metrics = WorkspaceMetrics::from_observations(&observations, "2026-07-24T00:00:00Z");
        WorkspaceContext {
            generated_at: "2026-07-24T00:00:00Z".into(),
            workspace: workspace.clone(),
            snapshot: WorkspaceSnapshot {
                workspace,
                workspace_name: "Suggest WS".into(),
                zones: vec![],
                applications: vec![],
                widgets: vec![],
                layout_id: None,
                layout_placements: vec![],
                relationships: vec![],
                generated_at: "2026-07-24T00:00:00Z".into(),
            },
            observations,
            metrics,
            capabilities: CapabilityDiscovery {
                actor_id: ActorId::new("local-user").unwrap(),
                actor_type: ActorType::LocalUser,
                capabilities: vec![Capability::workspace_read()],
                available_intents: vec![],
                generated_at: "2026-07-24T00:00:00Z".into(),
            },
        }
    }

    fn growth_context() -> WorkspaceContext {
        // 4 creations + 2 layout changes + lifecycle => crosses all thresholds.
        context_with(vec![
            observation("workspace.entity.created"),
            observation("zone.created"),
            observation("application.created"),
            observation("widget.created"),
            observation("layout.created"),
            observation("layout.updated"),
            observation("system.workspace.started"),
        ])
    }

    #[test]
    fn derives_suggestions_deterministically() {
        let context = growth_context();
        let first = derive_suggestions(&context);
        let second = derive_suggestions(&context);
        assert_eq!(first, second);
        assert!(!first.is_empty());
    }

    #[test]
    fn all_generated_suggestions_start_pending_and_validate() {
        let suggestions = derive_suggestions(&growth_context());
        for suggestion in &suggestions {
            assert_eq!(suggestion.status, SuggestionStatus::Pending);
            assert!(suggestion.validate().is_ok());
        }
    }

    #[test]
    fn resource_growth_suggestion_emitted_above_threshold() {
        let suggestions = derive_suggestions(&growth_context());
        let growth = suggestions
            .iter()
            .find(|s| s.suggestion_type == SuggestionType::ResourceGrowth)
            .expect("resource growth suggestion");
        assert_eq!(growth.confidence.supporting_observation_count, 4);
        assert_eq!(growth.related_resource_ref.as_ref(), Some(&workspace_ref()));
    }

    #[test]
    fn no_suggestions_below_thresholds() {
        // Single lifecycle event: below every threshold.
        let context = context_with(vec![observation("system.workspace.started")]);
        assert!(derive_suggestions(&context).is_empty());
    }

    #[test]
    fn preserves_resource_ref_identity() {
        let suggestions = derive_suggestions(&growth_context());
        assert!(suggestions
            .iter()
            .all(|s| s.related_resource_ref.as_ref() == Some(&workspace_ref())));
    }

    #[test]
    fn serializes_round_trip() {
        let suggestions = derive_suggestions(&growth_context());
        let json = serde_json::to_string(&suggestions).unwrap();
        let restored: Vec<Suggestion> = serde_json::from_str(&json).unwrap();
        assert_eq!(suggestions, restored);
    }

    #[test]
    fn validate_rejects_empty_fields() {
        let mut suggestion = derive_suggestions(&growth_context())
            .into_iter()
            .next()
            .unwrap();
        suggestion.id = String::new();
        assert_eq!(suggestion.validate(), Err(SuggestionError::EmptyId));
    }

    #[test]
    fn accept_and_reject_only_from_pending() {
        let pending = derive_suggestions(&growth_context())
            .into_iter()
            .next()
            .unwrap();
        let accepted = pending.clone().accept().unwrap();
        assert_eq!(accepted.status, SuggestionStatus::Accepted);
        assert!(matches!(
            accepted.accept(),
            Err(SuggestionError::InvalidTransition { .. })
        ));

        let rejected = pending.reject().unwrap();
        assert_eq!(rejected.status, SuggestionStatus::Rejected);
    }

    #[test]
    fn find_pending_suggestion_by_id() {
        let suggestions = derive_suggestions(&growth_context());
        let id = suggestions[0].id.clone();
        assert_eq!(find_pending_suggestion(&suggestions, &id).unwrap().id, id);
        assert!(matches!(
            find_pending_suggestion(&suggestions, "missing"),
            Err(SuggestionError::NotFound(_))
        ));
    }
}
