# Workspace Governance Delegation Contract

Sprint 161 — review delegation architecture.

**Delegation cannot transfer execution authority. Review scope only.**

## GovernanceDelegationContract

| Element | Meaning |
|---------|---------|
| `delegations` | Delegated reviewer records + chain parent |
| `max_chain_depth` | Hard limit on delegation chain |
| `temporary` / `expires_at` | Temporary delegation markers |
| `audit_events` | Append-only delegation audit |
| `scope` | Always `governance_review_only` |

Supports revoke and expire. `attempt_transfer_execution` hard-fails.

---

## Related docs

- [WORKSPACE-GOVERNANCE-NOTIFICATIONS.md](./WORKSPACE-GOVERNANCE-NOTIFICATIONS.md)
- [WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md](./WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md)
- [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
