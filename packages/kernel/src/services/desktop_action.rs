//! Action capability: plan resolution and declared desktop effects (PP-M1-02).
//!
//! Action never reads a saved-context identifier. Matching examines only the
//! declared hwnd. No candidate list, unmatched detail, or environment model
//! leaves this module.

use std::sync::{Arc, Mutex};
use std::time::Instant;

use chrono::Utc;
use workspace_domain::{
    compute_plan_digest, is_declared_action_type, is_reserved_action_type, match_exact_session,
    new_operation_id, new_plan_expiry, new_plan_id, operation_outcome_from_items,
    permission_scope_for, ActionItemOutcome, ActionOperationResult, ActionPlan, ActionPlanItem,
    ActionRequest, ActionTargetDescriptor, Capability, CapabilitySet, DesktopActionError,
    ItemDisposition, ItemEffectProof, LiveWindowIdentity, MatchResult, OperationOutcome,
    ProjectedDisposition, ProposedEffect, RestoreExecutionSummary, SCOPE_PLAN_RESOLVE,
    SCOPE_WINDOW_FOCUS, SCOPE_WINDOW_PLACE, ACTION_TYPE_WINDOW_FOCUS, ACTION_TYPE_WINDOW_PLACE,
};
use workspace_windows_integration::{
    MutatorEffectOutcome, WindowMutator, WindowPlacementRequest,
};

use crate::error::{KernelError, Result};

/// Optional execution controls for cancellation / mid-operation grant changes.
#[derive(Default)]
pub struct ActionExecutionControls {
    pub cancel_requested: Option<Arc<Mutex<bool>>>,
    pub grants: Option<Arc<Mutex<CapabilitySet>>>,
}

pub(crate) struct DesktopActionService;

impl DesktopActionService {
    /// ACT-REQ-004 — non-effecting plan resolution.
    pub(crate) fn resolve_plan(
        request: &ActionRequest,
        capability_set: &CapabilitySet,
        mutator: &dyn WindowMutator,
    ) -> Result<ActionPlan> {
        Self::require_scope(capability_set, SCOPE_PLAN_RESOLVE)?;
        Self::validate_request_shape(request)?;

        let now = Utc::now();
        let plan_id = new_plan_id();
        let expires_at = new_plan_expiry(now);
        let current_session = mutator
            .current_desktop_session_id()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;
        let monitors = mutator
            .attached_monitor_indices()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;

        let mut items = Vec::with_capacity(request.items.len());
        for target in &request.items {
            items.push(Self::resolve_item(target, &current_session, &monitors, mutator)?);
        }

        let digest = compute_plan_digest(&plan_id, &expires_at, &request.purpose, &items);
        Ok(ActionPlan {
            plan_id,
            expires_at,
            plan_digest: digest,
            purpose: request.purpose.clone(),
            items,
        })
    }

