//! Operational recovery contract helpers — detector predicates only.
//!
//! Does not own lifecycle. Services remain authorities; these helpers encode
//! shared fail-closed recovery expectations for tests and documentation.

use crate::execution_reconciliation::{ExecutionLifecycleRecord, ExecutionState};
use crate::workspace_recommendation::RecommendationHistoryEntry;

/// Stale in-progress claims must never advertise retry after fail-closed recovery.
pub fn recovered_stale_claim_is_non_retryable(record: &ExecutionLifecycleRecord) -> bool {
    record.state == ExecutionState::Failed && !record.retry_allowed
}

/// Recovery must not invent terminal history from an in-progress claim.
pub fn in_progress_claim_is_not_terminal_evidence(record: &ExecutionLifecycleRecord) -> bool {
    record.state == ExecutionState::InProgress
}

/// Missing / incomplete history evidence must not pass as terminal proof.
pub fn history_evidence_is_complete(entry: &RecommendationHistoryEntry) -> bool {
    entry.is_non_actionable()
}

/// Recovery repairs must go through service/command paths — never fabricate
/// an actionable history row.
pub fn recovery_must_not_fabricate_actionable_history(entry: &RecommendationHistoryEntry) -> bool {
    !entry.actionable
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execution_reconciliation::ExecutionLifecycleRecord;
    use crate::workspace_recommendation::{RecommendationHistoryEntry, RecommendationOutcomeView};

    #[test]
    fn stale_recovery_contract_rejects_retryable_failed_claims() {
        let record = ExecutionLifecycleRecord {
            execution_request_id: "e1".into(),
            suggestion_id: "s1".into(),
            intent_id: None,
            state: ExecutionState::Failed,
            retry_allowed: true,
            failure_reason: Some("stale".into()),
            claimed_at: "t0".into(),
            completed_at: Some("t1".into()),
            updated_at: "t1".into(),
        };
        assert!(!recovered_stale_claim_is_non_retryable(&record));
    }

    #[test]
    fn incomplete_history_is_not_complete_evidence() {
        let entry = RecommendationHistoryEntry {
            native_id: "r".into(),
            lifecycle_state: "accepted".into(),
            outcome: RecommendationOutcomeView {
                outcome_id: "".into(),
                recommendation_id: "r".into(),
                user_decision: "accepted".into(),
                result_kind: "accepted_follow_through".into(),
                lifecycle_resolution: Some("accepted".into()),
                recorded_at: "t".into(),
                explanation_keys: vec![],
                evidence_refs: vec![],
                experience_trace_match_keys: vec![],
                is_system_failure: false,
                authority_effect: "none".into(),
            },
            resolved_at: Some("t".into()),
            terminal: true,
            actionable: false,
            authority_effect: "none".into(),
        };
        assert!(!history_evidence_is_complete(&entry));
        assert!(recovery_must_not_fabricate_actionable_history(&entry));
    }
}
