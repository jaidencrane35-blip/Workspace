//! Product Proof wire-contract export root (Constitution §4.4 / §4.7).
//!
//! This type exists solely to drive TypeScript generation. It is not an IPC
//! payload and must not be constructed in product runtime paths.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::desktop_action::{ActionOperationResult, ResumePlanPreview};
use crate::pilot_measurement::{PilotMeasurementScope, PilotMeasurementSnapshot};
use crate::saved_context::{SavedContext, SavedContextCaptureScope};
use crate::workspace::Workspace;

/// Aggregate of Experience / Product Proof IPC payload types.
///
/// Not itself an Experience IPC payload — used only to force a closed TS graph
/// for `export_product_contracts`.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ProductProofContractRoot {
    pub workspace: Workspace,
    pub saved_context: SavedContext,
    pub saved_context_capture_scope: SavedContextCaptureScope,
    pub resume_plan_preview: ResumePlanPreview,
    pub action_operation_result: ActionOperationResult,
    pub pilot_measurement_scope: PilotMeasurementScope,
    pub pilot_measurement_snapshot: PilotMeasurementSnapshot,
}
