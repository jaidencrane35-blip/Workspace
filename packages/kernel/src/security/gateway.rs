use std::sync::{Arc, Mutex};

use serde_json::json;
use workspace_database::Database;
use workspace_domain::{ActorContext, CapabilitySet, IntentContext};

use crate::error::{KernelError, Result};
use crate::policy::{
    DefaultPolicyEvaluator, PermissionPolicy, PolicyContext, PolicyDecision, PolicyEvaluator,
};
use crate::services::{AuditService, PermissionApprovalService};

use super::gate::{PermissionDecision, PermissionGate, PermissionRequest};

/// Outcome from the Permission Gateway authority boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayDecision {
    Allow { reason: String },
    Deny { reason: String },
    ApprovalRequired {
        reason: String,
        approval_request_id: String,
    },
}

/// Coordinates policy + gate evaluation, approval persistence, and decision auditing.
///
/// Pipeline path:
/// `CommandPipeline → PermissionGateway → Allow | Deny | ApprovalRequired → Execution`
pub struct PermissionGateway;

impl PermissionGateway {
    /// Evaluates authorization without persisting an audit record (e.g. discovery probes).
    pub fn evaluate(
        policy: &dyn PermissionPolicy,
        gate: &dyn PermissionGate,
        request: &PermissionRequest,
        granted: &CapabilitySet,
    ) -> Result<GatewayDecision> {
        Self::evaluate_with_grants(policy, gate, request, granted, &[])
    }

    fn evaluate_with_grants(
        policy: &dyn PermissionPolicy,
        gate: &dyn PermissionGate,
        request: &PermissionRequest,
        granted: &CapabilitySet,
        active_grants: &[workspace_domain::CapabilityGrant],
    ) -> Result<GatewayDecision> {
        let policy_ctx = PolicyContext::new(
            request.actor.id.to_string(),
            request.intent.clone(),
            request.capability.clone(),
            request.command,
            request.subject.clone(),
        )
        .with_granted(granted.clone());

        let policy_result = DefaultPolicyEvaluator.evaluate(policy, &policy_ctx)?;

        if !policy_result.is_allowed() {
            let reason = match policy_result.decision {
                PolicyDecision::Deny { reason } => reason,
                PolicyDecision::Allow => "policy denied without reason".to_string(),
            };

            if PermissionApprovalService::is_non_human(request.actor.actor_type) {
                return Ok(GatewayDecision::ApprovalRequired {
                    reason,
                    approval_request_id: String::new(),
                });
            }
            return Ok(GatewayDecision::Deny { reason });
        }

        if let Some(grant) =
            PermissionApprovalService::matching_active_grant(active_grants, request)
        {
            return Ok(GatewayDecision::Allow {
                reason: format!(
                    "allow-once grant '{}' authorized '{}'",
                    grant.id, request.command
                ),
            });
        }

        match gate.authorize(request)? {
            PermissionDecision::Allowed => Ok(GatewayDecision::Allow {
                reason: format!(
                    "capability '{}' granted for '{}'",
                    request.capability.id, request.command
                ),
            }),
            PermissionDecision::Denied { reason } => Ok(GatewayDecision::Deny { reason }),
            PermissionDecision::ApprovalRequired { reason } => {
                Ok(GatewayDecision::ApprovalRequired {
                    reason,
                    approval_request_id: String::new(),
                })
            }
        }
    }

    /// Evaluates and records a durable permission decision, then enforces Allow-only.
    pub fn require(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        policy: &dyn PermissionPolicy,
        gate: &dyn PermissionGate,
        request: &PermissionRequest,
        granted: &CapabilitySet,
    ) -> Result<()> {
        let (effective, active_grants) = {
            let guard = db
                .lock()
                .map_err(|_| KernelError::lock_poisoned("database"))?;
            PermissionApprovalService::merge_active_grants(
                &guard,
                &request.actor,
                granted.clone(),
            )?
        };

        let mut decision =
            Self::evaluate_with_grants(policy, gate, request, &effective, &active_grants)?;

        let matching_grant =
            PermissionApprovalService::matching_active_grant(&active_grants, request)
                .map(|g| g.id.clone());

        if let GatewayDecision::ApprovalRequired { reason, .. } = &decision {
            let pending = {
                let guard = db
                    .lock()
                    .map_err(|_| KernelError::lock_poisoned("database"))?;
                PermissionApprovalService::ensure_pending_request(&guard, request, reason)?
            };
            decision = GatewayDecision::ApprovalRequired {
                reason: reason.clone(),
                approval_request_id: pending.id.to_string(),
            };
        }

        Self::record_decision(db, actor_context, intent_context, request, &decision);

        match decision {
            GatewayDecision::Allow { .. } => {
                if let Some(grant_id) = matching_grant {
                    let guard = db
                        .lock()
                        .map_err(|_| KernelError::lock_poisoned("database"))?;
                    PermissionApprovalService::consume_grant(&guard, &grant_id)?;
                }
                Ok(())
            }
            GatewayDecision::Deny { reason } => Err(KernelError::PermissionDenied(reason)),
            GatewayDecision::ApprovalRequired {
                reason,
                approval_request_id,
            } => Err(KernelError::ApprovalRequired {
                reason,
                approval_request_id,
            }),
        }
    }

