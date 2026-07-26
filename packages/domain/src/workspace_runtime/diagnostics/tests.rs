//! Domain unit tests for runtime diagnostics.

use super::*;
use crate::workspace_runtime::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    WorkspaceRuntimeCoherence, WorkspaceRuntimeContext, WorkspaceRuntimeHealth,
    WorkspaceRuntimeIntegrationContract,
};

fn sample_bundle() -> (
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
        "ws-diag",
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
    let graph = RuntimeDependencyGraph::canonical("ws-diag");
    let capabilities = RuntimeCapabilityMap::inventory("ws-diag");
    let snapshot = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &cognition,
        &graph,
        &capabilities,
        "t-snap",
    );
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let verification = RuntimeConsistencyVerification::verify(
        &graph,
        &capabilities,
        &ctx,
        &integration,
        &snapshot,
        "t-check",
    );
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
fn dependency_graph_has_no_cycles_and_required_edges() {
    let graph = RuntimeDependencyGraph::canonical("ws");
    assert!(!graph.has_cycles());
    assert!(!graph.required_edges().is_empty());
    assert!(RuntimeDependencyGraph::attempt_execute().is_err());
}

#[test]
fn capability_map_keeps_gateway_sole_execution() {
    let map = RuntimeCapabilityMap::inventory("ws");
    assert!(map.gateway_is_sole_execution_authority());
    assert!(!map.cognition_entries().is_empty());
}

#[test]
fn snapshot_verification_overview_review_are_read_only() {
    let (_ctx, _h, graph, _cap, snapshot, verification, overview, review) = sample_bundle();
    assert!(!snapshot.dependency_has_cycles);
    assert!(snapshot.gateway_sole_execution);
    assert!(!verification.has_errors());
    assert!(!verification.may_repair_automatically());
    assert!(verification.attempt_repair().is_err());
    assert!(overview.publication_blocked);
    assert!(overview.coherence_ok);
    assert!(review.passed());
    assert!(!graph.has_cycles());
}

#[test]
fn diagnostic_provenance_and_continuity_are_immutable() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
    assert!(provenance.cites_snapshot(&snapshot.id));
    assert!(provenance.ownership.boundaries_respected());
    assert!(!provenance.may_rewrite_history());
    assert!(provenance.attempt_rewrite_history().is_err());
    assert!(RuntimeDiagnosticOwnershipBoundary::canonical()
        .attempt_heal()
        .is_err());

    let continuity = RuntimeDiagnosticContinuityRecord::link(
        None,
        &snapshot,
        &provenance,
        "t-cont-0",
    );
    assert!(continuity.is_initial());
    assert!(!continuity.may_mutate_prior());
    assert!(continuity.attempt_mutate_prior().is_err());

    let snapshot2 = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &CognitionContextProjection::project_all(&ctx),
        &graph,
        &capabilities,
        "t-snap-2",
    );
    let continuity2 = RuntimeDiagnosticContinuityRecord::link(
        Some(&snapshot),
        &snapshot2,
        &provenance,
        "t-cont-1",
    );
    assert!(!continuity2.is_initial());
    assert!(continuity2
        .what_changed
        .iter()
        .any(|d| d == "no_observational_delta"));

    let explanation =
        OperatorRuntimeExplanation::explain(&overview, &provenance, &continuity);
    assert!(!explanation.is_execution_surface());
    assert!(explanation.publication_blocked);
    assert!(OperatorRuntimeExplanation::attempt_execute().is_err());
}

