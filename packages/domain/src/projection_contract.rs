//! Cross-domain projection contract helpers and invariant tests.
//!
//! Shared channels across RE / DQ / DE / Task Graph / Execution:
//! actionable + history + authoritative `history_count`.
//! Does not own lifecycle — domain services remain authorities.

use serde_json::Value;

/// `history_count` is authoritative over the returned window length.
pub fn history_count_is_authoritative(window_len: usize, history_count: usize) -> bool {
    history_count >= window_len
}

/// History evidence JSON must not expose command / transition affordances.
pub fn history_json_is_non_commandable(value: &Value) -> bool {
    let forbidden = [
        "execute",
        "handoff_command",
        "dispatch_allowed",
        "cancellation_allowed",
        "recommended_action",
        "select",
        "dismiss",
        "transition",
        "mutate",
    ];
    match value {
        Value::Object(map) => {
            for key in forbidden {
                if map.contains_key(key) {
                    return false;
                }
            }
            if map.get("actionable").and_then(|v| v.as_bool()) == Some(true) {
                return false;
            }
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decision_engine::{
        DecisionArtifactHistoryEntry, DecisionCandidate, DecisionEngineOverlay, DecisionOutcome,
    };
    use crate::decision_queue::{
        DecisionLifecycleOverlay, DecisionOverlayHistoryEntry, DecisionQueue, DecisionSourceType,
        DecisionState,
    };
    use crate::execution_reconciliation::{
        ExecutionLifecycleHistoryEntry, ExecutionLifecycleProjection, ExecutionLifecycleRecord,
        ExecutionState,
    };
    use crate::workspace_recommendation::{
        RecommendationHistoryEntry, RecommendationOutcomeView,
    };
    use crate::workspace_task_graph::{
        TaskGraph, WorkspaceTask, WorkspaceTaskPriority, WorkspaceTaskStatus,
    };

    #[test]
    fn count_authority_rejects_window_longer_than_count() {
        assert!(history_count_is_authoritative(2, 5));
        assert!(history_count_is_authoritative(0, 0));
        assert!(!history_count_is_authoritative(3, 2));
    }

    #[test]
    fn recommendation_history_dto_is_non_commandable() {
        let entry = RecommendationHistoryEntry {
            native_id: "rec-1".into(),
            lifecycle_state: "accepted".into(),
            outcome: RecommendationOutcomeView {
                outcome_id: "recommendation_outcome:rec-1".into(),
                recommendation_id: "rec-1".into(),
                user_decision: "accepted".into(),
                result_kind: "accepted_follow_through".into(),
                lifecycle_resolution: Some("accepted".into()),
                recorded_at: "t1".into(),
                explanation_keys: Vec::new(),
                evidence_refs: Vec::new(),
                experience_trace_match_keys: Vec::new(),
                is_system_failure: false,
                authority_effect: "none".into(),
            },
            resolved_at: Some("t1".into()),
            terminal: true,
            actionable: false,
            authority_effect: "none".into(),
        };
        let json = serde_json::to_value(&entry).unwrap();
        assert!(history_json_is_non_commandable(&json));
        assert!(entry.is_non_actionable());
    }

    #[test]
    fn decision_queue_history_is_non_commandable_and_count_authoritative() {
        let history: Vec<_> = (0..3)
            .map(|i| {
                DecisionOverlayHistoryEntry::from_overlay(
                    &DecisionLifecycleOverlay {
                        workspace_id: "ws".into(),
                        source_type: DecisionSourceType::IntentProposal,
                        source_id: format!("id-{i}"),
                        decision_state: DecisionState::Dismissed,
                        updated_at: format!("t{i}"),
                        actor_id: "u".into(),
                    },
                    false,
                )
                .unwrap()
            })
            .collect();
        assert!(history.iter().all(|h| h.is_non_actionable()));
        assert!(history_json_is_non_commandable(
            &serde_json::to_value(&history[0]).unwrap()
        ));

        let queue = DecisionQueue::from_items("ws", Vec::new()).with_overlay_history(history);
        let summary = queue.summary(1);
        assert_eq!(summary.history.len(), 1);
        assert_eq!(summary.history_count, 3);
        assert!(history_count_is_authoritative(
            summary.history.len(),
            summary.history_count
        ));
        assert!(summary.items.is_empty());
    }

    #[test]
    fn decision_engine_orphan_provenance_stays_unknown() {
        let overlay = DecisionEngineOverlay {
            workspace_id: "ws".into(),
            candidate_key: "attention:orphan".into(),
            outcome: DecisionOutcome::Dismissed,
            updated_at: "t1".into(),
            actor_id: "u".into(),
        };
        let entry = DecisionArtifactHistoryEntry::from_overlay(&overlay).unwrap();
        assert!(entry.is_non_actionable());
        assert_eq!(entry.origin, DecisionCandidate::ORIGIN_UNKNOWN);
        assert!(!entry.has_known_origin());
        assert!(history_json_is_non_commandable(
            &serde_json::to_value(&entry).unwrap()
        ));
    }

    #[test]
    fn task_graph_separates_actionable_and_history() {
        let mut open =
            WorkspaceTask::new("ws", "Live", None, WorkspaceTaskPriority::Medium).unwrap();
        open = open
            .transition_status(WorkspaceTaskStatus::InProgress, "working")
            .unwrap();
        let mut done =
            WorkspaceTask::new("ws", "Past", None, WorkspaceTaskPriority::Medium).unwrap();
        done = done
            .transition_status(WorkspaceTaskStatus::Completed, "done")
            .unwrap();
        let graph = TaskGraph::from_parts("ws", vec![open, done], Vec::new(), true, Vec::new());
        assert!(graph.nodes.iter().all(|n| n.task.status.is_open()));
        assert!(graph.history.iter().all(|h| h.is_non_actionable()));
        assert!(history_json_is_non_commandable(
            &serde_json::to_value(&graph.history[0]).unwrap()
        ));
        let summary = graph.summary_projection(1);
        assert!(history_count_is_authoritative(
            summary.history.len(),
            summary.history_count
        ));
    }

    #[test]
    fn execution_projection_separates_channels_and_keeps_unknown_unknown() {
        let records = vec![
            ExecutionLifecycleRecord {
                execution_request_id: "execution:live".into(),
                suggestion_id: "s1".into(),
                intent_id: None,
                state: ExecutionState::InProgress,
                retry_allowed: false,
                failure_reason: None,
                claimed_at: "t0".into(),
                completed_at: None,
                updated_at: "t1".into(),
            },
            ExecutionLifecycleRecord {
                execution_request_id: "execution:fail".into(),
                suggestion_id: "s2".into(),
                intent_id: None,
                state: ExecutionState::Failed,
                retry_allowed: true,
                failure_reason: Some("denied".into()),
                claimed_at: "t0".into(),
                completed_at: None,
                updated_at: "t2".into(),
            },
            ExecutionLifecycleRecord {
                execution_request_id: "execution:mystery".into(),
                suggestion_id: "s3".into(),
                intent_id: None,
                state: ExecutionState::Unknown,
                retry_allowed: false,
                failure_reason: None,
                claimed_at: "t0".into(),
                completed_at: None,
                updated_at: "t0".into(),
            },
        ];
        let projection = ExecutionLifecycleProjection::from_records(&records, &[], 1);
        assert_eq!(projection.actionable.len(), 1);
        assert!(projection
            .actionable
            .iter()
            .all(|e| e.state.is_actionable()));
        assert_eq!(projection.history.len(), 1);
        assert_eq!(projection.history_count, 1); // only Failed is terminal; Unknown skipped
        // Wait - Failed is terminal, Unknown skipped, so history_count should be 1
        assert!(history_count_is_authoritative(
            projection.history.len(),
            projection.history_count
        ));
        let full = ExecutionLifecycleProjection::from_records(&records, &[], 10);
        assert_eq!(full.history_count, 1);
        assert!(full.history.iter().all(|h| h.is_non_actionable()));
        let failed = ExecutionLifecycleHistoryEntry::from_record(&records[1]).unwrap();
        assert!(failed.retry_allowed);
        assert_eq!(failed.failure_reason.as_deref(), Some("denied"));
        assert!(history_json_is_non_commandable(
            &serde_json::to_value(&failed).unwrap()
        ));
        assert!(!full
            .history
            .iter()
            .any(|h| h.execution_request_id == "execution:mystery"));
    }
}