    fn record_decision(
        db: &Arc<Mutex<Database>>,
        actor_context: &ActorContext,
        intent_context: &IntentContext,
        request: &PermissionRequest,
        decision: &GatewayDecision,
    ) {
        let (event_type, success, reason, approval_request_id) = match decision {
            GatewayDecision::Allow { reason } => {
                ("permission.allowed", true, reason.as_str(), None)
            }
            GatewayDecision::Deny { reason } => {
                ("permission.denied", false, reason.as_str(), None)
            }
            GatewayDecision::ApprovalRequired {
                reason,
                approval_request_id,
            } => (
                "permission.approval_required",
                false,
                reason.as_str(),
                Some(approval_request_id.as_str()),
            ),
        };

        let metadata = json!({
            "decision": match decision {
                GatewayDecision::Allow { .. } => "allow",
                GatewayDecision::Deny { .. } => "deny",
                GatewayDecision::ApprovalRequired { .. } => "approval_required",
            },
            "reason": reason,
            "command": request.command,
            "capability": request.capability.id.as_str(),
            "subject": format!("{:?}", request.subject),
            "actor_type": format!("{:?}", request.actor.actor_type),
            "intent_type": format!("{:?}", request.intent.intent_type),
            "approval_request_id": approval_request_id,
        })
        .to_string();

        if let Err(error) = AuditService::record_permission_decision(
            db,
            actor_context,
            intent_context,
            event_type,
            request.command,
            &request.capability,
            success,
            metadata,
        ) {
            log::error!(
                "failed to record permission decision for {}: {error}",
                request.command
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{AlwaysAllowPolicy, CapabilityBoundPolicy};
    use crate::security::{AllowAllPermissionGate, PermissionSubject, StandardPermissionGate};
    use workspace_domain::{Actor, Capability, Intent, ResourceKind};

    fn workspace_write_request(actor: Actor) -> PermissionRequest {
        PermissionRequest {
            actor,
            intent: Intent::user_request(),
            capability: Capability::workspace_write(),
            command: "CreateWorkspace",
            subject: PermissionSubject::Resource(ResourceKind::Workspace),
            target_resource_id: None,
        }
    }

    #[test]
    fn allows_granted_local_user_capability() {
        let decision = PermissionGateway::evaluate(
            &CapabilityBoundPolicy,
            &StandardPermissionGate,
            &workspace_write_request(Actor::local_user()),
            &CapabilitySet::local_user_standard(),
        )
        .unwrap();

        assert!(matches!(decision, GatewayDecision::Allow { .. }));
    }

    #[test]
    fn denies_ungranted_capability_for_local_user() {
        let decision = PermissionGateway::evaluate(
            &CapabilityBoundPolicy,
            &AllowAllPermissionGate,
            &workspace_write_request(Actor::local_user()),
            &CapabilitySet::new(),
        )
        .unwrap();

        assert!(matches!(decision, GatewayDecision::Deny { .. }));
    }

    #[test]
    fn approval_required_for_ai_missing_capability() {
        let decision = PermissionGateway::evaluate(
            &CapabilityBoundPolicy,
            &StandardPermissionGate,
            &workspace_write_request(Actor::ai_assistant("ai-1").unwrap()),
            &CapabilitySet::new(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            GatewayDecision::ApprovalRequired { .. }
        ));
    }

    #[test]
    fn approval_required_for_ai_even_with_capability() {
        let decision = PermissionGateway::evaluate(
            &AlwaysAllowPolicy,
            &StandardPermissionGate,
            &workspace_write_request(Actor::ai_assistant("ai-1").unwrap()),
            &CapabilitySet::local_user_standard(),
        )
        .unwrap();

        assert!(matches!(
            decision,
            GatewayDecision::ApprovalRequired { .. }
        ));
    }

}
