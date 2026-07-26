//! # Governance subsystem (Sprints 165–169 consolidation)
//!
//! Coherent aggregate layout for adaptation governance. Public types remain
//! re-exported from [`crate::action_proposal`] / [`crate`] — this module only
//! organises ownership and dependency direction.
//!
//! ## Aggregate roots
//!
//! | Aggregate | Responsibility | Primary types |
//! |-----------|----------------|---------------|
//! | Ledger | Historical refs + publish request labels | `GovernanceRecord`, `PublishRequest`, `GovernanceTimeline` |
//! | Policy & risk | Review rules / routing / decisions / evidence | `GovernancePolicy`, `GovernanceRisk`, `GovernanceReviewDecision`, `GovernanceDecisionEvidence` |
//! | Publication prep | Readiness / environment / safety gates | `PublicationReadiness`, `PublicationEnvironment`, `PublicationSafetyContract` |
//! | Preconditions | Conditions, compatibility, integrity, archive | `contracts` module |
//! | Review ops | Workflow, conflict, package, compliance, dashboard | `workflow` module |
//! | Observability | Notifications, delegation, metrics, reports, export | `ops` module |
//!
//! ## Ownership boundaries (Sprint 166)
//!
//! | Owner | Owns | Must not own |
//! |-------|------|--------------|
//! | **Governance** | Review, ledger, conditions, compliance, archive, export | Command execution, Gateway Allow |
//! | **Experience** | DisplayReason / translation traces | Governance decisions |
//! | **Permission Gateway** | Execution Allow / Deny / ApprovalRequired | Adaptation publish activation |
//! | **Domain facts** | Observation, WorkspaceState schema facts | Governance authority |
//! | **UI** | Presentation of governance projections | Approval that grants execution |
//!
//! ## Dependency direction (Sprint 169)
//!
//! ```text
//! ops ──► workflow ──► contracts ──► ledger/policy/publication types (parent)
//!                ╲         │
//!                 ╲        ▼
//!                  └► RecommendationProvenance (immutable facts)
//! ```
//!
//! Observability and review-ops depend inward on preconditions and ledger types.
//! Nothing in governance depends on UI or Gateway Allow paths.
//!
//! ## Authority
//!
//! All governance artifacts use [`GOVERNANCE_AUTHORITY_EFFECT_NONE`].
//! Governance never executes commands and never activates publication.

mod authority;
mod contracts;
mod ops;
mod workflow;

pub use authority::*;
pub use contracts::*;
pub use ops::*;
pub use workflow::*;

/// Named aggregate roots for boundary audits and documentation (Sprint 165/169).
/// Not a runtime registry — naming consistency only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernanceAggregateRoot {
    Ledger,
    PolicyRisk,
    PublicationPrep,
    Preconditions,
    ReviewOps,
    Observability,
}

impl GovernanceAggregateRoot {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ledger => "ledger",
            Self::PolicyRisk => "policy_risk",
            Self::PublicationPrep => "publication_prep",
            Self::Preconditions => "preconditions",
            Self::ReviewOps => "review_ops",
            Self::Observability => "observability",
        }
    }

    /// Canonical module / type-family owner for each aggregate.
    pub fn owner_domain(self) -> &'static str {
        "governance"
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Ledger,
            Self::PolicyRisk,
            Self::PublicationPrep,
            Self::Preconditions,
            Self::ReviewOps,
            Self::Observability,
        ]
    }
}

/// Ownership domains referenced by the governance boundary audit (Sprint 166).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernanceBoundaryOwner {
    Governance,
    Experience,
    PermissionGateway,
    DomainFacts,
    UiPresentation,
}

impl GovernanceBoundaryOwner {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Governance => "governance",
            Self::Experience => "experience",
            Self::PermissionGateway => "permission_gateway",
            Self::DomainFacts => "domain_facts",
            Self::UiPresentation => "ui_presentation",
        }
    }
}

#[cfg(test)]
mod consolidation_tests {
    use super::*;

    #[test]
    fn aggregate_roots_are_named_and_governance_owned() {
        for root in GovernanceAggregateRoot::all() {
            assert!(!root.as_str().is_empty());
            assert_eq!(root.owner_domain(), "governance");
        }
        assert_eq!(GovernanceAggregateRoot::all().len(), 6);
    }

    #[test]
    fn authority_effect_constant_is_none() {
        assert_eq!(GOVERNANCE_AUTHORITY_EFFECT_NONE, "none");
    }

    #[test]
    fn boundary_owners_are_distinct() {
        let owners = [
            GovernanceBoundaryOwner::Governance,
            GovernanceBoundaryOwner::Experience,
            GovernanceBoundaryOwner::PermissionGateway,
            GovernanceBoundaryOwner::DomainFacts,
            GovernanceBoundaryOwner::UiPresentation,
        ];
        let mut labels: Vec<_> = owners.iter().map(|o| o.as_str()).collect();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), 5);
    }
}
