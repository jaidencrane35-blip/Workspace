//! Production Restore Executor (Action plan → live desktop effects).
//!
//! Canonical lineage:
//! ```text
//! RestoreCompatibilitySummary → ActionPlan (RestorePlan)
//!   → RestoreExecutor → ActionOperationResult (+ RestoreExecutionSummary)
//! ```
//!
//! Reuses [`DesktopActionService`] and [`WindowMutator`]; does not duplicate
//! matching or Win32 effects. Never relaunches applications.

use workspace_domain::{
    ActionOperationResult, ActionPlan, CapabilitySet, ItemEffectProof,
};
use workspace_windows_integration::WindowMutator;

use crate::error::Result;
use crate::services::{ActionExecutionControls, DesktopActionService};

/// Executes an approved restore plan against the live Windows session.
pub(crate) struct RestoreExecutor;

impl RestoreExecutor {
    /// Runs place/focus effects for `will_attempt` items; skips the rest safely.
    ///
    /// Partial success is always retained in [`ActionOperationResult::items`] and
    /// aggregated in [`ActionOperationResult::summary`].
    pub(crate) fn execute(
        plan: &ActionPlan,
        proofs: &[ItemEffectProof],
        capability_set: &CapabilitySet,
        mutator: &dyn WindowMutator,
        controls: &ActionExecutionControls,
    ) -> Result<ActionOperationResult> {
        DesktopActionService::execute(plan, proofs, capability_set, mutator, controls)
    }
}