    /// ACT-CMD-001 — execute an approved plan with per-item effect proofs.
    ///
    /// Prefer [`crate::services::RestoreExecutor::execute`] for the production
    /// restore entry point (same behaviour; documents the RestorePlan → Result lineage).
    pub(crate) fn execute(
        plan: &ActionPlan,
        proofs: &[ItemEffectProof],
        capability_set: &CapabilitySet,
        mutator: &dyn WindowMutator,
        controls: &ActionExecutionControls,
    ) -> Result<ActionOperationResult> {
        let started = Instant::now();
        Self::validate_plan_for_execution(plan)?;

        let expected = compute_plan_digest(
            &plan.plan_id,
            &plan.expires_at,
            &plan.purpose,
            &plan.items,
        );
        if expected != plan.plan_digest || plan.is_expired_at(Utc::now()) {
            return Err(KernelError::DesktopAction(DesktopActionError::PlanUnknown));
        }

        let current_session = mutator
            .current_desktop_session_id()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;
        let monitors = mutator
            .attached_monitor_indices()
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;

        let mut outcomes = Vec::with_capacity(plan.items.len());
        let mut cancelled = false;

        for item in &plan.items {
            if cancelled {
                outcomes.push(Self::outcome(
                    item,
                    ItemDisposition::NotAttempted,
                    Some("Operation cancelled before this item began.".into()),
                    None,
                ));
                continue;
            }

            if Self::cancel_requested(controls) {
                cancelled = true;
                outcomes.push(Self::outcome(
                    item,
                    ItemDisposition::NotAttempted,
                    Some("Operation cancelled before this item began.".into()),
                    None,
                ));
                continue;
            }

            match item.projected_disposition {
                ProjectedDisposition::WillSkipUnsupported => {
                    outcomes.push(Self::outcome(
                        item,
                        ItemDisposition::SkippedUnsupported,
                        item.reason.clone(),
                        item.error_code.clone(),
                    ));
                    continue;
                }
                ProjectedDisposition::WillSkipUnresolvable => {
                    outcomes.push(Self::outcome(
                        item,
                        ItemDisposition::SkippedUnresolvable,
                        item.reason.clone(),
                        item.error_code.clone(),
                    ));
                    continue;
                }
                ProjectedDisposition::WillAttempt => {}
            }

            let grants = Self::effective_grants(capability_set, controls);
            let scope = item.permission_scope.as_str();
            if !Self::grants_contain(&grants, scope) {
                outcomes.push(Self::outcome(
                    item,
                    ItemDisposition::Failed,
                    Some(format!(
                        "Effect authority for '{scope}' was not present at point of use."
                    )),
                    Some("ACTION_PERMISSION_DENIED".into()),
                ));
                // Remaining will_attempt items are not begun when authority is gone.
                cancelled = true;
                continue;
            }

            let proof = proofs.iter().find(|proof| {
                proof.item_id == item.item_id
                    && proof.plan_digest == plan.plan_digest
                    && proof.action_type == item.action_type
                    && proof.permission_scope == item.permission_scope
            });
            let Some(proof) = proof else {
                outcomes.push(Self::outcome(
                    item,
                    ItemDisposition::Failed,
                    Some("No valid item effect proof was presented for this approved item.".into()),
                    Some("ACTION_PERMISSION_DENIED".into()),
                ));
                continue;
            };
            if proof.purpose != plan.purpose {
                outcomes.push(Self::outcome(
                    item,
                    ItemDisposition::Failed,
                    Some("The item effect proof purpose does not match the approved plan.".into()),
                    Some("ACTION_PERMISSION_DENIED".into()),
                ));
                continue;
            }

            let match_now =
                Self::match_target(&item.target, &current_session, &monitors, mutator)?;
            if match_now != MatchResult::ExactSessionMatch {
                outcomes.push(Self::outcome(
                    item,
                    ItemDisposition::RefusedChanged,
                    Some(format!(
                        "Resolution changed since approval. {}",
                        match_now.safe_reason()
                    )),
                    Some("ACTION_PLAN_ITEM_CHANGED".into()),
                ));
                continue;
            }

            let hwnd = item
                .target
                .restore_identity
                .as_ref()
                .map(|identity| identity.captured_hwnd.clone())
                .unwrap_or_default();

            let effect = Self::apply_effect(&item.proposed_effect, &hwnd, mutator)?;
            let mut outcome = match effect {
                MutatorEffectOutcome::Committed => {
                    Self::outcome(item, ItemDisposition::Completed, None, None)
                }
                MutatorEffectOutcome::RefusedByEnvironment => Self::outcome(
                    item,
                    ItemDisposition::Failed,
                    Some(
                        "The window manager refused the effect. Nothing further was attempted for it."
                            .into(),
                    ),
                    Some("ACTION_TARGET_REFUSED_BY_ENVIRONMENT".into()),
                ),
                MutatorEffectOutcome::OutcomeUnknown => Self::outcome(
                    item,
                    ItemDisposition::OutcomeUnknown,
                    Some(
                        "The effect was attempted but its outcome could not be established."
                            .into(),
                    ),
                    Some("ACTION_EFFECT_OUTCOME_UNKNOWN".into()),
                ),
            };
            outcome.why = plan.purpose.clone();
            outcomes.push(outcome);
        }

        for outcome in &mut outcomes {
            if outcome.why.is_empty() {
                outcome.why = plan.purpose.clone();
            }
        }

        let mut outcome = operation_outcome_from_items(&outcomes);
        if cancelled && matches!(outcome, OperationOutcome::Failed | OperationOutcome::PartiallyCompleted | OperationOutcome::Completed) {
            // Cancellation dominates only when explicitly requested mid-flight and
            // some items were left not_attempted for that reason.
            if Self::cancel_requested(controls)
                && outcomes
                    .iter()
                    .any(|item| item.disposition == ItemDisposition::NotAttempted)
            {
                outcome = OperationOutcome::Cancelled;
            }
        }

        let duration_ms = started.elapsed().as_millis() as u64;
        Ok(ActionOperationResult {
            operation_id: new_operation_id(),
            outcome,
            summary: RestoreExecutionSummary::from_items(&outcomes, duration_ms),
            items: outcomes,
        })
    }

