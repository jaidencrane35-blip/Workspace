# Workspace Governance Compliance Contract

Sprint 158 — compliance verification producing diagnostics only.

**Never repairs automatically. Never grants authority.**

## Checks

- required evidence exists
- reviewer count satisfied
- required obligations completed
- compatibility verified
- integrity verified
- publication safety satisfied

## GovernanceComplianceContract

| API | Behaviour |
|-----|-----------|
| `verify(...)` | Emit Info / Warning / Error diagnostics |
| `has_errors()` | True if any Error diagnostic |
| `attempt_repair` | Hard-fail |
| `attempt_grant_authority` | Hard-fail |

---

## Related docs

- [WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md](./WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md)
- [WORKSPACE-GOVERNANCE-INTEGRITY.md](./WORKSPACE-GOVERNANCE-INTEGRITY.md)
- [WORKSPACE-PUBLICATION-SAFETY.md](./WORKSPACE-PUBLICATION-SAFETY.md)
- [WORKSPACE-GOVERNANCE-COMPATIBILITY.md](./WORKSPACE-GOVERNANCE-COMPATIBILITY.md)
- [WORKSPACE-GOVERNANCE-DASHBOARD.md](./WORKSPACE-GOVERNANCE-DASHBOARD.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
