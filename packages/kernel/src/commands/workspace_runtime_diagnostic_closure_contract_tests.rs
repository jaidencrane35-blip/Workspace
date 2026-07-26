//! Post–trust audit — runtime diagnostic closure, catalog, interop, explanation integrity.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeExplanation, OperatorRuntimeOverview, RuntimeCapabilityMap,
    RuntimeConsistencyVerification, RuntimeDependencyGraph, RuntimeDiagnosticArchive,
    RuntimeDiagnosticCompatibilityContract, RuntimeDiagnosticContractCatalog,
    RuntimeDiagnosticContinuityRecord, RuntimeDiagnosticEvidenceBundle,
    RuntimeDiagnosticEvolutionReport, RuntimeDiagnosticExplanationIntegrity,
    RuntimeDiagnosticInteropContract, RuntimeDiagnosticInteropDomain,
    RuntimeDiagnosticInterpretationView, RuntimeDiagnosticLifecycleClosure,
    RuntimeDiagnosticLifecyclePhase, RuntimeDiagnosticLineageRecord,
    RuntimeDiagnosticProvenance, RuntimeDiagnosticRestorationView, RuntimeDiagnosticSnapshot,
    RuntimeDiagnosticTrustRecord, RuntimeProjectionBoundaryRegistry, WorkspaceRuntimeCoherence,
    WorkspaceRuntimeContext, WorkspaceRuntimeHealth, WorkspaceRuntimeIntegrationContract,
};

fn bundle(workspace_id: &str) -> (
    RuntimeDiagnosticSnapshot,
    RuntimeDiagnosticProvenance,
    RuntimeDiagnosticContinuityRecord,
    RuntimeDiagnosticEvolutionReport,
    RuntimeDiagnosticInterpretationView,
    OperatorRuntimeOverview,
    RuntimeDiagnosticLineageRecord,
    RuntimeDiagnosticTrustRecord,
    RuntimeDiagnosticExplanationIntegrity,
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
    let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    let evidence =
        RuntimeDiagnosticEvidenceBundle::seal(&evolution, &provenance, &continuity, "seal0");
    let mut archive = RuntimeDiagnosticArchive::new(workspace_id);
    let entry = archive.archive_evidence(&evidence, "a0").unwrap().clone();
    let restoration =
        RuntimeDiagnosticRestorationView::rehydrate(&archive, &entry, Some(&evidence), "r0");
    let lineage = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        Some(&entry.id),
        Some(&restoration.id),
        false,
    );
    let trust = RuntimeDiagnosticTrustRecord::attest(
        &interpretation,
        &lineage,
        &RuntimeDiagnosticCompatibilityContract::canonical(),
        &RuntimeProjectionBoundaryRegistry::canonical(),
    );
    let integrity = RuntimeDiagnosticExplanationIntegrity::compose(
        &continuity,
        &interpretation,
        &trust,
        &lineage,
    );
    (
        snapshot,
        provenance,
        continuity,
        evolution,
        interpretation,
        overview,
        lineage,
        trust,
        integrity,
    )
}

#[test]
fn case1_lifecycle_closure_defines_terminal_and_forbids_mutation() {
    let (_s, _p, _c, _e, _i, _o, lineage, _t, _x) = bundle("ws-close");
    let closure = RuntimeDiagnosticLifecycleClosure::evaluate(&lineage);
    assert!(closure.closed());
    assert!(closure.terminal);
    assert!(closure.restored_view_is_currency_not_phase);
    assert!(RuntimeDiagnosticLifecyclePhase::Archived.is_terminal());
    assert!(closure.attempt_mutate().is_err());
}

#[test]
fn case2_contract_catalog_is_discoverable_and_non_executable() {
    let catalog = RuntimeDiagnosticContractCatalog::canonical();
    assert!(catalog.none_executable());
    assert_eq!(catalog.discover().len(), catalog.entries.len());
    assert!(catalog.attempt_dynamic_load().is_err());
    assert!(RuntimeDiagnosticContractCatalog::attempt_execute().is_err());
    let report =
        catalog.compatibility_report(&RuntimeDiagnosticCompatibilityContract::canonical());
    assert!(report.all_compatible());
    assert!(!report.may_migrate);
}

#[test]
fn case3_interop_keeps_domains_separate() {
    let interop = RuntimeDiagnosticInteropContract::canonical();
    assert!(interop.boundaries_respected());
    let experience = interop
        .entries
        .iter()
        .find(|e| e.domain == RuntimeDiagnosticInteropDomain::ExperienceTranslation)
        .unwrap();
    assert!(!experience.may_reference_read_only);
    assert!(!experience.may_own);
    assert!(interop.attempt_own_foreign().is_err());
}

#[test]
fn case4_explanation_integrity_answers_what_why_who_version_currency() {
    let (_s, _p, _c, _e, _i, _o, _l, trust, integrity) = bundle("ws-close");
    assert!(integrity.answers_complete);
    assert!(!integrity.what_changed.is_empty());
    assert!(!integrity.why_changed.is_empty());
    assert!(integrity.produced_by.contains("version="));
    assert_eq!(integrity.contract_version, trust.producer.contract_version);
    assert!(!integrity.may_execute());
    assert!(RuntimeDiagnosticExplanationIntegrity::attempt_execute().is_err());
}

#[test]
fn case5_operator_explanation_with_trust_stays_non_actionable() {
    let (_s, provenance, continuity, evolution, interpretation, overview, _l, _t, integrity) =
        bundle("ws-close");
    let explanation = OperatorRuntimeExplanation::explain_with_trust(
        &overview,
        &provenance,
        &continuity,
        &evolution,
        &interpretation,
        &integrity,
    );
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.exposes_actions());
    assert!(explanation.why.iter().any(|w| w.starts_with("produced_by=")));
    assert!(explanation.why.iter().any(|w| w.starts_with("currency=")));
    assert!(explanation
        .why
        .iter()
        .any(|w| w.contains("explanation_integrity_complete=true")));
}
