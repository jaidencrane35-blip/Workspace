//! Consolidation pass — diagnostic subsystem module boundary & API stability.

use workspace_domain::{
    OperatorRuntimeExplanation, RuntimeDiagnosticCompatibilityContract,
    RuntimeDiagnosticConsumptionContract, RuntimeDiagnosticContractCatalog,
    RuntimeDiagnosticInteropContract, RuntimeDiagnosticMaturityAssessment,
    RuntimeDiagnosticSubsystemBoundary, RuntimeProjectionBoundaryRegistry,
    RUNTIME_DIAGNOSTICS_CONTRACT_VERSION,
};

#[test]
fn case1_subsystem_boundary_forbids_authority_surfaces() {
    let boundary = RuntimeDiagnosticSubsystemBoundary::canonical();
    assert!(boundary.respected());
    assert!(boundary.observes_only);
    assert!(!boundary.may_own_workspace_state);
    assert!(!boundary.may_change_cognition_scoring);
    assert!(!boundary.may_translate_experience);
    assert!(!boundary.may_grant_governance_authority);
    assert!(!boundary.may_enter_command_pipeline);
    assert!(!boundary.may_execute);
    assert!(boundary.attempt_execute().is_err());
    assert!(boundary.attempt_enter_command_pipeline().is_err());
}

#[test]
fn case2_flat_exports_remain_discoverable_after_module_split() {
    // Smoke: public types still resolve through workspace_domain after consolidation.
    assert_eq!(RUNTIME_DIAGNOSTICS_CONTRACT_VERSION, "runtime_diagnostics:v1");
    assert!(RuntimeDiagnosticConsumptionContract::canonical().is_safe());
    assert!(RuntimeDiagnosticCompatibilityContract::canonical().all_current());
    assert!(RuntimeDiagnosticContractCatalog::canonical().none_executable());
    assert!(RuntimeDiagnosticInteropContract::canonical().boundaries_respected());
    assert!(RuntimeProjectionBoundaryRegistry::canonical().boundaries_respected());
    let _ = std::any::type_name::<OperatorRuntimeExplanation>();
    let _ = std::any::type_name::<RuntimeDiagnosticMaturityAssessment>();
}

#[test]
fn case3_catalog_and_boundary_agree_diagnostics_are_non_executable() {
    let catalog = RuntimeDiagnosticContractCatalog::canonical();
    let boundary = RuntimeDiagnosticSubsystemBoundary::canonical();
    assert!(catalog.none_executable());
    assert!(catalog.attempt_dynamic_load().is_err());
    assert!(boundary.respected());
    assert!(!boundary.may_execute);
}
