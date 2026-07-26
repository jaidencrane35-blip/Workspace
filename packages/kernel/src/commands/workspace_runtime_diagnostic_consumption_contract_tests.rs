//! Post–integrity audit — runtime diagnostic consumption & interpretation tests.

use workspace_domain::{
    CognitionContextProjection, GovernanceRuntimeSummary, OperatorContextProjection,
    OperatorRuntimeExplanation, OperatorRuntimeOverview, RuntimeCapabilityMap,
    RuntimeConsistencyVerification, RuntimeDependencyGraph, RuntimeDiagnosticArchive,
    RuntimeDiagnosticConsumerKind, RuntimeDiagnosticConsumptionContract,
    RuntimeDiagnosticContinuityRecord, RuntimeDiagnosticEvidenceBundle,
    RuntimeDiagnosticEvolutionReport, RuntimeDiagnosticForbiddenInterpretation,
    RuntimeDiagnosticInterpretationView, RuntimeDiagnosticLifecyclePhase,
    RuntimeDiagnosticProvenance, RuntimeDiagnosticRestorationView, RuntimeDiagnosticSnapshot,
    RuntimeProjectionBoundaryLayer, RuntimeProjectionBoundaryRegistry, WorkspaceRuntimeCoherence,
    WorkspaceRuntimeContext, WorkspaceRuntimeHealth, WorkspaceRuntimeIntegrationContract,
};

fn evolved(workspace_id: &str) -> (
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
fn case1_consumption_forbids_commands_and_recommendations() {
    let contract = RuntimeDiagnosticConsumptionContract::canonical();
    assert!(contract.is_safe());
    assert!(contract.allows_consumer(RuntimeDiagnosticConsumerKind::OperatorProjection));
    assert!(contract.forbids(RuntimeDiagnosticForbiddenInterpretation::Command));
    assert!(contract.forbids(RuntimeDiagnosticForbiddenInterpretation::Recommendation));
    assert!(contract.forbids(RuntimeDiagnosticForbiddenInterpretation::ExperienceTranslation));
    assert!(!contract.may_enter_command_pipeline);
    assert!(contract.attempt_as_command().is_err());
    assert!(contract.attempt_as_recommendation().is_err());
    assert!(contract.attempt_enter_command_pipeline().is_err());
}

#[test]
fn case2_interpretation_findings_have_structure_and_are_non_actionable() {
    let (evolution, _p, _c, _o) = evolved("ws-cons");
    let view = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    assert!(!view.findings.is_empty());
    assert!(view.all_findings_non_actionable());
    for f in &view.findings {
        assert!(!f.limitations.is_empty());
        assert!(!f.source.is_empty());
        assert!(!f.is_command);
        assert!(!f.is_recommendation);
    }
    assert!(view.attempt_as_recommendation().is_err());
    assert!(RuntimeDiagnosticInterpretationView::attempt_execute().is_err());
}

#[test]
fn case3_projection_boundaries_keep_experience_and_governance_separate() {
    let registry = RuntimeProjectionBoundaryRegistry::canonical();
    assert!(registry.boundaries_respected());
    let experience = registry
        .entries
        .iter()
        .find(|e| e.layer == RuntimeProjectionBoundaryLayer::Experience)
        .unwrap();
    assert!(!experience.may_consume_diagnostics);
    assert!(!experience.may_emit_commands);
    let audit = registry
        .entries
        .iter()
        .find(|e| e.layer == RuntimeProjectionBoundaryLayer::AuditHistory)
        .unwrap();
    assert!(!audit.may_consume_diagnostics);
}

#[test]
fn case4_restoration_is_read_only_rehydration() {
    let (evolution, provenance, continuity, _o) = evolved("ws-cons");
    let evidence =
        RuntimeDiagnosticEvidenceBundle::seal(&evolution, &provenance, &continuity, "seal0");
    let mut archive = RuntimeDiagnosticArchive::new("ws-cons");
    let entry = archive.archive_evidence(&evidence, "a0").unwrap().clone();
    let restoration =
        RuntimeDiagnosticRestorationView::rehydrate(&archive, &entry, Some(&evidence), "r0");
    assert!(!restoration.may_mutate);
    assert!(!restoration.may_delete);
    assert!(!restoration.restored_refs.is_empty());
    assert!(restoration.attempt_mutate().is_err());
    assert!(restoration.attempt_delete().is_err());
    assert!(RuntimeDiagnosticRestorationView::attempt_execute().is_err());
}

#[test]
fn case5_operator_explanation_with_interpretation_stays_non_actionable() {
    let (evolution, provenance, continuity, overview) = evolved("ws-cons");
    let interpretation = RuntimeDiagnosticInterpretationView::from_evolution(&evolution);
    let explanation = OperatorRuntimeExplanation::explain_with_interpretation(
        &overview,
        &provenance,
        &continuity,
        &evolution,
        &interpretation,
    );
    assert!(!explanation.is_execution_surface());
    assert!(!explanation.exposes_actions());
    assert!(explanation
        .why
        .iter()
        .any(|w| w.contains("consumption=observe_explain_only")));
    assert!(explanation.why.iter().any(|w| w.contains("limitation:")));
}
