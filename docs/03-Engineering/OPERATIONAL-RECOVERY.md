# Operational Integrity & Recovery

| Field | Value |
|-------|-------|
| **Purpose** | Recovery contracts under failure, restart, and partial state — without new lifecycle authorities |
| **Owner** | Platform Kernel |
| **Status** | Active |
| **Dependencies** | [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md), [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md), [Governed Audit Durability](./GOVERNED-AUDIT-DURABILITY.md), [Architecture Governance](./ARCHITECTURE-GOVERNANCE.md) |

## Architecture gap summary (pre-implementation)

| Gap | Severity | Response in this phase |
|-----|----------|------------------------|
| No startup sweep of stale `in_progress` execution claims | P0 | `ExecutionLifecycleService::reconcile_stale_claims_at_startup` after Ready |
| Startup domain events published before audit subscriber | P1 | Publish Started/Ready only after `apply_runtime` registers subscriber |
| Recommendation supersede + fresh Available non-atomic | P1 | Dual overlay writes in one DB transaction |
| DE candidate creation + overlay non-atomic | P2 | Same transactional dual-write |
| SQLite durability pragmas limited to FKs | P2 | WAL + `synchronous=NORMAL` on file DB connections |

## Recovery invariants

| Question | Contract |
|----------|----------|
| What survives restart? | Durable rows: execution lifecycle, recommendation/DE/DQ/task overlays, audit events, schema/migrations ledger |
| What is reconstructed? | Projections re-derived from services on generate; execution stale claims reconciled via existing service rules |
| What is missing? | In-memory AI plan/workflow stores (diagnostic only); unbound audit windows beyond scan limits |
| What must fail closed? | Incomplete terminal evidence; empty capability grants; poisoned DB locks; interrupted migrations; non-retryable stale claims |

Recovery must **never**:

- invent terminal evidence
- recreate desktop actions
- bypass PermissionGateway / CommandPipeline
- silently “heal” lifecycle into an open actionable state
- erase or weaken terminal history

## Reconciliation boundary

```
Persisted State (repository + service owners)
        ↓
Derived Projection (generate / list / get)
        ↓
Consistency Check (service-owned; e.g. stale claim age)
        ↓
Detect → Explain (failure_reason / integrity notes)
        ↓
Repair only through existing service/command paths
```

No direct database repair from a parallel reconciler. Startup may **invoke** `ExecutionLifecycleService` so existing `reconcile_stale` runs; it does not invent a second authority.

## Startup integrity sequence

1. Bootstrap shell (event bus present)
2. InitializeWorkspace: open DB → migrations → configuration → Ready state
3. `apply_runtime` — attach DB, register **AuditEventSubscriber**
4. Publish `WorkspaceStarted` / `WorkspaceReady` (now auditable)
5. `ExecutionLifecycleService::reconcile_stale_claims_at_startup` (fail-closed mark of stale claims; soft-log on list errors)
6. Observation startup trigger (existing soft-fail)

## Audit durability

- Pre-dispatch `command.authorized` / permission decisions remain fail-closed.
- Startup lifecycle events must hit audit after subscriber registration.
- Recovery and reconciliation must not delete audit or terminal overlay evidence.

## Related code

- `packages/kernel/src/services/execution_lifecycle.rs` — startup stale claim sweep
- `packages/kernel/src/commands/handler.rs` / `initialize.rs` — startup ordering
- `packages/database/src/connection.rs` — `run_in_transaction`, WAL pragmas
- `packages/domain/src/recovery_contract.rs` — shared recovery predicates for tests