#[test]
fn diagnostic_evolution_compares_validates_and_stays_read_only() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "e0");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "e-eval-0",
    );
    assert!(evolution.passed());
    assert!(evolution.lifecycle_ordering_ok);
    assert!(evolution.may_repair() == false);
    assert!(evolution.attempt_repair().is_err());

    let snapshot2 = RuntimeDiagnosticSnapshot::capture(
        &ctx,
        &health,
        &CognitionContextProjection::project_all(&ctx),
        &graph,
        &capabilities,
        "t-snap-evo-2",
    );
    let comparison = RuntimeDiagnosticComparison::compare(&snapshot, &snapshot2);
    assert!(comparison
        .deltas
        .iter()
        .any(|d| d.kind == RuntimeDiagnosticDeltaKind::None));
    let continuity2 = RuntimeDiagnosticContinuityRecord::link(
        Some(&snapshot),
        &snapshot2,
        &provenance,
        "e1",
    );
    // Provenance cites snap1; evolution against snap2 should fail provenance consistency.
    let evolution_mismatch = RuntimeDiagnosticEvolutionReport::evaluate(
        Some(&snapshot),
        &snapshot2,
        &provenance,
        &continuity2,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "e-eval-1",
    );
    assert!(!evolution_mismatch.provenance_consistent);

    let provenance2 = RuntimeDiagnosticProvenance::from_capture(
        &snapshot2,
        &ctx,
        &health,
        &graph,
        &capabilities,
        &verification,
        &coherence,
        Some(overview.id.as_str()),
    );
    let continuity3 = RuntimeDiagnosticContinuityRecord::link(
        Some(&snapshot),
        &snapshot2,
        &provenance2,
        "e2",
    );
    let evolution_ok = RuntimeDiagnosticEvolutionReport::evaluate(
        Some(&snapshot),
        &snapshot2,
        &provenance2,
        &continuity3,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "e-eval-2",
    );
    assert!(evolution_ok.passed());
    assert!(evolution_ok.continuity_consistent);

    let registry = RuntimeArchitectureOwnershipRegistry::canonical();
    assert!(registry.operator_artifacts_are_projections_only());
    assert_eq!(
        registry.owner_of("RuntimeDependencyGraph"),
        Some(RuntimeDiagnosticOwnershipRole::ObservationalDiagnostics)
    );

    let explanation = OperatorRuntimeExplanation::explain_with_evolution(
        &overview,
        &provenance2,
        &continuity3,
        &evolution_ok,
    );
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.exposes_actions());
    assert!(RuntimeDiagnosticLifecyclePhase::Captured
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::Provenanced));
    assert!(!RuntimeDiagnosticLifecyclePhase::Captured
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::Superseded));
    assert!(RuntimeDiagnosticLifecyclePhase::ContinuityLinked
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::Compared));
    assert!(RuntimeDiagnosticLifecyclePhase::Evolved
        .may_transition_to(RuntimeDiagnosticLifecyclePhase::Archived));
}

#[test]
fn diagnostic_evidence_archive_and_integrity_are_non_authoritative() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "arch0");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "arch-eval",
    );
    assert!(evolution.passed());
    assert!(!evolution.is_authoritative());
    assert!(!evolution.is_decision());
    assert!(evolution.attempt_promote_to_decision().is_err());
    assert_eq!(evolution.lifecycle.phase, RuntimeDiagnosticLifecyclePhase::Evolved);

    let evidence = RuntimeDiagnosticEvidenceBundle::seal(
        &evolution,
        &provenance,
        &continuity,
        "seal0",
    );
    assert!(evidence.is_complete());
    assert!(!evidence.is_authoritative());
    assert!(evidence.attempt_promote_to_decision().is_err());

    let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
    assert!(archive.archive_evidence(&evidence, "a0").is_ok());
    assert!(archive.mark_superseded(&snapshot.id, "snap-next", "a1").is_ok());
    assert!(!archive.may_delete());
    assert!(archive.attempt_delete().is_err());
    assert!(archive.attempt_mutate_entry().is_err());

    let ownership = RuntimeArchitectureOwnershipRegistry::canonical().validate();
    assert!(ownership.conflict_free);
    assert_eq!(
        ownership.registry_version,
        RuntimeArchitectureOwnershipRegistry::REGISTRY_VERSION
    );

    let integrity = RuntimeDiagnosticHistoricalIntegrity::verify(
        &evidence,
        &archive,
        &evolution,
        None,
        &ownership,
    );
    assert!(integrity.passed());
    assert!(!integrity.may_repair());
    assert!(integrity.attempt_repair().is_err());
}

