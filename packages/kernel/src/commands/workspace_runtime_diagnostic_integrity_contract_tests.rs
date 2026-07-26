//! Post–evolution audit — runtime diagnostic evidence integrity & retention tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeExplanation, OperatorRuntimeOverview, RuntimeArchitectureOwnershipRegistry,
    RuntimeCapabilityMap, RuntimeConsistencyVerification, RuntimeDependencyGraph,
    RuntimeDiagnosticArchive, RuntimeDiagnosticContinuityRecord, RuntimeDiagnosticEvidenceBundle,
    RuntimeDiagnosticEvolutionReport, RuntimeDiagnosticHistoricalIntegrity,
    RuntimeDiagnosticLifecyclePhase, RuntimeDiagnosticProvenance, RuntimeDiagnosticSnapshot,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract,
};

fn evolved_bundle(workspace_id: &str) -> (
    RuntimeDiagnosticEvolutionReport,
    RuntimeDiagnosticProvenance,
    RuntimeDiagnosticContinuityRecord,
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
    let provenance = RuntimeDiagnosticProvenance::from_capture(
        &snapshot,
        &ctx,
        &health,
        &graph,
        &capabilities,
        &verification,
        &coherence,
        Some(&overview),
    );
    let continuity =
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "c0");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval0",
    );
    (evolution, provenance, continuity, overview)
}

#[test]
fn case1_ownership_validation_is_conflict_free_and_versioned() {
    let validation = RuntimeArchitectureOwnershipRegistry::canonical().validate();
    assert!(validation.conflict_free);
    assert_eq!(
        validation.registry_version,
        RuntimeArchitectureOwnershipRegistry::REGISTRY_VERSION
    );
    assert!(!validation.may_mutate_registry());
}

#[test]
fn case2_evolution_and_evidence_cannot_become_decisions() {
    let (evolution, provenance, continuity, _overview) = evolved_bundle("ws-int");
    assert!(evolution.passed());
    assert!(!evolution.is_authoritative());
    assert!(!evolution.is_decision());
    assert!(evolution.attempt_promote_to_decision().is_err());

    let evidence =
        RuntimeDiagnosticEvidenceBundle::seal(&evolution, &provenance, &continuity, "seal0");
    assert!(evidence.is_complete());
    assert!(evidence.cites_evolution(&evolution.id));
    assert!(!evidence.is_authoritative());
    assert!(!evidence.is_decision());
    assert!(evidence.attempt_promote_to_decision().is_err());
    assert!(RuntimeDiagnosticEvidenceBundle::attempt_execute().is_err());
}

#[test]
fn case3_archive_is_append_only_without_deletion() {
    let (evolution, provenance, continuity, _overview) = evolved_bundle("ws-int");
    let evidence =
        RuntimeDiagnosticEvidenceBundle::seal(&evolution, &provenance, &continuity, "seal0");
    let mut archive = RuntimeDiagnosticArchive::new("ws-int");
    assert!(archive.archive_evidence(&evidence, "a0").is_ok());
    assert_eq!(archive.entries.len(), 1);
    assert!(archive
        .mark_superseded("snap-old", "snap-new", "a1")
        .is_ok());
    assert_eq!(archive.entries.len(), 2);
    assert!(!archive.may_delete());
    assert!(!archive.may_mutate_entries());
    assert!(archive.attempt_delete().is_err());
    assert!(archive.attempt_mutate_entry().is_err());
    assert!(RuntimeDiagnosticArchive::attempt_execute().is_err());
}

#[test]
fn case4_historical_integrity_passes_without_repair() {
    let (evolution, provenance, continuity, _overview) = evolved_bundle("ws-int");
    let evidence =
        RuntimeDiagnosticEvidenceBundle::seal(&evolution, &provenance, &continuity, "seal0");
    let mut archive = RuntimeDiagnosticArchive::new("ws-int");
    archive.archive_evidence(&evidence, "a0").unwrap();
    let ownership = RuntimeArchitectureOwnershipRegistry::canonical().validate();
    let integrity = RuntimeDiagnosticHistoricalIntegrity::verify(
        &evidence,
        &archive,
        &evolution,
        None,
        &ownership,
    );
    assert!(integrity.passed());
    assert!(integrity.reports_non_authoritative);
    assert!(integrity.retention_forbids_deletion);
    assert!(!integrity.may_repair());
    assert!(integrity.attempt_repair().is_err());
}

#[test]
fn case5_operator_explanation_stays_non_actionable_with_integrity() {
    let (evolution, provenance, continuity, overview) = evolved_bundle("ws-int");
    let explanation = OperatorRuntimeExplanation::explain_with_evolution(
        &overview,
        &provenance,
        &continuity,
        &evolution,
    );
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.exposes_actions());
    assert!(explanation.why.iter().any(|w| w.contains("authoritative=false")));
    assert!(explanation.why.iter().any(|w| w.contains("actions=none")));
}
