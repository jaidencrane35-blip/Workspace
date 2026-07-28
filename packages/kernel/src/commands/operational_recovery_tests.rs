//! Operational integrity & recovery — restart continuity and fail-closed repair.
//!
//! Extends governance failure tests with startup reconciliation and recovery
//! contracts. Does not introduce new lifecycle authorities.

use chrono::{Duration, Utc};
use workspace_database::{Database, ExecutionLifecycleRepository};
use workspace_domain::{
    history_evidence_is_complete, in_progress_claim_is_not_terminal_evidence,
    recovered_stale_claim_is_non_retryable, recovery_must_not_fabricate_actionable_history,
    ExecutionLifecycleHistoryEntry, ExecutionLifecycleRecord, ExecutionState,
    RecommendationHistoryEntry, RecommendationOutcomeView,
};

use crate::services::{AuditService, ExecutionLifecycleService};
use crate::WorkspaceKernel;

#[test]
fn startup_publishes_started_and_ready_after_audit_subscriber() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let events = AuditService::list_recent(&kernel.shared_database(), 50).unwrap();
    let names: Vec<_> = events.iter().map(|e| e.event_type.as_str()).collect();
    assert!(
        names.contains(&"system.workspace.started"),
        "WorkspaceStarted must be audited after subscriber registration; got {names:?}"
    );
    assert!(
        names.contains(&"system.workspace.ready"),
        "WorkspaceReady must be audited after subscriber registration; got {names:?}"
    );
}

#[test]
fn startup_reconciles_stale_in_progress_claims_fail_closed() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();

    let stale_claimed_at = (Utc::now() - Duration::seconds(400)).to_rfc3339();
    {
        let guard = db.lock().unwrap();
        let repo = ExecutionLifecycleRepository::new(&guard);
        let record = ExecutionLifecycleRecord {
            execution_request_id: "execution:startup-stale".into(),
            suggestion_id: "suggestion:startup-stale".into(),
            intent_id: Some("intent:startup-stale".into()),
            state: ExecutionState::InProgress,
            retry_allowed: false,
            failure_reason: None,
            claimed_at: stale_claimed_at,
            completed_at: None,
            updated_at: Utc::now().to_rfc3339(),
        };
        assert!(repo.insert_claim(&record).unwrap());
    }

    let inspected =
        ExecutionLifecycleService::reconcile_stale_claims_at_startup(&db).expect("sweep");
    assert!(inspected >= 1);

    let recovered = ExecutionLifecycleService::get(&db, "execution:startup-stale")
        .unwrap()
        .expect("present");
    assert!(
        recovered_stale_claim_is_non_retryable(&recovered),
        "stale claim must fail closed without retry: {recovered:?}"
    );
    assert!(
        ExecutionLifecycleHistoryEntry::from_record(&recovered)
            .expect("terminal history")
            .is_non_actionable()
    );
}

#[test]
fn fresh_in_progress_claim_survives_startup_sweep_without_fabrication() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();

    let claimed = ExecutionLifecycleService::claim(
        &db,
        "execution:startup-fresh",
        "suggestion:startup-fresh",
        Some("intent:startup-fresh"),
    )
    .unwrap();
    assert!(in_progress_claim_is_not_terminal_evidence(&claimed));
    assert!(ExecutionLifecycleHistoryEntry::from_record(&claimed).is_none());

    ExecutionLifecycleService::reconcile_stale_claims_at_startup(&db).unwrap();
    let still = ExecutionLifecycleService::get(&db, "execution:startup-fresh")
        .unwrap()
        .expect("present");
    assert_eq!(still.state, ExecutionState::InProgress);
    assert!(
        ExecutionLifecycleHistoryEntry::from_record(&still).is_none(),
        "fresh claim must not fabricate terminal history"
    );
}

#[test]
fn restart_continuity_preserves_completed_execution_without_retry() {
    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let db = kernel.shared_database();
    ExecutionLifecycleService::claim(
        &db,
        "execution:restart-done",
        "suggestion:restart-done",
        Some("intent:restart-done"),
    )
    .unwrap();
    ExecutionLifecycleService::complete(&db, "execution:restart-done", Some("intent:restart-done"))
        .unwrap();

    // Simulate restart sweep on the same durable DB handle.
    ExecutionLifecycleService::reconcile_stale_claims_at_startup(&db).unwrap();
    let done = ExecutionLifecycleService::get(&db, "execution:restart-done")
        .unwrap()
        .expect("present");
    assert_eq!(done.state, ExecutionState::Completed);
    assert!(!done.retry_allowed);
}

