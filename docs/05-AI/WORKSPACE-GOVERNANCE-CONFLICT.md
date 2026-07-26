# Workspace Governance Conflict Resolution Contract

Sprint 156 — reviewer disagreement architecture.

**Cannot publish. Cannot execute. Cannot modify history.**

## Principle

```
Detected → UnderArbitration → ConsensusReached | Unresolved
```

Conflict kinds: conflicting decisions, conflicting conditions, dissent recorded,
arbitration required.

Consensus rules: unanimous, majority, quorum with arbiter.

## GovernanceConflictResolutionContract

| Guard | Value |
|-------|-------|
| `history_immutable` | `true` |
| `may_publish` | `false` |
| `may_execute` | `false` |
| `may_modify_history` | `false` |
| `is_blocking_publication` | true while unresolved / in conflict |

---

## Related docs

- [WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md](./WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md)
- [WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md](./WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md)
- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md](./WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
