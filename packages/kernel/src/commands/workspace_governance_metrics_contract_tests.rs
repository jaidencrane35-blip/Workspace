//! Sprint 162 — Governance metrics contract tests.

use workspace_domain::{
    AttentionReason, AttentionSignal, AttentionSourceType, GovernanceMetricKind,
    GovernanceMetricsContract, RecommendationConfidence, RecommendationEvidence,
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
            related_attention_id: None,
            attention_reasons: vec![AttentionReason::new(
                AttentionSourceType::Continuity,
                AttentionSignal::ResumableWork,
                30,
                "k",
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
fn case1_append_only_observational_metrics() {
    let proposal = proposal();
    let mut m = GovernanceMetricsContract::new(&proposal);
    m.record(
        GovernanceMetricKind::ReviewDuration,
        90_000,
        "ms",
        "t0",
        Some(proposal.id.clone()),
    )
    .unwrap();
    m.record(
        GovernanceMetricKind::ConflictFrequency,
        2_000,
        "count_milli",
        "t1",
        None,
    )
    .unwrap();
    m.record(
        GovernanceMetricKind::ObligationCompletionRate,
        750,
        "permille",
        "t2",
        None,
    )
    .unwrap();
    m.record(
        GovernanceMetricKind::ArchiveGrowth,
        4_000,
        "count_milli",
        "t3",
        None,
    )
    .unwrap();
    m.record(
        GovernanceMetricKind::PublicationReadinessTrend,
        1_000,
        "state_index_milli",
        "t4",
        None,
    )
    .unwrap();
    m.record(
        GovernanceMetricKind::ApprovalLatency,
        45_000,
        "ms",
        "t5",
        None,
    )
    .unwrap();
    assert!(m.append_only);
    assert_eq!(m.samples.len(), 6);
    assert_eq!(m.samples_of(GovernanceMetricKind::ReviewDuration).len(), 1);
}

#[test]
fn case2_metrics_cannot_optimise_or_adapt() {
    let proposal = proposal();
    let m = GovernanceMetricsContract::new(&proposal);
    assert!(!m.may_optimise());
    assert!(!m.may_adapt_automatically());
    assert!(m.attempt_optimise().is_err());
    assert!(m.attempt_adapt().is_err());
    assert!(GovernanceMetricsContract::attempt_execute().is_err());
}
