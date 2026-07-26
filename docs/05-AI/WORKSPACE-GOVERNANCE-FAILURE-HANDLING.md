# Workspace Governance Resilience & Failure Handling

Sprint 150 — failure handling across the governance and publication preparation lifecycle.

**No runtime activation. Recovery never executes. Failures never delete provenance or grant authority.**

## Principle

```
FailureDetected
    ↓
Recorded
    ↓
RecoveryPlanned
    ↓
Recovered | Abandoned
```

Abandoned and recovered paths remain historically visible. Evidence and provenance are preserved.

---

## GovernanceFailureState

| Field | Meaning |
|-------|---------|
| `category` | Failure category (pattern source audited; domains not merged) |
| `lifecycle_stage` | Governance stage where failure was observed |
| `actors` | `GovernanceActorRefs` (identity refs — not capability grants) |
| `recovery_requirements` | Safe recovery plan markers |
| `preserved_evidence_references` | Evidence ids retained after failure |
| `recovery_state` / `recovery_history` | Recovery lifecycle + append-only history |
| `provenance` | Frozen recommendation provenance |
| `historically_visible` | Always true for recorded failures / abandoned paths |

Categories:

| Category | Pattern source (audit) |
|----------|------------------------|
| `MigrationApply` | MigrationRunner apply failures |
| `CommandValidation` | Command validation preconditions |
| `PermissionDenial` | Permission Gateway Deny |
| `AuditTrail` | AuditService / append-only trail failures |
| `ReviewExpiry` | GovernancePolicyExpired / proposal expire |
| `PublicationValidation` | PublicationSafetyContract validation gates |
| `ReviewRejection` | GovernanceReviewDecision Reject |

Guards:

- `may_delete_provenance() == false`
- `may_grant_authority() == false`
- `may_activate_runtime() == false`
- `may_bypass_permission_gateway() == false`
- `attempt_recovery_execute()` → blocked
- Unsafe recovery requirements rejected at `plan_recovery`

Builders: `from_failed_review`, `from_publication_validation_failure`, `from_review_expiry`, `detect`.

---

## Audit of reusable failure patterns

| System | Pattern | Boundary |
|--------|---------|----------|
| **MigrationRunner** | Failed apply blocks init; prior versions remain | Reusable block-on-failure — **not** cognition mutation |
| **Command validation** | Precondition fail before pipeline work | Reusable record + stop |
| **Permission denials** | Gateway Deny / ApprovalRequired | **Separate** execution domain — failure handling never Allows |
| **Audit failures** | Append-only trail; warn/log without erase | Reusable preserve-history |
| **Review expiry** | Expired blocks further approval | Reusable stop-without-authority |

**Do not merge:** Gateway Deny/Allow with governance recovery planning.

---

## Forbidden

Failure handling / recovery must not:

- delete provenance
- grant authority
- activate runtime changes
- bypass Permission Gateway
- execute commands as “recovery”

---

## Related docs

- [WORKSPACE-GOVERNANCE.md](./WORKSPACE-GOVERNANCE.md) *(canonical index)*
- [WORKSPACE-PUBLICATION-SAFETY.md](./WORKSPACE-PUBLICATION-SAFETY.md)
- [WORKSPACE-GOVERNANCE-LIFECYCLE.md](./WORKSPACE-GOVERNANCE-LIFECYCLE.md)
- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-GOVERNANCE-POLICY.md](./WORKSPACE-GOVERNANCE-POLICY.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
