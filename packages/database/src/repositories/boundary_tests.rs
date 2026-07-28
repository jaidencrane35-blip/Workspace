use super::{
    DecisionEngineRepository, DecisionQueueRepository, RecommendationLifecycleRepository,
    TaskGraphRepository,
};
use crate::error::DatabaseError;
use crate::init::DatabaseService;
use workspace_domain::{
    DecisionCandidateScore, DecisionEngineIntakeEvaluation, DecisionEngineOverlay,
    DecisionLifecycleOverlay, DecisionOutcome, DecisionScore, DecisionSourceType, DecisionState,
    RecommendationDecisionConfirmation, RecommendationDecisionIntakePackageSeal,
    RecommendationFamily, RecommendationIdentity, RecommendationLifecycleOverlay,
    RecommendationLifecycleState, RecommendationOutcome, RecommendationOutcomeQuality,
    RecommendationProvenance, RecommendationResolutionType, RecommendationResultKind,
    RecommendationUserDecision, WorkspaceTask, WorkspaceTaskPriority, WorkspaceTaskStatus,
};

fn database() -> (tempfile::TempDir, crate::Database) {
    let dir = tempfile::tempdir().unwrap();
    let db = DatabaseService::initialize(dir.path().join("workspace.db"))
        .unwrap()
        .into_database();
    (dir, db)
}

fn sample_outcome(native_id: &str) -> RecommendationOutcome {
    RecommendationOutcome {
        id: format!("recommendation_outcome:{native_id}"),
        identity: RecommendationIdentity {
            native_id: native_id.into(),
            family: RecommendationFamily::RecommendationEngine,
            source_domain: "recommendation_engine".into(),
            originating_reasoning_ref: None,
            decision_ref: None,
            action_proposal_ref: None,
        },
        provenance: RecommendationProvenance {
            recommendation_id: native_id.into(),
            family: RecommendationFamily::RecommendationEngine,
            source_evidence: vec![],
            reasoning_origins: vec![],
            explanation_keys: vec![],
            experience_trace_match_keys: vec![],
            confidence: None,
            priority_or_impact: None,
            related_attention_id: None,
            future_capability_target: None,
        },
        lifecycle_resolution: Some(RecommendationResolutionType::Accepted),
        user_decision: RecommendationUserDecision::Accepted,
        result_kind: RecommendationResultKind::AcceptedFollowThrough,
        recorded_at: "t2".into(),
        quality: RecommendationOutcomeQuality {
            confidence_at_outcome: None,
            useful_to_user: None,
            notes: None,
        },
        experience_trace_match_keys: vec![],
        authority_effect: RecommendationOutcome::AUTHORITY_EFFECT_NONE.into(),
    }
}

fn accepted_overlay(native_id: &str) -> RecommendationLifecycleOverlay {
    RecommendationLifecycleOverlay {
        workspace_id: "ws-1".into(),
        native_id: native_id.into(),
        lifecycle_state: RecommendationLifecycleState::Accepted,
        created_at: "t0".into(),
        presented_at: Some("t1".into()),
        resolved_at: Some("t2".into()),
        resolution_type: Some(RecommendationResolutionType::Accepted),
        actor_id: Some("local-user".into()),
        outcome: Some(sample_outcome(native_id)),
        prior_outcomes: vec![],
        content_fingerprint: Some("fp-1".into()),
        decision_confirmation: Some(RecommendationDecisionConfirmation {
            recommendation_id: native_id.into(),
            confirmation_state: RecommendationDecisionConfirmation::STATE_REQUIRED.into(),
            confirmation_intent: RecommendationDecisionConfirmation::INTENT_AGREEMENT_ONLY.into(),
            recommendation_owner: RecommendationDecisionConfirmation::OWNER_RECOMMENDATION.into(),
            confirmation_owner: RecommendationDecisionConfirmation::OWNER_USER.into(),
            decision_owner: RecommendationDecisionConfirmation::OWNER_DECISION.into(),
            execution_owner: RecommendationDecisionConfirmation::OWNER_GATEWAY.into(),
            creates_decision_engine_object: false,
            creates_intent: false,
            grants_execution_authority: false,
            handoff_performed: false,
            confirmed_at: None,
            note: "required".into(),
            authority_effect: RecommendationDecisionConfirmation::AUTHORITY_EFFECT_NONE.into(),
        }),
        decision_intake_package_seal: None,
        decision_intake_adapter_preparation: None,
        decision_handoff_request: None,
        decision_engine_acceptance: None,
        updated_at: "t2".into(),
        authority_effect: "none".into(),
    }
}

