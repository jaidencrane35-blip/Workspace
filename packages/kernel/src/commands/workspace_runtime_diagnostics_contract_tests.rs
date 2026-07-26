//! Sprints 176–181 — Workspace runtime diagnostics contract tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeOverview, RuntimeArchitectureReview, RuntimeCapabilityMap,
    RuntimeConsistencyVerification, RuntimeDependencyGraph, RuntimeDiagnosticSnapshot,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract,
};

fn bundle(workspace_id: &str) -> (
    WorkspaceRuntimeContext,
    WorkspaceRuntimeHealth,
    RuntimeDependencyGraph,
    RuntimeCapabilityMap,
    RuntimeDiagnosticSnapshot,
    RuntimeConsistencyVerification,
    OperatorRuntimeOverview,
    RuntimeArchitectureReview,
) {
    let ctx = WorkspaceRuntimeContext::assemble(
        workspace_id,
        "t0",
        None,
        None,
        None,
        None,
        None,
        GovernanceRuntimeSummary::empty(),
    );
    let health = WorkspaceRuntimeHealth::observe(&ctx);
    let cognition = CognitionContextProjection::project_all(&ctx);
    let graph = RuntimeDependencyGraph::canonical(workspace_id);
    let capabilities = RuntimeCapabilityMap::inventory(workspace_id);
    let snapshot = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &cognition,
        &graph,
        &capabilities,
        "t-snap",
    );
    let integration = WorkspaceRuntimeIntegrationContract::audit_default(workspace_id);
    let verification = RuntimeConsistencyVerification::verify(
        &graph,
        &capabilities,
        &ctx,
        &integration,
        &snapshot,
        "t-check",
    );
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence = WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
    let overview = OperatorRuntimeOverview::project(
        &ctx,
        &health,
        &operator,
        &snapshot,
        &verification,
        &graph,
        &capabilities,
        &coherence,
    );
    let review = RuntimeArchitectureReview::review(
        &graph,
        &capabilities,
        &verification,
        &overview,
        &coherence,
    );
    (
        ctx,
        health,
        graph,
        capabilities,
        snapshot,
        verification,
        overview,
        review,
    )
}

#[test]
fn case1_dependency_graph_is_acyclic_and_non_executing() {
    let graph = RuntimeDependencyGraph::canonical("ws-diag");
    assert!(!graph.has_cycles());
    assert!(!graph.required_edges().is_empty());
    assert_eq!(graph.authority_effect, "none");
    assert!(!graph.may_execute());
    assert!(RuntimeDependencyGraph::attempt_execute().is_err());
}

#[test]
fn case2_capability_map_keeps_gateway_sole_execution_authority() {
    let map = RuntimeCapabilityMap::inventory("ws-diag");
    assert!(map.gateway_is_sole_execution_authority());
    assert!(!map.cognition_entries().is_empty());
    assert!(!map.may_execute());
    assert!(RuntimeCapabilityMap::attempt_execute().is_err());
}

#[test]
fn case3_diagnostic_snapshot_is_observational() {
    let (_ctx, _health, _graph, _cap, snapshot, _v, _o, _r) = bundle("ws-diag");
    assert!(!snapshot.dependency_has_cycles);
    assert!(snapshot.gateway_sole_execution);
    assert!(snapshot.governance.publication_blocked);
    assert!(!snapshot.may_execute());
    assert!(RuntimeDiagnosticSnapshot::attempt_execute().is_err());
}

#[test]
fn case4_consistency_verification_never_repairs() {
    let (_ctx, _h, _g, _c, _s, verification, _o, _r) = bundle("ws-diag");
    assert!(!verification.has_errors());
    assert!(!verification.may_repair_automatically());
    assert!(verification.attempt_repair().is_err());
    assert!(RuntimeConsistencyVerification::attempt_execute().is_err());
}

#[test]
fn case5_operator_runtime_overview_is_projection_only() {
    let (_ctx, _h, _g, _c, _s, _v, overview, _r) = bundle("ws-diag");
    assert!(overview.publication_blocked);
    assert!(overview.coherence_ok);
    assert!(!overview.consistency_has_errors);
    assert!(!overview.may_execute());
    assert!(OperatorRuntimeOverview::attempt_execute().is_err());
}

#[test]
fn case6_architecture_review_passes_without_authority() {
    let (_ctx, _h, _g, _c, _s, _v, _o, review) = bundle("ws-diag");
    assert!(review.passed());
    assert!(review.ownership_ok);
    assert!(review.dependency_direction_ok);
    assert!(review.projection_layering_ok);
    assert!(review.authority_boundaries_ok);
    assert!(review.aggregate_cohesion_ok);
    assert!(!review.may_execute());
    assert!(RuntimeArchitectureReview::attempt_execute().is_err());
}