    fn resolve_item(
        target: &ActionTargetDescriptor,
        current_session: &str,
        monitors: &[i32],
        mutator: &dyn WindowMutator,
    ) -> Result<ActionPlanItem> {
        if target.saved_context_id.is_some() {
            return Err(KernelError::DesktopAction(DesktopActionError::ContractInvalid(
                "Action must not receive a saved-context identifier".into(),
            )));
        }
        if target.confidence_threshold.is_some() {
            return Err(KernelError::DesktopAction(DesktopActionError::ContractInvalid(
                "callers must not supply a confidence threshold".into(),
            )));
        }

        if is_reserved_action_type(&target.action_type)
            || (!is_declared_action_type(&target.action_type)
                && matches!(
                    target.proposed_effect,
                    ProposedEffect::Unsupported { .. }
                ))
        {
            return Ok(ActionPlanItem {
                item_id: target.item_id.clone(),
                action_type: target.action_type.clone(),
                target_summary: target.target_summary.clone(),
                proposed_effect: target.proposed_effect.clone(),
                permission_scope: String::new(),
                projected_disposition: ProjectedDisposition::WillSkipUnsupported,
                reason: Some(format!(
                    "'{}' is not a declared restore action for Product Proof.",
                    target.action_type
                )),
                error_code: Some("ACTION_TYPE_NOT_DECLARED".into()),
                target: target.clone(),
            });
        }

        if !is_declared_action_type(&target.action_type) {
            return Err(KernelError::DesktopAction(DesktopActionError::TypeNotDeclared(
                target.action_type.clone(),
            )));
        }

        let scope = permission_scope_for(&target.action_type)
            .unwrap_or("")
            .to_string();

        let match_result = Self::match_target(target, current_session, monitors, mutator)?;
        let (disposition, reason, error_code) = match match_result {
            MatchResult::ExactSessionMatch => (
                ProjectedDisposition::WillAttempt,
                None,
                None,
            ),
            other => (
                ProjectedDisposition::WillSkipUnresolvable,
                Some(other.safe_reason()),
                other.error_code().map(str::to_string),
            ),
        };

        Ok(ActionPlanItem {
            item_id: target.item_id.clone(),
            action_type: target.action_type.clone(),
            target_summary: target.target_summary.clone(),
            proposed_effect: target.proposed_effect.clone(),
            permission_scope: scope,
            projected_disposition: disposition,
            reason,
            error_code,
            target: target.clone(),
        })
    }

    fn match_target(
        target: &ActionTargetDescriptor,
        current_session: &str,
        monitors: &[i32],
        mutator: &dyn WindowMutator,
    ) -> Result<MatchResult> {
        if let Some(reason) = &target.identity_unavailable_reason {
            return Ok(MatchResult::IdentityUnavailable {
                reason: reason.clone(),
            });
        }
        let Some(identity) = &target.restore_identity else {
            return Ok(MatchResult::IdentityUnavailable {
                reason: "restore identity missing".into(),
            });
        };

        // Bounded lookup: only the declared hwnd. No enumeration of others.
        let live = mutator
            .window_by_hwnd(&identity.captured_hwnd)
            .map_err(|error| KernelError::WindowsIntegration {
                message: error.to_string(),
            })?;
        let candidates: Vec<LiveWindowIdentity> = match live {
            Some(view) => vec![LiveWindowIdentity {
                hwnd: view.hwnd,
                process_id: view.process_id as i32,
                title: view.title,
            }],
            None => Vec::new(),
        };

        Ok(match_exact_session(
            identity,
            current_session,
            &candidates,
            monitors,
            &target.proposed_effect,
        ))
    }