#[test]
fn recommendation_repository_rejects_terminal_reopen() {
    let (_dir, db) = database();
    let repo = RecommendationLifecycleRepository::new(&db);
    let mut overlay = accepted_overlay("rec-1");
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
fn recommendation_repository_rejects_terminal_outcome_overwrite() {
    let (_dir, db) = database();
    let repo = RecommendationLifecycleRepository::new(&db);
    let overlay = accepted_overlay("rec-outcome");
    repo.upsert_overlay(&overlay).unwrap();

    let mut erased = overlay.clone();
    erased.outcome = None;
    erased.updated_at = "t3".into();
    match repo.upsert_overlay(&erased) {
        Err(DatabaseError::ImmutableArtifact(message)) => {
            assert!(message.contains("erase terminal outcome"));
        }
        other => panic!("expected ImmutableArtifact, got {other:?}"),
    }

    let mut replaced = overlay.clone();
    let mut other_outcome = sample_outcome("rec-outcome");
    other_outcome.id = "recommendation_outcome:tampered".into();
    replaced.outcome = Some(other_outcome);
    replaced.updated_at = "t4".into();
    assert!(matches!(
        repo.upsert_overlay(&replaced),
        Err(DatabaseError::ImmutableArtifact(_))
    ));

    let stored = repo.get_overlay("ws-1", "rec-outcome").unwrap().unwrap();
    assert_eq!(stored.outcome, overlay.outcome);
}

#[test]
fn recommendation_repository_rejects_sealed_evidence_erase() {
    let (_dir, db) = database();
    let repo = RecommendationLifecycleRepository::new(&db);
    let mut overlay = accepted_overlay("rec-seal");
    overlay.decision_intake_package_seal = Some(RecommendationDecisionIntakePackageSeal {
        recommendation_id: "rec-seal".into(),
        intake_package_digest: "digest-1".into(),
        continuity_fingerprint_at_seal: "fp-1".into(),
        contract_version: "v1".into(),
        confirmation_intent: RecommendationDecisionConfirmation::INTENT_CREATE_FUTURE_DECISION
            .into(),
        confirmed_at: "t2".into(),
        eligibility_state: "eligible".into(),
        sealed: true,
        seal_state: RecommendationDecisionIntakePackageSeal::STATE_SEALED.into(),
        package_matches_seal: true,
        sealed_at: "t2".into(),
        current_owner: RecommendationDecisionIntakePackageSeal::OWNER_RECOMMENDATION.into(),
        proceed_authorized: false,
        consume_authorized: false,
        adapter_invokable: false,
        handoff_performed: false,
        decision_engine_object_id: None,
        permission_effect: RecommendationDecisionIntakePackageSeal::PERMISSION_EFFECT_NONE.into(),
        note: "sealed".into(),
        authority_effect: RecommendationDecisionIntakePackageSeal::AUTHORITY_EFFECT_NONE.into(),
    });
    repo.upsert_overlay(&overlay).unwrap();

    let mut erased = overlay.clone();
    erased.decision_intake_package_seal = None;
    erased.updated_at = "t3".into();
    assert!(matches!(
        repo.upsert_overlay(&erased),
        Err(DatabaseError::ImmutableArtifact(_))
    ));
    assert!(repo
        .get_overlay("ws-1", "rec-seal")
        .unwrap()
        .unwrap()
        .decision_intake_package_seal
        .is_some());
}

#[test]
fn recommendation_repository_allows_generation_reopen_with_prior_outcomes() {
    let (_dir, db) = database();
    let repo = RecommendationLifecycleRepository::new(&db);
    let mut overlay = accepted_overlay("rec-reopen");
    repo.upsert_overlay(&overlay).unwrap();

    overlay.lifecycle_state = RecommendationLifecycleState::Available;
    overlay.resolved_at = None;
    overlay.resolution_type = None;
    overlay.outcome = None;
    overlay.prior_outcomes = vec![sample_outcome("rec-reopen")];
    overlay.content_fingerprint = Some("fp-2".into());
    overlay.updated_at = "t3".into();
    repo.upsert_overlay(&overlay).unwrap();
    let stored = repo.get_overlay("ws-1", "rec-reopen").unwrap().unwrap();
    assert_eq!(stored.lifecycle_state, RecommendationLifecycleState::Available);
    assert_eq!(stored.prior_outcomes.len(), 1);
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
fn decision_engine_repository_rejects_evaluation_replacement() {
    let (_dir, db) = database();
    let repo = DecisionEngineRepository::new(&db);
    let evaluation = DecisionEngineIntakeEvaluation {
        evaluation_id: "engine_decision_intake_eval:intake-1".into(),
        workspace_id: "ws-1".into(),
        intake_candidate_id: "intake-1".into(),
        evaluated_at: "t1".into(),
        evaluation_state: DecisionEngineIntakeEvaluation::STATE_EVALUATED.into(),
        evaluation_reason: "ok".into(),
        creates_decision_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        handoff_command: None,
        note: "evaluated".into(),
        authority_effect: DecisionEngineIntakeEvaluation::AUTHORITY_EFFECT_NONE.into(),
    };
    repo.upsert_intake_evaluation(&evaluation).unwrap();
    repo.upsert_intake_evaluation(&evaluation).unwrap();

    let mut replaced = evaluation.clone();
    replaced.evaluation_reason = "tampered".into();
    assert!(matches!(
        repo.upsert_intake_evaluation(&replaced),
        Err(DatabaseError::ImmutableArtifact(_))
    ));
    let stored = repo
        .get_intake_evaluation_for_candidate("ws-1", "intake-1")
        .unwrap()
        .unwrap();
    assert_eq!(stored.evaluation_reason, "ok");
}

#[test]
fn decision_engine_repository_rejects_score_replacement() {
    let (_dir, db) = database();
    let repo = DecisionEngineRepository::new(&db);
    let score = DecisionCandidateScore {
        score_id: "engine_decision_score:cand-1".into(),
        workspace_id: "ws-1".into(),
        decision_candidate_id: "cand-1".into(),
        origin: "native".into(),
        resolution_id: "res-1".into(),
        score: DecisionScore {
            total: 70,
            attention_contribution: 10,
            memory_contribution: 10,
            personalization_contribution: 10,
            goal_contribution: 10,
            factors: vec!["factor-a".into()],
        },
        scoring_factors: vec!["factor-a".into()],
        scored_at: "t1".into(),
        intake_candidate_id: None,
        creation_request_id: None,
        package_seal_digest: None,
        recommendation_reference: None,
        ranking_applied: false,
        selects_candidate: false,
        creates_goal: false,
        creates_intent: false,
        adapter_invoked: false,
        planner_invoked: false,
        ownership_transferred: false,
        mutates_recommendation_engine: false,
        handoff_command: None,
        note: "scored".into(),
        authority_effect: DecisionCandidateScore::AUTHORITY_EFFECT_NONE.into(),
    };
    repo.upsert_candidate_score(&score).unwrap();
    let mut replaced = score.clone();
    replaced.score.total = 1;
    assert!(matches!(
        repo.upsert_candidate_score(&replaced),
        Err(DatabaseError::ImmutableArtifact(_))
    ));
    assert_eq!(
        repo.list_candidate_scores("ws-1").unwrap()[0].score.total,
        70
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
fn decision_engine_repository_rejects_invalidated_provenance_mutation() {
    let (_dir, db) = database();
    db.connection()
        .execute(
            "INSERT INTO decision_engine_intake_candidate (
                workspace_id, intake_candidate_id, intake_receipt_reference,
                recommendation_reference, package_seal_digest, acceptance_reference,
                compatibility_version, state, created_at, updated_at,
                lifecycle_state, lifecycle_reason, lifecycle_updated_at
             ) VALUES ('ws-1','intake-prov','receipt-1','rec-1','seal-1','accept-1',
                       'v1','candidate','t0','t0','invalidated','reason','t0')",
            [],
        )
        .unwrap();
    let repo = DecisionEngineRepository::new(&db);
    let mut candidate = repo
        .get_intake_candidate("ws-1", "intake-prov")
        .unwrap()
        .unwrap();
    candidate.package_seal_digest = "tampered-seal".into();
    candidate.lifecycle.updated_at = "t1".into();
    assert!(matches!(
        repo.upsert_intake_candidate(&candidate),
        Err(DatabaseError::ImmutableArtifact(_))
    ));
    let stored = repo
        .get_intake_candidate("ws-1", "intake-prov")
        .unwrap()
        .unwrap();
    assert_eq!(stored.package_seal_digest, "seal-1");
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
    overlay.source_id = "source-owned".into();
    overlay.decision_state = DecisionState::Accepted;
    assert!(repo.upsert_overlay(&overlay).is_err());
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

#[test]
fn task_repository_preserves_completed_progress_evidence() {
    let (_dir, db) = database();
    let repo = TaskGraphRepository::new(&db);
    let task = WorkspaceTask::new(
        "ws-1",
        "Done task",
        None,
        WorkspaceTaskPriority::Medium,
    )
    .unwrap()
    .transition_status(WorkspaceTaskStatus::Completed, "done")
    .unwrap();
    assert_eq!(task.progress_percent, 100);
    repo.upsert_task(&task).unwrap();

    let mut weakened = task.clone();
    weakened.progress_percent = 40;
    weakened.title = "Still completed".into();
    assert!(matches!(
        repo.upsert_task(&weakened),
        Err(DatabaseError::ImmutableArtifact(_))
    ));

    let mut cleared_explanation = task.clone();
    cleared_explanation.explanation = String::new();
    assert!(matches!(
        repo.upsert_task(&cleared_explanation),
        Err(DatabaseError::ImmutableArtifact(_))
    ));

    let stored = repo.get_task(task.id.as_str()).unwrap().unwrap();
    assert_eq!(stored.progress_percent, 100);
    assert_eq!(stored.explanation, "done");
}

#[test]
fn repository_error_classification_distinguishes_immutable_artifact() {
    let err = DatabaseError::ImmutableArtifact("score locked".into());
    assert!(matches!(err, DatabaseError::ImmutableArtifact(_)));
    let transition = DatabaseError::InvalidTransition("bad transition".into());
    assert!(matches!(transition, DatabaseError::InvalidTransition(_)));
}
