# Workspace Governance Condition & Obligation Contract

Sprint 151 — formal model for conditions attached to governance approvals.

**Conditions never execute work, never grant authority, never modify cognition.**

## Principle

Approvals may carry obligations. Tracking obligation status is architecture metadata only —
satisfying an obligation does not run commands or activate publication.

## GovernanceConditionContract

| Element | Meaning |
|---------|---------|
| `obligations` | Declared conditions on an approve decision |
| `decision_reference` | Linked `GovernanceReviewDecision` |
| `provenance` | Frozen recommendation provenance |
| `authority_effect` | Always `none` |

Obligation kinds:

- `RequiresDocumentation`
- `RequiresTesting`
- `RequiresCompatibilityVerification`
- `RequiresRollbackPlan`
- `RequiresSecondReviewer`
- `ExpiresIfUnmet`

Status: `Declared` → `Pending` → `Satisfied` | `Unmet` | `Expired`.

Guards: `may_execute`, `may_grant_authority`, `may_mutate_cognition` are all false.

---

## Related docs

- [WORKSPACE-GOVERNANCE-COMPATIBILITY.md](./WORKSPACE-GOVERNANCE-COMPATIBILITY.md)
- [WORKSPACE-GOVERNANCE-INTEGRITY.md](./WORKSPACE-GOVERNANCE-INTEGRITY.md)
- [WORKSPACE-GOVERNANCE-ARCHIVE.md](./WORKSPACE-GOVERNANCE-ARCHIVE.md)
- [WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md)
- [WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md](./WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
