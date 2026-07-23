//! Permission approval requests and capability grants (Sprint 43).
//!
//! Human control loop:
//! Request → Gateway → ApprovalRequired → User Decision → Capability Grant → Retry Allowed

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actor::ActorType;
use crate::ids::{CapabilityGrantId, PermissionApprovalRequestId};
use crate::intent::IntentType;

pub type Result<T> = std::result::Result<T, PermissionApprovalError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PermissionApprovalError {
    #[error("Permission approval request not found")]
    NotFound,
    #[error("Permission approval request is not pending")]
    NotPending,
    #[error("Invalid permission approval state: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionApprovalStatus {
    Pending,
    Approved,
    Denied,
}

impl PermissionApprovalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Denied => "denied",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "pending" => Ok(Self::Pending),
            "approved" => Ok(Self::Approved),
            "denied" => Ok(Self::Denied),
            other => Err(PermissionApprovalError::Invalid(format!(
                "unknown approval status '{other}'"
            ))),
        }
    }
}

/// Durable pending/decided approval for a blocked permission check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionApprovalRequest {
    pub id: PermissionApprovalRequestId,
    pub created_at: String,
    pub status: PermissionApprovalStatus,
    pub requesting_actor_type: ActorType,
    pub requesting_actor_id: String,
    pub command_name: String,
    pub capability: String,
    pub subject: String,
    pub intent_type: IntentType,
    pub reason: String,
    pub decided_at: Option<String>,
    pub decided_by_actor_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantKind {
    AllowOnce,
    Lasting,
}

impl GrantKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowOnce => "allow_once",
            Self::Lasting => "lasting",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "allow_once" => Ok(Self::AllowOnce),
            "lasting" => Ok(Self::Lasting),
            other => Err(PermissionApprovalError::Invalid(format!(
                "unknown grant kind '{other}'"
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityGrantStatus {
    Active,
    Consumed,
    Revoked,
}

impl CapabilityGrantStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Consumed => "consumed",
            Self::Revoked => "revoked",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "active" => Ok(Self::Active),
            "consumed" => Ok(Self::Consumed),
            "revoked" => Ok(Self::Revoked),
            other => Err(PermissionApprovalError::Invalid(format!(
                "unknown grant status '{other}'"
            ))),
        }
    }
}

/// Capability grant issued after an approval decision (allow-once in this batch).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityGrant {
    pub id: CapabilityGrantId,
    pub approval_request_id: PermissionApprovalRequestId,
    pub grantee_actor_id: String,
    pub capability: String,
    pub command_name: Option<String>,
    pub grant_kind: GrantKind,
    pub status: CapabilityGrantStatus,
    pub created_at: String,
    pub consumed_at: Option<String>,
}

/// Local-user decision on a pending approval request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionKind {
    AllowOnce,
    Deny,
}

impl ApprovalDecisionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllowOnce => "allow_once",
            Self::Deny => "deny",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "allow_once" => Ok(Self::AllowOnce),
            "deny" => Ok(Self::Deny),
            other => Err(PermissionApprovalError::Invalid(format!(
                "unknown approval decision '{other}'"
            ))),
        }
    }
}

/// Result of deciding a pending approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalDecisionResult {
    pub request: PermissionApprovalRequest,
    pub grant: Option<CapabilityGrant>,
}
