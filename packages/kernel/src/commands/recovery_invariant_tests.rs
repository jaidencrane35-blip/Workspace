//! Shared recovery invariant contract tests (Batch F).
//!
//! Reusable architectural assertions: recovery never invents terminal state /
//! provenance, never bypasses lifecycle/repository/gateway authorities, and
//! recovery diagnostics remain evidence only.

use workspace_domain::{
    recovery_diagnostic_event_type_is_non_commandable, recovery_diagnostic_is_evidence_only,
    recovery_must_not_fabricate_actionable_history, recovery_must_not_invent_completed,
    recovered_stale_claim_is_non_retryable, Actor, AuditEvent, ExecutionLifecycleRecord,
    ExecutionState, RecommendationHistoryEntry, RecommendationOutcomeView,
    RECOVERY_DIAGNOSTIC_EVENT_TYPES, RECOVERY_DIAGNOSTIC_FAILED,
};

use crate::services::AuditService;
use crate::WorkspaceKernel;

#[test]
fn recovery_never_invents_completed_from_failed_stale_shape() {
    let failed = ExecutionLifecycleRecord {
        execution_request_id: "e".into(),
        suggestion_id: "s".into(),
        intent_id: None,
        state: ExecutionState::Failed,
        retry_allowed: false,
        failure_reason: Some("stale execution claim; dispatch outcome unknown".into()),
        claimed_at: "t0".into(),
        completed_at: None,
        updated_at: "t1".into(),
    };
    assert!(recovered_stale_claim_is_non_retryable(&failed));
    assert!(recovery_must_not_invent_completed(&failed));
    assert_ne!(failed.state, ExecutionState::Completed);
}

#[test]
fn recovery_never_invents_provenance_via_actionable_history() {
    let fabricated = RecommendationHistoryEntry {
        native_id: "r".into(),
        lifecycle_state: "accepted".into(),
        outcome: RecommendationOutcomeView {
            outcome_id: "recommendation_outcome:r".into(),
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
        actionable: true,
        authority_effect: "none".into(),
    };
    assert!(!recovery_must_not_fabricate_actionable_history(&fabricated));
}

#[test]
fn recovery_diagnostics_are_evidence_only_across_event_types() {
    for event_type in RECOVERY_DIAGNOSTIC_EVENT_TYPES {
        assert!(recovery_diagnostic_event_type_is_non_commandable(event_type));
        let event = AuditEvent::from_actor(*event_type, &Actor::system(), true).with_metadata(
            r#"{"authority_effect":"none","subsystem":"execution_lifecycle"}"#,
        );
        assert!(recovery_diagnostic_is_evidence_only(&event));
        assert!(event.command_name.is_none());
    }
}

#[test]
fn recovery_diagnostic_recorder_rejects_commandable_event_types() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let err = AuditService::record_recovery_diagnostic(
        &kernel.shared_database(),
        "system.recovery.startup.execute",
        false,
        r#"{"authority_effect":"none"}"#.into(),
    );
    assert!(err.is_err());
}

#[test]
fn startup_recovery_diagnostics_never_attach_command_names() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let events = AuditService::list_recent(&kernel.shared_database(), 100).unwrap();
    let recovery: Vec<_> = events
        .iter()
        .filter(|e| RECOVERY_DIAGNOSTIC_EVENT_TYPES.contains(&e.event_type.as_str()))
        .collect();
    assert!(!recovery.is_empty());
    for event in recovery {
        assert!(recovery_diagnostic_is_evidence_only(event));
        assert!(event.command_name.is_none());
        assert_ne!(event.event_type, "command.executed");
    }
    assert!(events.iter().all(|e| e.event_type != RECOVERY_DIAGNOSTIC_FAILED));
}
