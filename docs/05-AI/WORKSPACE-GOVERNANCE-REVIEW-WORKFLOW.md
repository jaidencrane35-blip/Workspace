# Workspace Governance Review Workflow Contract

Sprint 155 — complete review workflow architecture (metadata only).

**No execution. No publication.**

## Principle

```
Unassigned → Assigned → InQueue → InReview → Completed
                              ↘ Escalated ↗
```

Supports ordered, parallel, and optional review modes; multiple reviewers;
reassignment; escalation; pending queue.

## GovernanceReviewWorkflowContract

| Element | Meaning |
|---------|---------|
| `assignments` | Reviewer assignment records |
| `pending_queue` | Assignment ids awaiting review |
| `mode` | `Ordered` \| `Parallel` \| `Optional` |
| `stage` / `stage_history` | Workflow stage tracking |
| `escalation_notes` | Append-only escalation markers |

Guards: `may_execute` / `may_publish` false; hard-fail attempt APIs.

---

## Related docs

- [WORKSPACE-GOVERNANCE-CONFLICT.md](./WORKSPACE-GOVERNANCE-CONFLICT.md)
- [WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md](./WORKSPACE-GOVERNANCE-DECISION-PACKAGE.md)
- [WORKSPACE-GOVERNANCE-COMPLIANCE.md](./WORKSPACE-GOVERNANCE-COMPLIANCE.md)
- [WORKSPACE-GOVERNANCE-DASHBOARD.md](./WORKSPACE-GOVERNANCE-DASHBOARD.md)
- [WORKSPACE-ADAPTATION-REVIEW.md](./WORKSPACE-ADAPTATION-REVIEW.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
