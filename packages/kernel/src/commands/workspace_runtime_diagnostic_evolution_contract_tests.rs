//! Post–continuity audit — runtime diagnostic evolution contract tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeExplanation, OperatorRuntimeOverview, RuntimeArchitectureOwnershipRegistry,
    RuntimeCapabilityMap, RuntimeConsistencyVerification, RuntimeDependencyGraph,
    RuntimeDiagnosticComparison, RuntimeDiagnosticContinuityRecord,
    RuntimeDiagnosticDeltaKind, RuntimeDiagnosticEvolutionReport,
    RuntimeDiagnosticLifecyclePhase, RuntimeDiagnosticLifecycleRecord,
    RuntimeDiagnosticOwnershipRole, RuntimeDiagnosticProvenance, RuntimeDiagnosticSnapshot,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract,
};

fn fixtures(workspace_id: &str) -> (
    WorkspaceRuntimeContext,
    WorkspaceRuntimeHealth,
    RuntimeDependencyGraph,
    RuntimeCapabilityMap,
    RuntimeDiagnosticSnapshot,
    RuntimeConsistencyVerification,
    WorkspaceRuntimeCoherence,
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
    let snapshot = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &cognition,
        &graph,
        &capabilities,
        "t-a",
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
    (
        ctx,
        health,
        graph,
        capabilities,
        snapshot,
        verification,
        coherence,
        overview,
    )
}

#[test]
fn case1_ownership_registry_keeps_operator_as_projection() {
    let registry = RuntimeArchitectureOwnershipRegistry::canonical();
    assert!(registry.operator_artifacts_are_projections_only());
    assert_eq!(
        registry.owner_of("RuntimeDependencyGraph"),
        Some(RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics)
    );
    assert_eq!(
        registry.owner_of("Kernel WorkspaceHealth"),
        Some(RuntimeDiagnosticOwnershipRole::KernelLifecycleHealth)
    );
}

#[test]
fn case2_comparison_is_structured_and_non_executing() {
    let (ctx, health, graph, capabilities, snap_a, _v, _c, _o) = fixtures("ws-evo");
    let snap_b = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &CognitionContextProjection::project_all(&ctx),
        &graph,
        &capabilities,
        "t-b",
    );
    let comparison = RuntimeDiagnosticComparison::compare(&snap_a, &snap_b);
    assert!(comparison
        .deltas
        .iter()
        .any(|d| d.kind == RuntimeDiagnosticDeltaKind::None));
    assert!(!comparison.may_execute());
    assert!(RuntimeDiagnosticComparison::attempt_execute().is_err());
}

#[test]
fn case3_lifecycle_phases_are_ordered_and_immutable() {
    assert!(RuntimeDiagnosticLifecyclePhase::Captured
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::Provenanced));
    assert!(RuntimeDiagnosticLifecyclePhase::Provenanced
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::ContinuityLinked));
    assert!(!RuntimeDiagnosticLifecyclePhase::Captured
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::Superseded));

    let (_ctx, _h, _g, _cap, snapshot, _v, _coh, _o) = fixtures("ws-evo");
    let captured = RuntimeDiagnosticLifecycleRecord::captured(&snapshot, "lc0");
    assert_eq!(captured.phase, RuntimeDiagnosticLifecyclePhase::Captured);
    assert!(!captured.may_mutate());
    assert!(captured.attempt_mutate().is_err());
}

#[test]
fn case4_evolution_validates_continuity_and_blocks_repair() {
    let (ctx, health, graph, capabilities, snap_a, verification, coherence, overview) =
        fixtures("ws-evo");
    let provenance = RuntimeDiagnosticProvenance::from_capture(
        &snap_a,
        &ctx,
        &health,
        &graph,
        &capabilities,
        &verification,
        &coherence,
        Some(overview.id.as_str()),
    );
    let continuity =
        RuntimeDiagnosticContinuityRecord::link(None, &snap_a, &provenance, "c0");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snap_a,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval0",
    );
    assert!(evolution.passed());
    assert!(evolution.continuity_consistent);
    assert!(!evolution.may_repair());
    assert!(evolution.attempt_repair().is_err());
    assert!(RuntimeDiagnosticEvolutionReport::attempt_execute().is_err());
}

#[test]
fn case5_operator_explanation_with_evolution_exposes_change_not_actions() {
    let (ctx, health, graph, capabilities, snap_a, verification, coherence, overview) =
        fixtures("ws-evo");
    let provenance = RuntimeDiagnosticProvenance::from_capture(
        &snap_a,
        &ctx,
        &health,
        &graph,
        &capabilities,
        &verification,
        &coherence,
        Some(overview.id.as_str()),
    );
    let continuity =
        RuntimeDiagnosticContinuityRecord::link(None, &snap_a, &provenance, "c0");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snap_a,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval0",
    );
    let explanation = OperatorRuntimeExplanation::explain_with_evolution(
        &overview,
        &provenance,
        &continuity,
        &evolution,
    );
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.exposes_actions());
    assert!(explanation
        .why
        .iter()
        .any(|w| w.contains("permission_gateway_only")));
    assert!(explanation.why.iter().any(|w| w.contains("actions=none")));
}