#[test]
fn diagnostic_consumption_interpretation_and_restoration_are_read_only() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "c-cons");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval-cons",
    );

    let contract = RuntimeDiagnosticConsumptionContract::canonical();
    assert!(contract.is_safe());
    assert!(contract.allows_consumer(RuntimeDiagnosticConsumerKind::OperatorProjection));
    assert!(contract.attempt_as_command().is_err());
    assert!(contract.attempt_as_recommendation().is_err());
    assert!(contract.attempt_enter_command_pipeline().is_err());

    let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    assert!(interpretation.all_findings_non_actionable());
    assert!(!interpretation.findings.is_empty());
    assert!(interpretation
        .findings
        .iter()
        .all(|f| !f.limitations.is_empty()));
    assert!(interpretation.attempt_as_recommendation().is_err());

    let boundaries = RuntimeProjectionBoundaryRegistry::canonical();
    assert!(boundaries.boundaries_respected());

    let evidence = RuntimeDiagnosticEvidenceBundle::seal(
        &evolution,
        &provenance,
        &continuity,
        "seal-cons",
    );
    let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
    let entry = archive.archive_evidence(&evidence, "a-cons").unwrap().clone();
    let restoration = RuntimeDiagnosticRestorationView::rehydrate(
        &archive,
        &entry,
        Some(&evidence),
        "rest0",
    );
    assert!(!restoration.may_mutate);
    assert!(!restoration.may_delete);
    assert!(restoration.attempt_mutate().is_err());
    assert!(restoration.attempt_delete().is_err());

    let explanation = OperatorRuntimeExplanation::explain_with_interpretation(
        &overview,
        &provenance,
        &continuity,
        &evolution,
        &interpretation,
    );
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.exposes_actions());
}

#[test]
fn diagnostic_trust_compatibility_and_lineage_are_informational() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "c-trust");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval-trust",
    );
    let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    let evidence = RuntimeDiagnosticEvidenceBundle::seal(
        &evolution,
        &provenance,
        &continuity,
        "seal-trust",
    );
    let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
    let entry = archive.archive_evidence(&evidence, "a-trust").unwrap().clone();
    let restoration = RuntimeDiagnosticRestorationView::rehydrate(
        &archive,
        &entry,
        Some(&evidence),
        "rest-trust",
    );

    let compatibility = RuntimeDiagnosticCompatibilityContract::canonical();
    assert!(compatibility.all_current());
    assert!(!compatibility.may_migrate);
    assert!(compatibility.attempt_migrate().is_err());
    assert_eq!(
        RUNTIME_DIAGNOSTICS_CONTRACT_VERSION,
        "runtime_diagnostics:v1"
    );

    let lineage = RuntimeDiagnosticLineageRecord::assemble(
        &snapshot,
        &provenance,
        &continuity,
        &evolution,
        Some(&entry.id),
        Some(&restoration.id),
        false,
    );
    assert_eq!(lineage.currency, RuntimeDiagnosticCurrency::RestoredView);
    assert!(lineage.ordering_ok());
    assert!(lineage.attempt_mutate().is_err());

    let lineage_validation =
        RuntimeDiagnosticLineageValidation::validate(&lineage, Some(&restoration));
    assert!(lineage_validation.passed());
    assert!(lineage_validation.attempt_repair().is_err());

    let trust = RuntimeDiagnosticTrustRecord::attest(
        &interpretation,
        &lineage,
        &compatibility,
        &RuntimeProjectionBoundaryRegistry::canonical(),
    );
    assert!(trust.trustworthy());
    assert!(!trust.is_authoritative());
    assert!(trust.attempt_promote_to_decision().is_err());
    assert!(trust
        .limitations
        .iter()
        .any(|l| l.contains("contract_version=")));
    assert!(trust.limitations.iter().any(|l| l.contains("currency=")));
}

#[test]
fn diagnostic_closure_catalog_interop_and_explanation_integrity() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "c-close");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval-close",
    );
    let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    let evidence = RuntimeDiagnosticEvidenceBundle::seal(
        &evolution,
        &provenance,
        &continuity,
        "seal-close",
    );
    let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
    let entry = archive.archive_evidence(&evidence, "a-close").unwrap().clone();
    let restoration = RuntimeDiagnosticRestorationView::rehydrate(
        &archive,
        &entry,
        Some(&evidence),
        "rest-close",
    );
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
    assert!(closure.closed());
    assert!(closure.terminal);
    assert!(closure.restored_view_is_currency_not_phase);
    assert!(RuntimeDiagnosticLifecyclePhase::Archived.is_terminal());
    assert!(!RuntimeDiagnosticLifecyclePhase::Evolved.is_terminal());
    assert!(closure.attempt_mutate().is_err());

    let catalog = RuntimeDiagnosticContractCatalog::canonical();
    assert!(catalog.none_executable());
    assert!(!catalog.discover().is_empty());
    assert!(catalog.attempt_dynamic_load().is_err());
    let report = catalog.compatibility_report(
        &RuntimeDiagnosticCompatibilityContract::canonical(),
    );
    assert!(report.all_compatible());

    let interop = RuntimeDiagnosticInteropContract::canonical();
    assert!(interop.boundaries_respected());
    assert!(interop.attempt_own_foreign().is_err());

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
    assert!(integrity.answers_complete);
    let explanation = OperatorRuntimeExplanation::explain_with_trust(
        &overview,
        &provenance,
        &continuity,
        &evolution,
        &interpretation,
        &integrity,
    );
    assert!(!explanation.is_execution_surface());
    assert!(explanation.why.iter().any(|w| w.starts_with("produced_by=")));
    assert!(explanation.why.iter().any(|w| w.starts_with("currency=")));
    assert!(explanation.why.iter().any(|w| w.starts_with("what_changed:")));
    assert!(explanation.why.iter().any(|w| w.starts_with("why_changed:")));
}

