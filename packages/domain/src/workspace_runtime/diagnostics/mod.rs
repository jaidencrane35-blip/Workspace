//! Runtime diagnostics subsystem — observational / read-only.
//!
//! Module layout (consolidation + layering audit):
//! - [`foundation`] — dependency graph, capabilities, snapshot, consistency
//! - [`history`] — provenance, evolution, continuity, evidence, archive
//! - [`surface`] — consumption, interpretation, projection boundaries, restoration,
//!   operator overview, architecture review
//! - [`meta`] — trust, lineage, closure, catalog, interop, maturity, operator explanation
//!
//! Dependency direction (hard): foundation → history → surface → meta.
//! Lower layers must not import higher-layer types. Provenance cites overview by
//! artifact id only so history never depends on surface projections.
//!
//! Public API is re-exported flat for compatibility with `workspace_runtime::*` / `lib.rs`.
//! Diagnostics never own WorkspaceState, scoring, Experience translation, governance
//! authority, or Permission Gateway execution.

mod foundation;
mod history;
mod surface;
mod meta;

#[cfg(test)]
mod tests;

pub use foundation::*;
pub use history::*;
pub use surface::*;
pub use meta::*;

use serde::{Deserialize, Serialize};

use crate::action_proposal::GOVERNANCE_AUTHORITY_EFFECT_NONE as AUTH_NONE;
use crate::workspace_runtime::WorkspaceRuntimeError;

/// Hard boundary for the diagnostic subsystem — informational invariant only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticSubsystemBoundary {
    pub observes_only: bool,
    pub may_own_workspace_state: bool,
    pub may_change_cognition_scoring: bool,
    pub may_translate_experience: bool,
    pub may_grant_governance_authority: bool,
    pub may_enter_command_pipeline: bool,
    pub may_execute: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticSubsystemBoundary {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        Self {
            observes_only: true,
            may_own_workspace_state: false,
            may_change_cognition_scoring: false,
            may_translate_experience: false,
            may_grant_governance_authority: false,
            may_enter_command_pipeline: false,
            may_execute: false,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn respected(&self) -> bool {
        self.observes_only
            && !self.may_own_workspace_state
            && !self.may_change_cognition_scoring
            && !self.may_translate_experience
            && !self.may_grant_governance_authority
            && !self.may_enter_command_pipeline
            && !self.may_execute
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn attempt_execute(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }

    pub fn attempt_enter_command_pipeline(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::DiagnosticConsumptionForbidden)
    }
}

/// Module dependency-direction contract for the diagnostics split.
///
/// Layers (low → high): foundation, history, surface, meta.
/// Operator projections (`OperatorRuntimeOverview`, `RuntimeArchitectureReview`)
/// live in surface; history may cite them only by artifact id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDiagnosticModuleLayering {
    pub foundation_owns_operator_projections: bool,
    pub history_depends_on_surface_types: bool,
    pub foundation_depends_on_surface_or_meta: bool,
    pub surface_may_compose_foundation_and_history: bool,
    pub meta_may_compose_lower_layers: bool,
    pub overview_cited_by_artifact_id_only: bool,
    pub authority_effect: String,
}

impl RuntimeDiagnosticModuleLayering {
    pub const AUTHORITY_EFFECT_NONE: &'static str = AUTH_NONE;

    pub fn canonical() -> Self {
        Self {
            foundation_owns_operator_projections: false,
            history_depends_on_surface_types: false,
            foundation_depends_on_surface_or_meta: false,
            surface_may_compose_foundation_and_history: true,
            meta_may_compose_lower_layers: true,
            overview_cited_by_artifact_id_only: true,
            authority_effect: Self::AUTHORITY_EFFECT_NONE.into(),
        }
    }

    pub fn respected(&self) -> bool {
        !self.foundation_owns_operator_projections
            && !self.history_depends_on_surface_types
            && !self.foundation_depends_on_surface_or_meta
            && self.surface_may_compose_foundation_and_history
            && self.meta_may_compose_lower_layers
            && self.overview_cited_by_artifact_id_only
            && self.authority_effect == Self::AUTHORITY_EFFECT_NONE
    }

    pub fn attempt_execute(&self) -> Result<(), WorkspaceRuntimeError> {
        Err(WorkspaceRuntimeError::CannotExecute)
    }
}