#[test]
fn missing_evidence_and_stale_actionable_history_fail_closed() {
    let incomplete = RecommendationHistoryEntry {
        native_id: "rec-miss".into(),
        lifecycle_state: "accepted".into(),
        outcome: RecommendationOutcomeView {
            outcome_id: "".into(),
            recommendation_id: "rec-miss".into(),
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
    assert!(!history_evidence_is_complete(&incomplete));

    let stale = RecommendationHistoryEntry {
        native_id: "rec-stale".into(),
        lifecycle_state: "accepted".into(),
        outcome: RecommendationOutcomeView {
            outcome_id: "recommendation_outcome:rec-stale".into(),
            recommendation_id: "rec-stale".into(),
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
    assert!(!history_evidence_is_complete(&stale));
    assert!(!recovery_must_not_fabricate_actionable_history(&stale));
    assert!(stale.actionable);
    assert!(!stale.is_non_actionable());
}

#[test]
fn dual_write_transaction_rolls_back_partial_recommendation_continuity() {
    // Prove Database::run_in_transaction rolls back when second write fails.
    let db = Database::open_in_memory().unwrap();
    let runner = workspace_database::MigrationRunner::load_from_dir(
        workspace_database::bundled_migrations_dir(),
    )
    .unwrap();
    runner.apply_all(&db).unwrap();

    let err: Result<(), workspace_database::DatabaseError> = db.run_in_transaction(|database| {
        database.connection().execute(
            "INSERT INTO recommendation_lifecycle (
                workspace_id, native_id, lifecycle_state, presented_at, resolved_at,
                resolution_type, outcome_json, prior_outcomes_json, content_fingerprint,
                created_at, updated_at, actor_id
             ) VALUES ('ws', 'rec-tx', 'superseded', NULL, 't', 'superseded', NULL, NULL, 'fp', 't', 't', 'a')",
            [],
        )?;
        Err(workspace_database::DatabaseError::Migration(
            "forced continuity rollback".into(),
        ))
    });
    assert!(err.is_err());
    let count: i64 = db
        .connection()
        .query_row(
            "SELECT COUNT(*) FROM recommendation_lifecycle WHERE native_id = 'rec-tx'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "failed continuity dual-write must not leave partial row"
    );
}

#[test]
fn recovery_does_not_bypass_permission_gateway_for_repair() {
    use workspace_domain::{
        ActorContext, Capability, CapabilitySet, IntentContext, ResourceKind,
    };

    use crate::commands::create_workspace::CreateWorkspace;
    use crate::commands::pipeline::CommandPipeline;
    use crate::security::{GatewayDecision, PermissionGateway, PermissionRequest, PermissionSubject};

    let kernel = WorkspaceKernel::initialize_in_memory().unwrap();
    let actor = ActorContext::local_user();
    let intent = IntentContext::user_request();
    let request = PermissionRequest {
        actor: actor.actor.clone(),
        intent: intent.intent.clone(),
        capability: Capability::workspace_write(),
        command: "CreateWorkspace",
        subject: PermissionSubject::Resource(ResourceKind::Workspace),
        target_resource_id: None,
    };

    let decision = PermissionGateway::evaluate(
        kernel.permission_policy(),
        kernel.permission_gate(),
        &request,
        &CapabilitySet::new(),
    )
    .expect("evaluate must return a decision");
    assert!(
        !matches!(decision, GatewayDecision::Allow { .. }),
        "empty capabilities must not authorize recovery-style repair"
    );

    let mut ctx = kernel.command_context(actor, intent);
    ctx.capability_set = CapabilitySet::new();
    let err = CommandPipeline::new(ctx)
        .execute_mutation(CreateWorkspace::new("recovery-unauthorized".into()))
        .expect_err("unauthorized repair must stay on command path and fail closed");
    let message = err.to_string().to_lowercase();
    assert!(
        message.contains("denied")
            || message.contains("permission")
            || message.contains("capability")
            || message.contains("not granted")
            || message.contains("unauthorized"),
        "deny must be explicit, got: {err}"
    );
}