#[test]
fn diagnostic_maturity_is_meta_only_and_ready_when_consistent() {
    let (ctx, health, graph, capabilities, snapshot, verification, overview, _review) =
        sample_bundle();
    let integration = WorkspaceRuntimeIntegrationContract::audit_default("ws-diag");
    let operator = OperatorContextProjection::from_runtime_context(&ctx, &health);
    let coherence =
        WorkspaceRuntimeCoherence::review(&ctx, &integration, &health, &operator);
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
        RuntimeDiagnosticContinuityRecord::link(None, &snapshot, &provenance, "c-mat");
    let evolution = RuntimeDiagnosticEvolutionReport::evaluate(
        None,
        &snapshot,
        &provenance,
        &continuity,
        Some(RuntimeDiagnosticLifecyclePhase::Provenanced),
        "eval-mat",
    );
    let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    let evidence = RuntimeDiagnosticEvidenceBundle::seal(
        &evolution,
        &provenance,
        &continuity,
        "seal-mat",
    );
    let mut archive = RuntimeDiagnosticArchive::new("ws-diag");
    let entry = archive.archive_evidence(&evidence, "a-mat").unwrap().clone();
    let restoration = RuntimeDiagnosticRestorationView::rehydrate(
        &archive,
        &entry,
        Some(&evidence),
        "rest-mat",
    );
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
    let catalog = RuntimeDiagnosticContractCatalog::canonical();
    let catalog_integrity = RuntimeDiagnosticCatalogIntegrity::verify(&catalog);
    assert!(catalog_integrity.passed());
    let interop = RuntimeDiagnosticInteropContract::canonical();
    let reference_integrity = RuntimeDiagnosticReferenceIntegrity::verify(&interop);
    assert!(reference_integrity.passed());
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
    assert!(explanation_consistency.passed());
    let maturity = RuntimeDiagnosticMaturityAssessment::assess(
        "ws-diag",
        &catalog_integrity,
        &reference_integrity,
        &explanation_consistency,
        &closure,
    );
    assert!(maturity.ready());
    assert!(maturity.meta_diagnostic_only);
    assert_eq!(maturity.completeness_score, 100);
    assert_eq!(maturity.level, RuntimeDiagnosticMaturityLevel::Mature);
    assert!(!maturity.may_prescribe());
    assert!(maturity.attempt_prescribe().is_err());
    assert!(RuntimeDiagnosticMaturityAssessment::attempt_execute().is_err());
}

#[test]
fn diagnostic_subsystem_boundary_remains_observational_only() {
    let boundary = RuntimeDiagnosticSubsystemBoundary::canonical();
    assert!(boundary.respected());
    assert!(!boundary.may_own_workspace_state);
    assert!(!boundary.may_change_cognition_scoring);
    assert!(!boundary.may_translate_experience);
    assert!(!boundary.may_grant_governance_authority);
    assert!(!boundary.may_enter_command_pipeline);
    assert!(boundary.attempt_execute().is_err());
    assert!(boundary.attempt_enter_command_pipeline().is_err());
}

#[test]
fn diagnostic_module_layering_keeps_projections_out_of_foundation() {
    let layering = RuntimeDiagnosticModuleLayering::canonical();
    assert!(layering.respected());
    assert!(!layering.foundation_owns_operator_projections);
    assert!(layering.overview_cited_by_artifact_id_only);
    assert!(layering.attempt_execute().is_err());
}
