//! Post–consumption audit — runtime diagnostic trust, compatibility & lineage tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeOverview, RuntimeCapabilityMap, RuntimeConsistencyVerification,
    RuntimeDependencyGraph, RuntimeDiagnosticArchive, RuntimeDiagnosticCompatibilityContract,
    RuntimeDiagnosticContinuityRecord, RuntimeDiagnosticCurrency,
    RuntimeDiagnosticEvidenceBundle, RuntimeDiagnosticEvolutionReport,
    RuntimeDiagnosticInterpretationView, RuntimeDiagnosticLifecyclePhase,
    RuntimeDiagnosticLineageRecord, RuntimeDiagnosticLineageValidation,
    RuntimeDiagnosticProvenance, RuntimeDiagnosticRestorationView, RuntimeDiagnosticSnapshot,
    RuntimeDiagnosticTrustRecord, RuntimeProjectionBoundaryRegistry, WorkspaceRuntimeCoherence,
    WorkspaceRuntimeContext, WorkspaceRuntimeHealth, WorkspaceRuntimeIntegrationContract,
    RUNTIME_DIAGNOSTICS_CONTRACT_VERSION, RUNTIME_DIAGNOSTICS_SCHEMA_VERSION,
};

fn bundle(workspace_id: &str) -> (
    RuntimeDiagnosticSnapshot,
    RuntimeDiagnosticProvenance,
    RuntimeDiagnosticContinuityRecord,
    RuntimeDiagnosticEvolutionReport,
    RuntimeDiagnosticInterpretationView,
    RuntimeDiagnosticArchive,
    RuntimeDiagnosticRestorationView,
    String,
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
        Some(overview.id.as_str()),
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
    let entry_id = archive
        .archive_evidence(&evidence, "a0")
        .unwrap()
        .id
        .clone();
    let entry = archive
        .entries
        .iter()
        .find(|e| e.id == entry_id)
        .unwrap()
        .clone();
    let restoration =
        RuntimeDiagnosticRestorationView::rehydrate(&archive, &entry, Some(&evidence), "r0");
    (
        snapshot,
        provenance,
        continuity,
        evolution,
        interpretation,
        archive,
        restoration,
        entry_id,
    )
}

#[test]
fn case1_compatibility_is_identity_only_without_migration() {
    let compatibility = RuntimeDiagnosticCompatibilityContract::canonical();
    assert!(compatibility.all_current());
    assert!(!compatibility.may_migrate);
    assert_eq!(RUNTIME_DIAGNOSTICS_CONTRACT_VERSION, "runtime_diagnostics:v1");
    assert_eq!(RUNTIME_DIAGNOSTICS_SCHEMA_VERSION, 1);
    assert!(compatibility.attempt_migrate().is_err());
    assert!(RuntimeDiagnosticCompatibilityContract::attempt_execute().is_err());
}

#[test]
fn case2_lineage_tracks_currency_and_ordering() {
    let (snapshot, provenance, continuity, evolution, _i, _a, restoration, entry_id) =
        bundle("ws-trust");
    let current = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        None,
        None,
        false,
    );
    assert_eq!(current.currency, RuntimeDiagnosticCurrency::Current);
    assert!(current.ordering_ok());

    let restored = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        Some(&entry_id),
        Some(&restoration.id),
        false,
    );
    assert_eq!(restored.currency, RuntimeDiagnosticCurrency::RestoredView);
    assert!(restored.ordering_ok());
    assert!(restored.attempt_mutate().is_err());
}

#[test]
fn case3_lineage_validation_requires_read_only_restoration() {
    let (snapshot, provenance, continuity, evolution, _i, _a, restoration, entry_id) =
        bundle("ws-trust");
    let lineage = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        Some(&entry_id),
        Some(&restoration.id),
        false,
    );
    let validation = RuntimeDiagnosticLineageValidation::validate(&lineage, Some(&restoration));
    assert!(validation.passed());
    assert!(validation.restoration_read_only);
    assert!(validation.attempt_repair().is_err());
}

#[test]
fn case4_trust_record_exposes_producer_version_and_limitations() {
    let (snapshot, provenance, continuity, evolution, interpretation, _a, restoration, entry_id) =
        bundle("ws-trust");
    let lineage = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        Some(&entry_id),
        Some(&restoration.id),
        false,
    );
    let trust = RuntimeDiagnosticTrustRecord::attest(
        &interpretation,
        &lineage,
        &RuntimeDiagnosticCompatibilityContract::canonical(),
        &RuntimeProjectionBoundaryRegistry::canonical(),
    );
    assert!(trust.trustworthy());
    assert!(!trust.is_authoritative());
    assert!(!trust.is_decision());
    assert!(trust.attempt_promote_to_decision().is_err());
    assert!(trust
        .limitations
        .iter()
        .any(|l| l.contains(RUNTIME_DIAGNOSTICS_CONTRACT_VERSION)));
    assert!(trust
        .limitations
        .iter()
        .any(|l| l.contains("currency=restored_view")));
    assert!(!trust.finding_ids.is_empty());
}

#[test]
fn case5_historical_currency_when_superseded_without_restoration() {
    let (snapshot, provenance, continuity, evolution, _i, _a, _r, _e) = bundle("ws-trust");
    let lineage = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        None,
        None,
        true,
    );
    assert_eq!(lineage.currency, RuntimeDiagnosticCurrency::Historical);
    assert!(lineage.ordering_ok());
    let validation = RuntimeDiagnosticLineageValidation::validate(&lineage, None);
    assert!(validation.passed());
}
