# Workspace Governance Decision Package Contract

Sprint 157 — canonical immutable review artifact.

**No activation. Seal freezes the package.**

## Principle

The decision package aggregates references across the governance surface:

- proposal, evidence, conditions/obligations, reviewers
- risk, compatibility, integrity verification
- archive snapshots, publication readiness
- workflow + conflict resolution refs

`seal()` makes the package immutable for further mutation attempts.

## GovernanceDecisionPackage

| Field | Role |
|-------|------|
| `*_reference` fields | Stable ids into other contracts |
| `obligation_ids` / `reviewer_actor_ids` | Embedded identity lists |
| `sealed` | Immutability latch |
| `authority_effect` | Always `none` |

---

## Related docs

- [WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md](./WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md)
- [WORKSPACE-GOVERNANCE-CONFLICT.md](./WORKSPACE-GOVERNANCE-CONFLICT.md)
- [WORKSPACE-GOVERNANCE-COMPLIANCE.md](./WORKSPACE-GOVERNANCE-COMPLIANCE.md)
- [WORKSPACE-GOVERNANCE-CONDITIONS.md](./WORKSPACE-GOVERNANCE-CONDITIONS.md)
- [WORKSPACE-GOVERNANCE-INTEGRITY.md](./WORKSPACE-GOVERNANCE-INTEGRITY.md)
- [WORKSPACE-GOVERNANCE-ARCHIVE.md](./WORKSPACE-GOVERNANCE-ARCHIVE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
