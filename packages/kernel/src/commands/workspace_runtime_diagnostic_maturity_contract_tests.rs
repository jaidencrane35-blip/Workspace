//! Post–closure audit — runtime diagnostic maturity / meta-readiness tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeExplanation, OperatorRuntimeOverview, RuntimeCapabilityMap,
    RuntimeConsistencyVerification, RuntimeDependencyGraph, RuntimeDiagnosticArchive,
    RuntimeDiagnosticCatalogIntegrity, RuntimeDiagnosticCompatibilityContract,
    RuntimeDiagnosticContractCatalog, RuntimeDiagnosticContinuityRecord,
    RuntimeDiagnosticEvidenceBundle, RuntimeDiagnosticEvolutionReport,
    RuntimeDiagnosticExplanationConsistency, RuntimeDiagnosticExplanationIntegrity,
    RuntimeDiagnosticInteropContract, RuntimeDiagnosticInterpretationView,
    RuntimeDiagnosticLifecycleClosure, RuntimeDiagnosticLifecyclePhase,
    RuntimeDiagnosticLineageRecord, RuntimeDiagnosticMaturityAssessment,
    RuntimeDiagnosticMaturityLevel, RuntimeDiagnosticProvenance,
    RuntimeDiagnosticReferenceIntegrity, RuntimeDiagnosticRestorationView,
    RuntimeDiagnosticSnapshot, RuntimeDiagnosticTrustRecord, RuntimeProjectionBoundaryRegistry,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract,
};

fn maturity_bundle(workspace_id: &str) -> RuntimeDiagnosticMaturityAssessment {
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
    let closure = RuntimeDiagnosticLifecycleClosure::evaluate(&lineage);
    let catalog_integrity =
        RuntimeDiagnosticCatalogIntegrity::verify(&RuntimeDiagnosticContractCatalog::canonical());
    let reference_integrity =
        RuntimeDiagnosticReferenceIntegrity::verify(&RuntimeDiagnosticInteropContract::canonical());
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
    let explanation = OperatorRuntimeExplanation::explain_with_trust(
        &overview,
        &provenance,
        &continuity,
        &evolution,
        &interpretation,
        &integrity,
    );
    let explanation_consistency = RuntimeDiagnosticExplanationConsistency::verify(
        &integrity,
        &trust,
        &lineage,
        Some(&explanation),
    );
    RuntimeDiagnosticMaturityAssessment::assess(
        workspace_id,
        &catalog_integrity,
        &reference_integrity,
        &explanation_consistency,
        &closure,
    )
}

#[test]
fn case1_catalog_integrity_enforces_registration_and_dependency_order() {
    let integrity =
        RuntimeDiagnosticCatalogIntegrity::verify(&RuntimeDiagnosticContractCatalog::canonical());
    assert!(integrity.passed());
    assert!(integrity.no_duplicate_families);
    assert!(integrity.none_executable);
    assert!(integrity.dependency_order_ok);
    assert!(integrity.required_families_present);
}

#[test]
fn case2_reference_integrity_blocks_experience_drive_and_foreign_ownership() {
    let integrity =
        RuntimeDiagnosticReferenceIntegrity::verify(&RuntimeDiagnosticInteropContract::canonical());
    assert!(integrity.passed());
    assert!(integrity.experience_not_driven);
    assert!(integrity.foreign_ownership_forbidden);
}

#[test]
fn case3_explanation_consistency_requires_trust_lineage_alignment() {
    let maturity = maturity_bundle("ws-mat");
    assert!(maturity.explanation_consistency_ok);
}

#[test]
fn case4_maturity_assessment_is_meta_diagnostic_only() {
    let maturity = maturity_bundle("ws-mat");
    assert!(maturity.ready());
    assert!(maturity.meta_diagnostic_only);
    assert_eq!(maturity.level, RuntimeDiagnosticMaturityLevel::Mature);
    assert_eq!(maturity.completeness_score, 100);
    assert!(!maturity.may_prescribe());
    assert!(!maturity.may_execute());
    assert!(maturity.attempt_prescribe().is_err());
    assert!(RuntimeDiagnosticMaturityAssessment::attempt_execute().is_err());
}

#[test]
fn case5_maturity_reports_healthy_when_fully_consistent() {
    let maturity = maturity_bundle("ws-mat");
    assert!(maturity.catalog_integrity_ok);
    assert!(maturity.reference_integrity_ok);
    assert!(maturity.lifecycle_closure_ok);
    assert_eq!(
        maturity.health.as_str(),
        workspace_domain::WorkspaceHealthLevel::Healthy.as_str()
    );
}