    fn apply_effect(
        effect: &ProposedEffect,
        hwnd: &str,
        mutator: &dyn WindowMutator,
    ) -> Result<MutatorEffectOutcome> {
        match effect {
            ProposedEffect::Place {
                x,
                y,
                width,
                height,
                minimized,
                ..
            } => mutator
                .place_window(
                    hwnd,
                    &WindowPlacementRequest {
                        x: *x,
                        y: *y,
                        width: *width,
                        height: *height,
                        minimized: *minimized,
                    },
                )
                .map_err(|error| KernelError::WindowsIntegration {
                    message: error.to_string(),
                }),
            ProposedEffect::Focus => mutator.focus_window(hwnd).map_err(|error| {
                KernelError::WindowsIntegration {
                    message: error.to_string(),
                }
            }),
            ProposedEffect::Unsupported { .. } => Ok(MutatorEffectOutcome::RefusedByEnvironment),
        }
    }

    fn validate_request_shape(request: &ActionRequest) -> Result<()> {
        if request.items.is_empty() {
            return Err(KernelError::DesktopAction(DesktopActionError::RequestEmpty));
        }
        for item in &request.items {
            if item.saved_context_id.is_some() {
                return Err(KernelError::DesktopAction(DesktopActionError::ContractInvalid(
                    "Action must not receive a saved-context identifier".into(),
                )));
            }
            if item.confidence_threshold.is_some() {
                return Err(KernelError::DesktopAction(DesktopActionError::ContractInvalid(
                    "callers must not supply a confidence threshold".into(),
                )));
            }
            if !is_declared_action_type(&item.action_type)
                && !is_reserved_action_type(&item.action_type)
                && !matches!(item.proposed_effect, ProposedEffect::Unsupported { .. })
            {
                return Err(KernelError::DesktopAction(DesktopActionError::TypeNotDeclared(
                    item.action_type.clone(),
                )));
            }
        }
        Ok(())
    }

    fn validate_plan_for_execution(plan: &ActionPlan) -> Result<()> {
        if plan.items.is_empty() {
            return Err(KernelError::DesktopAction(DesktopActionError::RequestEmpty));
        }
        for item in &plan.items {
            if item.projected_disposition == ProjectedDisposition::WillAttempt
                && !is_declared_action_type(&item.action_type)
            {
                return Err(KernelError::DesktopAction(DesktopActionError::TypeNotDeclared(
                    item.action_type.clone(),
                )));
            }
        }
        Ok(())
    }

    fn require_scope(capability_set: &CapabilitySet, scope: &str) -> Result<()> {
        let capability = Capability::new(scope, workspace_domain::CapabilityScope::System)
            .map_err(KernelError::Domain)?;
        if !capability_set.contains(&capability) {
            return Err(KernelError::DesktopAction(DesktopActionError::PermissionDenied(
                format!("missing {scope}"),
            )));
        }
        Ok(())
    }

    fn grants_contain(set: &CapabilitySet, scope: &str) -> bool {
        Capability::new(scope, workspace_domain::CapabilityScope::System)
            .map(|capability| set.contains(&capability))
            .unwrap_or(false)
    }

    fn effective_grants(
        capability_set: &CapabilitySet,
        controls: &ActionExecutionControls,
    ) -> CapabilitySet {
        controls
            .grants
            .as_ref()
            .and_then(|grants| grants.lock().ok().map(|guard| guard.clone()))
            .unwrap_or_else(|| capability_set.clone())
    }

    fn cancel_requested(controls: &ActionExecutionControls) -> bool {
        controls
            .cancel_requested
            .as_ref()
            .and_then(|flag| flag.lock().ok().map(|guard| *guard))
            .unwrap_or(false)
    }

