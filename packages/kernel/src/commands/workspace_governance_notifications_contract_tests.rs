//! Sprint 160 — Governance notification contract tests.

use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, GovernanceNotificationContract,
    GovernanceNotificationDeliveryStatus, GovernanceNotificationKind,
    GovernanceNotificationTrigger, RecommendationConfidence, RecommendationEvidence,
    RecommendationGovernanceRecord, RecommendationItem, RecommendationKind,
    RecommendationLifecycleState,
};

fn proposal() -> workspace_domain::OutcomeAdaptationProposal {
    let mut record = RecommendationGovernanceRecord::from_recommendation_item(
        &RecommendationItem {
            id: "recommendation:continue_work:focus-1".into(),
            kind: RecommendationKind::ContinueWork,
            title: "Continue".into(),
            reason: "Resumable".into(),
            evidence: vec![RecommendationEvidence {
                id: "ev".into(),
                source_model: "continuity".into(),
                source_ref: "c:1".into(),
                summary: "s".into(),
            }],
            impact: "i".into(),
            confidence: RecommendationConfidence::Medium,
            related_attention_id: Some("a:1".into()),
            attention_reasons: vec![AttentionReason::new(
                AttentionSourceType::Continuity,
                AttentionSignal::ResumableWork,
                30,
                "continuity.resumable",
            )],
            related_task_id: None,
            related_purpose_label: None,
            related_decision_id: None,
            lifecycle_state: None,
            lifecycle_presented_at: None,
            lifecycle_resolved_at: None,
            lifecycle_resolution_type: None,
            explanation: None,
                outcome: None,
                decision_context: None,
                decision_readiness: None,
                decision_boundary: None,
                decision_confirmation: None,
                decision_intake: None,
                decision_intake_inspection: None,
                decision_intake_compatibility: None,
                decision_intake_proceed_denial: None,
                decision_intake_package_seal: None,
                decision_intake_adapter_preparation: None,
                decision_handoff_request: None,
                authority_effect: RecommendationItem::AUTHORITY_EFFECT_NONE.into(),
        },
        "t0",
    );
    record
        .transition(RecommendationLifecycleState::Available, "t1", None)
        .unwrap();
    record
        .transition(RecommendationLifecycleState::Presented, "t2", None)
        .unwrap();
    record.accept("t3", None).unwrap();
    let outcome = record.record_outcome("t4").unwrap();
    let mut p = outcome.to_adaptation_proposal("a", "b", "c");
    p.require_review().unwrap();
    p
}

#[test]
fn case1_emit_deliver_acknowledge_and_remind() {
    let proposal = proposal();
    let mut n = GovernanceNotificationContract::new(&proposal);
    let id = n.emit(
        GovernanceNotificationKind::ReviewAssigned,
        GovernanceNotificationTrigger::WorkflowAssigned,
        vec!["reviewer-1".into()],
        "assignment:1",
        "Please review",
        None,
        Some("t+7d".into()),
    );
    n.mark_delivered(&id).unwrap();
    n.schedule_reminder(&id, "t+1d").unwrap();
    n.acknowledge(&id, "reviewer-1").unwrap();
    assert_eq!(
        n.notifications[0].delivery_status,
        GovernanceNotificationDeliveryStatus::Acknowledged
    );
    assert!(n.notifications[0].reminder_at.is_some());
}

#[test]
fn case2_expiry_and_no_authority() {
    let proposal = proposal();
    let mut n = GovernanceNotificationContract::new(&proposal);
    let id = n.emit(
        GovernanceNotificationKind::ExpiryWarning,
        GovernanceNotificationTrigger::TtlApproaching,
        vec!["reviewer-1".into()],
        proposal.id.clone(),
        "Expiring soon",
        None,
        Some("t+1h".into()),
    );
    n.expire(&id).unwrap();
    assert!(n.mark_delivered(&id).is_err());
    assert!(!n.may_execute());
    assert!(!n.may_grant_authority());
    assert!(GovernanceNotificationContract::attempt_execute().is_err());
    assert!(GovernanceNotificationContract::attempt_grant_authority().is_err());
}
