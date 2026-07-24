//! Shared workspace attribution helpers (Phase 4 Batch 9.5).
//!
//! Single implementation for plan/approval → workspace scoping used by
//! Intelligence, Decision Queue, and Activity Graph. Prevents attribution drift.

use std::collections::HashSet;

use workspace_domain::{AiOrchestratedPlan, PermissionApprovalRequest};

/// True when the plan is linked to the workspace or targets a workspace application.
pub(crate) fn plan_belongs_to_workspace(
    plan: &AiOrchestratedPlan,
    app_ids: &HashSet<String>,
    linked_plan_ids: &HashSet<String>,
) -> bool {
    if linked_plan_ids.contains(plan.id.as_str()) {
        return true;
    }
    plan.steps.iter().any(|step| {
        step.proposal
            .target_resource
            .as_ref()
            .is_some_and(|resource| app_ids.contains(resource.id.as_str()))
    })
}

/// True when the approval is linked via a plan step or mentions workspace/app ids.
pub(crate) fn approval_belongs_to_workspace(
    item: &PermissionApprovalRequest,
    workspace_id: &str,
    app_ids: &HashSet<String>,
    linked_approval_ids: &HashSet<String>,
) -> bool {
    if linked_approval_ids.contains(item.id.as_str()) {
        return true;
    }
    if item.subject.contains(workspace_id) || item.reason.contains(workspace_id) {
        return true;
    }
    app_ids.iter().any(|app_id| {
        item.subject.contains(app_id.as_str()) || item.reason.contains(app_id.as_str())
    })
}
