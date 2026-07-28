# State Transition and Lifecycle Governance

| Field | Value |
|-------|-------|
| **Purpose** | Define explicit lifecycle rules enforced at existing domain and kernel authorities |
| **Owner** | Platform Kernel |
| **Status** | Active |

## Rules

Lifecycle mutation must pass through the owning type or service transition guard.
Invalid transitions return the subsystem's existing classified validation error;
kernel runtime and capture integrity violations return `integrity_violation`.
Rejected transitions do not mutate state or emit a success event.

| Lifecycle | Valid progression clarified by this phase | Terminal states |
|-----------|--------------------------------------------|-----------------|
| Workspace runtime | `starting → initializing → ready → shutting_down`; `starting`, `initializing`, or `ready` may enter `error` as defined by the runtime matrix | `error`, `shutting_down` |
| Observation capture | `requested → started → completed|failed`; concurrent requests use `requested → rejected_concurrent` | `completed`, `failed`, `rejected_concurrent` |
| Decision Queue overlay | `pending → viewed|deferred|dismissed`; `viewed ↔ deferred`; either may enter `dismissed` | `dismissed` plus source-owned `accepted`, `rejected`, `expired` |
| Task Graph status | Open statuses may move to another distinct status; `completed` and `cancelled` cannot reopen | `completed`, `cancelled` |
| Recommendation activity projection | Only absent, `created`, `available`, and `presented` values are actionable; unknown persisted values fail closed | `accepted`, `rejected`, `expired`, `superseded`; unknown values are non-actionable |
| Decision Engine intake evaluation | An active intake candidate may be evaluated once; a persisted evaluation cannot be overwritten by reevaluation | `evaluated`, `rejected`, `deferred` evaluation record |
| Suggestion lifecycle projection | Observed audit states move from `created|presented` toward `accepted|rejected|expired`; repeated presentation is observationally valid | `accepted`, `rejected`, `expired` |
| Governed intent execution | Unclaimed, retryable `failed`, or `cancelled` → `in_progress` → `completed|failed`; mapped durable writes are transactional; stale claims reconcile to non-retryable `failed` | `completed` and non-retryable `failed`; `cancelled` and rolled-back `failed` permit retry |

## Authority boundaries

- Command handlers and the execution pipeline remain dispatch authorities; this
  phase does not add command states or alter valid command results.
- Recommendation Engine and Decision Engine domain types remain their existing
  lifecycle authorities. No lifecycle is merged across those systems.
- Audit remains append-only evidence. It does not become a mutable state machine.
- Audit-derived projections remain read-only. Sequence validation rejects an
  impossible observed lifecycle but does not rewrite history.
- Source-projected Task Graph synchronization remains source-authoritative.
  Explicit Task Graph status commands use the Task Graph transition guard.

## Observability

Existing audit and domain event paths remain the observability mechanism. A
successful transition continues to emit its existing event. Invalid transitions
return before persistence or success-event publication. No parallel lifecycle
event system is introduced.

## Related

- [Architecture Governance](./ARCHITECTURE-GOVERNANCE.md)
- `packages/kernel/src/lifecycle/mod.rs`
- `packages/kernel/src/state/mod.rs`
- `packages/kernel/src/services/capture_coordinator.rs`
- `packages/domain/src/decision_queue/mod.rs`
- `packages/domain/src/workspace_task_graph/mod.rs`
- `packages/domain/src/suggestion_lifecycle/mod.rs`
- [Governed Execution Audit Durability](./GOVERNED-AUDIT-DURABILITY.md)
