# Workspace Governance Readiness Dashboard Contract

Sprint 159 — domain projection for a future governance workspace.

**Not UI implementation. No execution. No publication.**

## Principle

`GovernanceReadinessDashboardProjection` aggregates projection fields only:

- review progress (completed / total + workflow stage)
- outstanding obligations
- unresolved conflicts
- compliance status
- publication readiness state
- archived history count
- decision package reference

## Guards

- `may_execute` / `may_publish` false
- `attempt_execute` → dashboard cannot execute
- `attempt_publish` → activation not implemented

Projection readiness (`is_publication_ready_projection`) is informational only —
it does not activate runtime publication.

---

## Related docs

- [WORKSPACE-GOVERNANCE-NOTIFICATIONS.md](./WORKSPACE-GOVERNANCE-NOTIFICATIONS.md)
- [WORKSPACE-GOVERNANCE-METRICS.md](./WORKSPACE-GOVERNANCE-METRICS.md)
- [WORKSPACE-GOVERNANCE-REPORTING.md](./WORKSPACE-GOVERNANCE-REPORTING.md)
- [WORKSPACE-GOVERNANCE-EXPORT.md](./WORKSPACE-GOVERNANCE-EXPORT.md)
- [WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md](./WORKSPACE-GOVERNANCE-REVIEW-WORKFLOW.md)
- [WORKSPACE-GOVERNANCE-CONFLICT.md](./WORKSPACE-GOVERNANCE-CONFLICT.md)
- [WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md](./WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md)
- [WORKSPACE-GOVERNANCE-COMPLIANCE.md](./WORKSPACE-GOVERNANCE-COMPLIANCE.md)
- [WORKSPACE-GOVERNANCE-WORKSPACE.md](./WORKSPACE-GOVERNANCE-WORKSPACE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
