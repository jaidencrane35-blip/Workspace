//! Post-consolidation architecture audit — diagnostic module dependency direction.

use std::path::PathBuf;

use workspace_domain::{
    OperatorRuntimeOverview, RuntimeArchitectureReview, RuntimeDiagnosticModuleLayering,
    RuntimeDiagnosticSubsystemBoundary,
};

fn diagnostics_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../domain/src/workspace_runtime/diagnostics")
}

#[test]
fn case1_module_layering_contract_respected() {
    let layering = RuntimeDiagnosticModuleLayering::canonical();
    assert!(layering.respected());
    assert!(!layering.foundation_owns_operator_projections);
    assert!(!layering.history_depends_on_surface_types);
    assert!(!layering.foundation_depends_on_surface_or_meta);
    assert!(layering.overview_cited_by_artifact_id_only);
    assert!(layering.attempt_execute().is_err());
    assert!(RuntimeDiagnosticSubsystemBoundary::canonical().respected());
}

#[test]
fn case2_operator_projections_live_in_surface_not_foundation() {
    let dir = diagnostics_dir();
    let foundation = std::fs::read_to_string(dir.join("foundation.rs")).expect("foundation.rs");
    let surface = std::fs::read_to_string(dir.join("surface.rs")).expect("surface.rs");
    assert!(
        !foundation.contains("struct OperatorRuntimeOverview"),
        "foundation must not own OperatorRuntimeOverview"
    );
    assert!(
        !foundation.contains("struct RuntimeArchitectureReview"),
        "foundation must not own RuntimeArchitectureReview"
    );
    assert!(
        surface.contains("struct OperatorRuntimeOverview"),
        "OperatorRuntimeOverview must live in surface"
    );
    assert!(
        surface.contains("struct RuntimeArchitectureReview"),
        "RuntimeArchitectureReview must live in surface"
    );
    let _ = std::any::type_name::<OperatorRuntimeOverview>();
    let _ = std::any::type_name::<RuntimeArchitectureReview>();
}

#[test]
fn case3_lower_layers_do_not_import_higher_layer_types() {
    let dir = diagnostics_dir();
    let foundation = std::fs::read_to_string(dir.join("foundation.rs")).expect("foundation.rs");
    let history = std::fs::read_to_string(dir.join("history.rs")).expect("history.rs");

    for forbidden in [
        "OperatorRuntimeOverview",
        "RuntimeArchitectureReview",
        "RuntimeDiagnosticConsumptionContract",
        "OperatorRuntimeExplanation",
        "RuntimeDiagnosticMaturityAssessment",
    ] {
        assert!(
            !foundation.contains(forbidden),
            "foundation must not reference higher-layer type {forbidden}"
        );
    }

    // History may mention overview by name in ownership registry strings, but must
    // not import or take the surface type in from_capture.
    assert!(
        !history.contains("use super::{\n    OperatorRuntimeOverview"),
        "history must not import OperatorRuntimeOverview"
    );
    assert!(
        history.contains("overview_id: Option<&str>"),
        "history must cite overview by artifact id only"
    );
    assert!(
        !history.contains("overview: Option<&OperatorRuntimeOverview>"),
        "history must not take OperatorRuntimeOverview by type"
    );
}
