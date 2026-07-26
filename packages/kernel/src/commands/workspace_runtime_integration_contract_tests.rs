//! Sprints 170–175 — Workspace runtime integration contract tests.

use workspace_domain::{
    CognitionContextProjection, CognitionProjectionKind, GovernanceRuntimeSummary,
    OperatorContextProjection, PublicationReadinessState, WorkspaceRuntimeCoherence,
    WorkspaceRuntimeContext, WorkspaceRuntimeHealth, WorkspaceRuntimeIntegrationContract,
};

#[test]
fn case1_governance_visible_never_authoritative() {
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-runtime");
    assert!(integration.all_non_authoritative());
    assert!(integration.points.iter().all(|p| p.observable && !p.authoritative));
    assert!(!integration.may_execute());
    assert!(!integration.may_grant_authority());
    assert!(WorkspaceRuntimeIntegrationContract::attempt_execute().is_err());
}

#[test]
fn case2_runtime_context_is_read_only_and_does_not_own_state() {
    let governance = GovernanceRuntimeSummary::from_labels(
        Some("governance_decision_package:x".into()),
        Some("in_review".into()),
        Some("warnings".into()),
        Some(PublicationReadinessState::ReadyForPublication),
        0,
        1,
    );
    assert!(governance.publication_blocked);
    let ctx = WorkspaceRuntimeContext::assemble(
        "ws-runtime",
        "2026-07-27T00:00:00Z",
        None,
        None,
        None,
        None,
        None,
        governance,
    );
    assert!(!ctx.owns_workspace_state());
    assert!(!ctx.may_execute());
    assert!(WorkspaceRuntimeContext::attempt_execute().is_err());
    assert!(WorkspaceRuntimeContext::attempt_mutate().is_err());
}

#[test]
fn case3_cognition_projections_expose_context_only() {
    let ctx = WorkspaceRuntimeContext::assemble(
        "ws-runtime",
        "t0",
        None,
        None,
        None,
        None,
        None,
        GovernanceRuntimeSummary::empty(),
    );
    let projections = CognitionContextProjection::project_all(&ctx);
    assert_eq!(projections.len(), 4);
    assert!(projections
        .iter()
        .any(|p| p.kind == CognitionProjectionKind::Attention));
    assert!(projections
        .iter()
        .any(|p| p.kind == CognitionProjectionKind::Decision));
    for p in &projections {
        assert!(!p.may_change_scoring());
        assert!(!p.may_execute());
        assert!(p.governance_publication_blocked);
    }
    assert!(CognitionContextProjection::attempt_execute().is_err());
}

#[test]
fn case4_operator_health_coherence_publication_blocked() {
    let ctx = WorkspaceRuntimeContext::assemble(
        "ws-runtime",
        "t0",
        None,
        None,
        None,
        None,
        None,
        GovernanceRuntimeSummary::empty(),
    );
    let health = WorkspaceRuntimeHealth::observe(&ctx);
    assert!(health.publication_blocked);
    assert!(!health.may_prescribe());
    assert!(health.attempt_prescribe().is_err());
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    assert!(operator.publication_blocked);
    assert_eq!(operator.publication_readiness_label, "blocked");
    assert!(OperatorContextProjection::attempt_execute().is_err());

    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-runtime");
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
    assert!(coherence.coherent);
    assert_eq!(
        coherence.chain,
        WorkspaceRuntimeCoherence::CANONICAL_CHAIN
            .iter()
            .map(|s| (*s).into())
            .collect::<Vec<String>>()
    );
    assert!(WorkspaceRuntimeCoherence::attempt_execute().is_err());
}
