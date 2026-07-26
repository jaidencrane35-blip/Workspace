//! Post–181 audit — runtime diagnostic provenance & continuity contract tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeExplanation, OperatorRuntimeOverview, RuntimeCapabilityMap,
    RuntimeConsistencyVerification, RuntimeDependencyGraph, RuntimeDiagnosticContinuityRecord,
    RuntimeDiagnosticOwnershipBoundary, RuntimeDiagnosticProvenance, RuntimeDiagnosticSnapshot,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract,
};

fn capture_pair(workspace_id: &str) -> (
    RuntimeDiagnosticSnapshot,
    RuntimeDiagnosticSnapshot,
    RuntimeDiagnosticProvenance,
    OperatorRuntimeOverview,
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
    let snap_a = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &cognition,
        &graph,
        &capabilities,
        "t-a",
    );
    let snap_b = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &cognition,
        &graph,
        &capabilities,
        "t-b",
    );
    let integration = WorkspaceRuntimeIntegrationContract::audit_default(workspace_id);
    let verification = RuntimeConsistencyVerification::verify(
        &graph,
        &capabilities,
        &ctx,
        &integration,
        &snap_a,
        "t-check",
    );
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence = WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
    let overview = OperatorRuntimeOverview::project(
        &ctx,
        &health,
        &operator,
        &snap_a,
        &verification,
        &graph,
        &capabilities,
        &coherence,
    );
    let provenance = RuntimeDiagnosticProvenance::from_capture(
        &snap_a,
        &ctx,
        &health,
        &graph,
        &capabilities,
        &verification,
        &coherence,
        Some(&overview),
    );
    (snap_a, snap_b, provenance, overview)
}

#[test]
fn case1_ownership_boundary_keeps_diagnostics_observational() {
    let boundary = RuntimeDiagnosticOwnershipBoundary::canonical();
    assert!(boundary.boundaries_respected());
    assert!(!boundary.may_mutate_sources);
    assert!(!boundary.may_prescribe_healing);
    assert!(boundary.attempt_mutate_sources().is_err());
    assert!(boundary.attempt_heal().is_err());
}

#[test]
fn case2_provenance_cites_inputs_and_forbids_history_rewrite() {
    let (snap_a, _b, provenance, _overview) = capture_pair("ws-cont");
    assert!(provenance.cites_snapshot(&snap_a.id));
    assert!(provenance.source_refs.len() >= 6);
    assert!(provenance.ownership.boundaries_respected());
    assert!(!provenance.may_rewrite_history());
    assert!(provenance.attempt_rewrite_history().is_err());
    assert!(RuntimeDiagnosticProvenance::attempt_execute().is_err());
}

#[test]
fn case3_continuity_links_snapshots_without_mutation() {
    let (snap_a, snap_b, provenance, _overview) = capture_pair("ws-cont");
    let initial = RuntimeDiagnosticContinuityRecord::link(None, &snap_a, &provenance, "c0");
    assert!(initial.is_initial());
    assert!(initial
        .what_changed
        .iter()
        .any(|d| d == "initial_diagnostic_snapshot"));

    let next = RuntimeDiagnosticContinuityRecord::link(Some(&snap_a), &snap_b, &provenance, "c1");
    assert_eq!(next.previous_snapshot_id.as_deref(), Some(snap_a.id.as_str()));
    assert_eq!(next.current_snapshot_id, snap_b.id);
    assert!(!next.may_mutate_prior());
    assert!(next.attempt_mutate_prior().is_err());
    assert!(RuntimeDiagnosticContinuityRecord::attempt_execute().is_err());
}

#[test]
fn case4_operator_explanation_is_not_execution_surface() {
    let (snap_a, _b, provenance, overview) = capture_pair("ws-cont");
    let continuity =
        RuntimeDiagnosticContinuityRecord::link(None, &snap_a, &provenance, "c-explain");
    let explanation = OperatorRuntimeExplanation::explain(&overview, &provenance, &continuity);
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.may_execute());
    assert!(explanation.publication_blocked);
    assert!(explanation
        .why
        .iter()
        .any(|w| w.contains("permission_gateway_only")));
    assert!(OperatorRuntimeExplanation::attempt_execute().is_err());
}