    fn outcome(
        item: &ActionPlanItem,
        disposition: ItemDisposition,
        reason: Option<String>,
        error_code: Option<String>,
    ) -> ActionItemOutcome {
        let what = match &item.proposed_effect {
            ProposedEffect::Place {
                x,
                y,
                width,
                height,
                minimized,
                ..
            } => {
                if *minimized {
                    format!("Minimise “{}”", item.target_summary)
                } else {
                    format!(
                        "Place “{}” at ({x}, {y}) size {width}×{height}",
                        item.target_summary
                    )
                }
            }
            ProposedEffect::Focus => format!("Focus “{}”", item.target_summary),
            ProposedEffect::Unsupported { intent } => {
                format!("Unsupported intent “{intent}” for “{}”", item.target_summary)
            }
        };
        let user_action = if disposition == ItemDisposition::Completed {
            "none"
        } else {
            "inspect"
        };
        ActionItemOutcome {
            item_id: item.item_id.clone(),
            action_type: item.action_type.clone(),
            target_summary: item.target_summary.clone(),
            disposition,
            what,
            why: String::new(), // filled by caller/companion with purpose
            reason,
            error_code,
            user_action_available: user_action.into(),
        }
    }

    pub(crate) fn proofs_for_plan(
        plan: &ActionPlan,
        requester: &str,
    ) -> Vec<ItemEffectProof> {
        plan.items
            .iter()
            .filter(|item| item.projected_disposition == ProjectedDisposition::WillAttempt)
            .map(|item| ItemEffectProof {
                plan_digest: plan.plan_digest.clone(),
                item_id: item.item_id.clone(),
                action_type: item.action_type.clone(),
                permission_scope: item.permission_scope.clone(),
                purpose: plan.purpose.clone(),
                requester: requester.into(),
            })
            .collect()
    }
}

/// Builds Action request items from a saved context without exposing the
/// saved-context identifier to Action (Companion responsibility).
pub fn action_request_from_saved_context(
    context: &workspace_domain::SavedContext,
) -> ActionRequest {
    let mut items = Vec::new();
    for window in &context.windows {
        let place_id = format!("{}:place", window.id);
        items.push(ActionTargetDescriptor {
            item_id: place_id,
            action_type: ACTION_TYPE_WINDOW_PLACE.into(),
            target_summary: window.title.clone(),
            restore_identity: window.restore_identity.clone(),
            identity_unavailable_reason: window.restore_identity_unavailable_reason.clone(),
            proposed_effect: ProposedEffect::place(
                window.x,
                window.y,
                window.width,
                window.height,
                window.monitor_index,
                window.minimized,
            ),
            confidence_threshold: None,
            saved_context_id: None,
        });

        if window.focused {
            items.push(ActionTargetDescriptor {
                item_id: format!("{}:focus", window.id),
                action_type: ACTION_TYPE_WINDOW_FOCUS.into(),
                target_summary: window.title.clone(),
                restore_identity: window.restore_identity.clone(),
                identity_unavailable_reason: window.restore_identity_unavailable_reason.clone(),
                proposed_effect: ProposedEffect::Focus,
                confidence_threshold: None,
                saved_context_id: None,
            });
        }

        if window.z_order.is_some() {
            items.push(ActionTargetDescriptor {
                item_id: format!("{}:z_order", window.id),
                action_type: "window.z_order".into(),
                target_summary: window.title.clone(),
                restore_identity: None,
                identity_unavailable_reason: Some(
                    "z-order restoration beyond focus is unsupported in Product Proof".into(),
                ),
                proposed_effect: ProposedEffect::Unsupported {
                    intent: "z-order restoration".into(),
                },
                confidence_threshold: None,
                saved_context_id: None,
            });
        }
    }

    ActionRequest {
        purpose: format!("Resume saved context “{}”", context.name),
        items,
    }
}

#[allow(dead_code)]
fn _scope_constants_used() {
    let _ = (SCOPE_WINDOW_PLACE, SCOPE_WINDOW_FOCUS);
}
