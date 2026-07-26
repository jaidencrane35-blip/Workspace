# Workspace Governance Lifecycle & Timeline

Sprint 148 — end-to-end governance lifecycle integrity proving provenance and authority
boundaries across all stages.

**No runtime activation. Timeline events are immutable. Governance never executes commands.**

## Principle

```
Change Proposal
    ↓
Risk Classification
    ↓
Evidence Collection
    ↓
Review
    ↓
Decision
    ↓
Governance Record
    ↓
Workspace Presentation
    ↓
Publication Readiness
```

Rejected changes remain historically visible (`RejectedVisible` stage). Dissent refs on the
timeline are immutable.

---

## GovernanceTimeline architecture

| Element | Meaning |
|---------|---------|
| `events` | Append-only `GovernanceTimelineEvent` list |
| `stage` | Lifecycle stage enum |
| `at` | Timestamp |
| `actor_reference` | Who acted (when applicable) |
| `evidence_reference` | Linked evidence id |
| `decision_reference` | Linked review decision id |
| `provenance` | Frozen recommendation provenance for the whole chain |
| `rejected_visible` | Rejected path remains on timeline |
| `dissent_immutable_refs` | Dissent ids that must never be erased |

Guards:

- `may_rewrite_events() == false`
- `attempt_rewrite_event()` → `GovernanceTimelineImmutable`
- `may_execute() == false` / `attempt_execute()` hard-fails
- `attempt_activate_runtime()` hard-fails
- `validate_lifecycle_integrity()` requires all happy-path stages

Builders:

- `from_full_lifecycle(...)` — approved path through readiness
- `from_rejected_lifecycle(...)` — reject remains visible

---

## Audit of existing event systems

| System | Pattern | Boundary |
|--------|---------|----------|
| **AuditService** | Durable append-only command/permission/AI events | Execution / ops trail — not merged into governance timeline authority |
| **GovernanceRecord** | Ledger refs across proposal → publish | Direct input to timeline stages |
| **Recommendation provenance** | Frozen reasoning origins | Timeline `provenance` snapshot |
| **Experience traces** | Translation diagnostics | Optional evidence refs only |
| **Decision records** | Review / AI planning id events | Decision stage references — not Gateway Allow |

**Reuse:** append-only events, actor + timestamp + reference ids.  
**Do not merge:** Permission Gateway Allow domain with adaptation governance timeline.

---

## Related docs

- [WORKSPACE-GOVERNANCE-WORKSPACE.md](./WORKSPACE-GOVERNANCE-WORKSPACE.md)
- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-GOVERNANCE-LEDGER.md](./WORKSPACE-GOVERNANCE-LEDGER.md)
- [WORKSPACE-GOVERNANCE-RISK.md](./WORKSPACE-GOVERNANCE-RISK.md)
- [WORKSPACE-RECOMMENDATION-PROVENANCE.md](./WORKSPACE-RECOMMENDATION-PROVENANCE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
