//! Permission approval service cleanup — remove unused helpers.

use chrono::Utc;
use workspace_database::{Database, PermissionApprovalRepository};
use workspace_domain::{
    Actor, ActorContext, ActorType, ApprovalDecisionKind, ApprovalDecisionResult, CapabilityGrant,
    CapabilityGrantId, CapabilityGrantStatus, CapabilityId, CapabilitySet, GrantKind,
    PermissionApprovalRequest, PermissionApprovalRequestId, PermissionApprovalStatus,
};

use crate::error::{KernelError, Result};
use crate::security::PermissionRequest;

/// Coordinates approval request persistence and capability grants.
pub struct PermissionApprovalService;

impl PermissionApprovalService {
    /// Ensures a pending approval exists for this blocked request (idempotent).
    pub fn ensure_pending_request(
        db: &Database,
        request: &PermissionRequest,
        reason: &str,
    ) -> Result<PermissionApprovalRequest> {
        let repo = PermissionApprovalRepository::new(db);
        if let Some(existing) = repo.find_pending(
            request.actor.id.as_str(),
            request.command,
            request.capability.id.as_str(),
        )? {
            return Ok(existing);
        }

        let pending = PermissionApprovalRequest {
            id: PermissionApprovalRequestId::generate(),
            created_at: Utc::now().to_rfc3339(),
            status: PermissionApprovalStatus::Pending,
            requesting_actor_type: request.actor.actor_type,
            requesting_actor_id: request.actor.id.to_string(),
            command_name: request.command.to_string(),
            capability: request.capability.id.to_string(),
            subject: format!("{:?}", request.subject),
            intent_type: request.intent.intent_type,
            reason: reason.to_string(),
            decided_at: None,
            decided_by_actor_id: None,
        };
        repo.create_request(&pending)?;
        Ok(pending)
    }

    pub fn list_recent(db: &Database, limit: Option<usize>) -> Result<Vec<PermissionApprovalRequest>> {
        PermissionApprovalRepository::new(db)
            .list_recent(limit.unwrap_or(50))
            .map_err(Into::into)
    }

    pub fn active_grants_for_actor(db: &Database, actor_id: &str) -> Result<Vec<CapabilityGrant>> {
        PermissionApprovalRepository::new(db)
            .list_active_grants_for_actor(actor_id)
            .map_err(Into::into)
    }

    pub fn merge_active_grants(
        db: &Database,
        actor: &Actor,
        base: CapabilitySet,
    ) -> Result<(CapabilitySet, Vec<CapabilityGrant>)> {
        let grants = Self::active_grants_for_actor(db, actor.id.as_str())?;
        let mut set = base;
        for grant in &grants {
            if let Ok(id) = CapabilityId::new(grant.capability.clone()) {
                set = set.with_capability_id(id);
            }
        }
        Ok((set, grants))
    }

    pub fn matching_active_grant<'a>(
        grants: &'a [CapabilityGrant],
        request: &PermissionRequest,
    ) -> Option<&'a CapabilityGrant> {
        grants.iter().find(|grant| {
            grant.grantee_actor_id == request.actor.id.as_str()
                && grant.capability == request.capability.id.as_str()
                && grant
                    .command_name
                    .as_deref()
                    .map(|name| name == request.command)
                    .unwrap_or(true)
                && grant.status == CapabilityGrantStatus::Active
        })
    }

    pub fn consume_grant(db: &Database, grant_id: &CapabilityGrantId) -> Result<()> {
        let consumed = PermissionApprovalRepository::new(db)
            .consume_grant(grant_id, &Utc::now().to_rfc3339())?;
        if !consumed {
            return Err(KernelError::PermissionApprovalValidation {
                message: format!("grant '{grant_id}' could not be consumed"),
            });
        }
        Ok(())
    }

    pub fn decide(
        db: &Database,
        decider: &ActorContext,
        request_id: &PermissionApprovalRequestId,
        decision: ApprovalDecisionKind,
    ) -> Result<ApprovalDecisionResult> {
        if decider.actor.actor_type != ActorType::LocalUser {
            return Err(KernelError::PermissionDenied(
                "only the local user may decide permission approvals".into(),
            ));
        }

        let repo = PermissionApprovalRepository::new(db);
        let mut request = repo
            .get_request(request_id)?
            .ok_or(KernelError::PermissionApprovalNotFound)?;

        if request.status != PermissionApprovalStatus::Pending {
            return Err(KernelError::PermissionApprovalValidation {
                message: "approval request is not pending".into(),
            });
        }

        let now = Utc::now().to_rfc3339();
        request.decided_at = Some(now.clone());
        request.decided_by_actor_id = Some(decider.actor.id.to_string());

        let grant = match decision {
            ApprovalDecisionKind::AllowOnce => {
                request.status = PermissionApprovalStatus::Approved;
                repo.update_request(&request)?;

                let grant = CapabilityGrant {
                    id: CapabilityGrantId::generate(),
                    approval_request_id: request.id.clone(),
                    grantee_actor_id: request.requesting_actor_id.clone(),
                    capability: request.capability.clone(),
                    command_name: Some(request.command_name.clone()),
                    grant_kind: GrantKind::AllowOnce,
                    status: CapabilityGrantStatus::Active,
                    created_at: now,
                    consumed_at: None,
                };
                repo.create_grant(&grant)?;
                Some(grant)
            }
            ApprovalDecisionKind::Deny => {
                request.status = PermissionApprovalStatus::Denied;
                repo.update_request(&request)?;
                None
            }
        };

        Ok(ApprovalDecisionResult { request, grant })
    }

    pub fn is_non_human(actor_type: ActorType) -> bool {
        matches!(
            actor_type,
            ActorType::AIAssistant
                | ActorType::Automation
                | ActorType::Plugin
                | ActorType::RemoteSession
        )
    }
}
