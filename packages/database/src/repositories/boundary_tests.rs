use super::{
    DecisionEngineRepository, DecisionQueueRepository, RecommendationLifecycleRepository,
    TaskGraphRepository,
};
use crate::init::DatabaseService;
use workspace_domain::{
    DecisionEngineOverlay, DecisionLifecycleOverlay, DecisionOutcome, DecisionSourceType,
    DecisionState, RecommendationLifecycleOverlay, RecommendationLifecycleState,
    WorkspaceTask, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

fn database() -> (tempfile::TempDir, crate::Database) {
    let dir = tempfile::tempdir().unwrap();
    let db = DatabaseService::initialize(dir.path().join("workspace.db"))
        .unwrap()
        .into_database();
    (dir, db)
}

#[test]
fn recommendation_repository_rejects_terminal_reopen() {
    let (_dir, db) = database();
    let repo = RecommendationLifecycleRepository::new(&db);
    let mut overlay = RecommendationLifecycleOverlay {
        workspace_id: "ws-1".into(),
        native_id: "rec-1".into(),
        lifecycle_state: RecommendationLifecycleState::Accepted,
        created_at: "t0".into(),
        presented_at: Some("t1".into()),
        resolved_at: Some("t2".into()),
        resolution_type: None,
        actor_id: Some("local-user".into()),
        outcome: None,
        prior_outcomes: vec![],
        content_fingerprint: Some("fp-1".into()),
        decision_confirmation: None,
        decision_intake_package_seal: None,
        decision_intake_adapter_preparation: None,
        decision_handoff_request: None,
        decision_engine_acceptance: None,
        updated_at: "t2".into(),
        authority_effect: "none".into(),
    };
    repo.upsert_overlay(&overlay).unwrap();
    overlay.lifecycle_state = RecommendationLifecycleState::Presented;
    overlay.updated_at = "t3".into();
    assert!(repo.upsert_overlay(&overlay).is_err());
    assert_eq!(
        repo.get_overlay("ws-1", "rec-1")
            .unwrap()
            .unwrap()
            .lifecycle_state,
        RecommendationLifecycleState::Accepted
    );
}

#[test]
fn decision_engine_repository_rejects_terminal_reopen() {
    let (_dir, db) = database();
    let repo = DecisionEngineRepository::new(&db);
    let mut overlay = DecisionEngineOverlay {
        workspace_id: "ws-1".into(),
        candidate_key: "candidate-1".into(),
        outcome: DecisionOutcome::Selected,
        updated_at: "t1".into(),
        actor_id: "local-user".into(),
    };
    repo.upsert_overlay(&overlay).unwrap();
    overlay.outcome = DecisionOutcome::Open;
    assert!(repo.upsert_overlay(&overlay).is_err());
    assert_eq!(
        repo.list_overlays("ws-1").unwrap()[0].outcome,
        DecisionOutcome::Selected
    );
}

#[test]
fn decision_engine_repository_rejects_invalidated_intake_reactivation() {
    let (_dir, db) = database();
    db.connection()
        .execute(
            "INSERT INTO decision_engine_intake_candidate (
                workspace_id, intake_candidate_id, intake_receipt_reference,
                recommendation_reference, package_seal_digest, acceptance_reference,
                compatibility_version, state, created_at, updated_at,
                lifecycle_state, lifecycle_reason, lifecycle_updated_at
             ) VALUES ('ws-1','intake-1','receipt-1','rec-1','seal-1','accept-1',
                       'v1','candidate','t0','t0','active',NULL,'t0')",
            [],
        )
        .unwrap();
    let repo = DecisionEngineRepository::new(&db);
    let mut candidate = repo
        .get_intake_candidate("ws-1", "intake-1")
        .unwrap()
        .unwrap();
    candidate.lifecycle.lifecycle_state = "invalidated".into();
    candidate.lifecycle.reason = Some("test invalidation".into());
    candidate.lifecycle.updated_at = "t1".into();
    repo.upsert_intake_candidate(&candidate).unwrap();
    candidate.lifecycle.lifecycle_state = "active".into();
    candidate.lifecycle.updated_at = "t2".into();
    assert!(repo.upsert_intake_candidate(&candidate).is_err());
    assert_eq!(
        repo.get_intake_candidate("ws-1", "intake-1")
            .unwrap()
            .unwrap()
            .lifecycle
            .lifecycle_state,
        "invalidated"
    );
}

#[test]
fn decision_queue_repository_rejects_dismissed_reopen() {
    let (_dir, db) = database();
    let repo = DecisionQueueRepository::new(&db);
    let mut overlay = DecisionLifecycleOverlay {
        workspace_id: "ws-1".into(),
        source_type: DecisionSourceType::IntentProposal,
        source_id: "source-1".into(),
        decision_state: DecisionState::Dismissed,
        updated_at: "t1".into(),
        actor_id: "local-user".into(),
    };
    repo.upsert_overlay(&overlay).unwrap();
    overlay.decision_state = DecisionState::Viewed;
    assert!(repo.upsert_overlay(&overlay).is_err());
    assert_eq!(
        repo.get_overlay("ws-1", DecisionSourceType::IntentProposal, "source-1")
            .unwrap()
            .unwrap()
            .decision_state,
        DecisionState::Dismissed
    );
}

#[test]
fn task_repository_rejects_terminal_reopen_but_allows_terminal_sync() {
    let (_dir, db) = database();
    let repo = TaskGraphRepository::new(&db);
    let task = WorkspaceTask::new(
        "ws-1",
        "Task",
        None,
        WorkspaceTaskPriority::Medium,
    )
    .unwrap()
    .transition_status(WorkspaceTaskStatus::Completed, "done")
    .unwrap();
    repo.upsert_task(&task).unwrap();
    let mut same_terminal = task.clone();
    same_terminal.title = "Updated title".into();
    repo.upsert_task(&same_terminal).unwrap();
    let mut reopened = task.clone();
    reopened.status = WorkspaceTaskStatus::InProgress;
    assert!(repo.upsert_task(&reopened).is_err());
    let stored = repo.get_task(task.id.as_str()).unwrap().unwrap();
    assert_eq!(stored.status, WorkspaceTaskStatus::Completed);
    assert_eq!(stored.title, "Updated title");
}
