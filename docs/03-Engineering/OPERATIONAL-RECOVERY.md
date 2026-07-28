# Operational Integrity & Recovery

| Field | Value |
|-------|-------|
| **Purpose** | Recovery contracts under failure, restart, and partial state — without new lifecycle authorities |
| **Owner** | Platform Kernel |
| **Status** | Active |
| **Dependencies** | [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md), [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md), [Governed Audit Durability](./GOVERNED-AUDIT-DURABILITY.md), [Architecture Governance](./ARCHITECTURE-GOVERNANCE.md) |

## Recovery authority ownership

| Concern | Owner |
|---------|-------|
| Lifecycle transitions (claim / complete / fail / cancel / stale mark) | `ExecutionLifecycleService` (and peer domain services for RE/DE/DQ/task) |
| Persistence guards (terminal reopen reject, immutable artifacts) | Repositories |
| Mutation routing / authorization | `CommandPipeline` / `PermissionGateway` |
| Startup stale-claim sweep | Invokes **existing** `ExecutionLifecycleService::reconcile_stale` — not a second authority |
| Recovery diagnostics | Append-only audit evidence (`system.recovery.*`) — observational only |
| Recovery contract helpers (`recovery_contract`) | Detector predicates for tests/docs/governance — **no mutation** |

## Recovery invariants

| Question | Contract |
|----------|----------|
| What survives restart? | Durable rows: execution lifecycle, recommendation/DE/DQ/task overlays, planning artefacts, reasoning records + history, cognitive graph snapshots + history, orchestration snapshots + history, learning snapshots + history, cognitive agent cast snapshots + history, cognitive autonomy snapshots + history, workspace state envelope snapshots + history, policy governance evaluations + history, historical reconstruction artefacts + history, temporal intelligence analyses + history, workspace explanation packages + history, contextual understanding snapshots + history, knowledge synthesis snapshots + history, knowledge integration snapshots + history, audit events, schema/migrations ledger |
| What is reconstructed? | Projections re-derived from services on generate/load; execution stale claims reconciled via existing service rules; planning/reasoning/graph/orchestration/learning/agent-cast/autonomy/state-envelope/policy-governance/historical-reconstruction/temporal-intelligence/workspace-explanation/contextual-understanding/knowledge-synthesis/knowledge-integration snapshots loaded from durable tables |
| What is missing? | In-memory AI plan/workflow stores (diagnostic only); audit windows beyond scan limits; in-progress claims beyond the startup sweep cap until lazy reconcile; **absent reasoning/graph/orchestration/learning/agent-cast/autonomy/state-envelope/policy-governance/historical-reconstruction/temporal-intelligence/workspace-explanation/contextual-understanding/knowledge-synthesis/knowledge-integration remains absent** |
| What must fail closed? | Incomplete terminal evidence; empty capability grants; poisoned DB locks; interrupted migrations; non-retryable stale claims; fabricated reasoning, graph, orchestration, learning, agent-cast, autonomy, unified-state, policy-compliance, historical-transition, temporal-cause, explanation-authority, contextual-understanding, knowledge-synthesis, or knowledge-integration structure |

Recovery must **never**:

- invent terminal evidence (including fabricating `Completed`)
- fabricate reasoning records or actionable reasoning history
- fabricate cognitive graph structure or invent missing relationships
- fabricate orchestration state, dependencies, or refresh plans
- fabricate learning lessons, success, or failure evidence
- fabricate agents, perspectives, agreement, disagreement, or confidence
- fabricate autonomy opportunities, approvals, confidence, safety guarantees, or automation history
- fabricate unified-state revisions, freshness, source availability, completeness, or conflicts
- fabricate policy compliance, approvals, or capability grants from missing evidence
- fabricate historical transitions, completeness, or repair missing reconstruction evidence
- fabricate temporal causes, forecasts, simulations, or Complete understandings from gaps
- fabricate explanation authority, approvals, conflict resolutions, or Complete packages from missing upstreams
- fabricate contextual understanding, situational certainty, conflict winners, or Complete understandings from gaps (`recovery_must_not_fabricate_contextual_understanding`)
- fabricate knowledge concepts, relationships, synthesis confidence, or historical knowledge claims from missing evidence (`recovery_must_not_fabricate_knowledge_synthesis`)
- fabricate knowledge integration hits, links, retrieval confidence, or historical integration claims from missing evidence (`recovery_must_not_fabricate_knowledge_integration`)
- fabricate insight coordination clusters, intersections, attention ranks, or historical coordination claims from missing evidence (`recovery_must_not_fabricate_insight_coordination`)
- recreate desktop actions
- bypass PermissionGateway / CommandPipeline for user mutations
- silently “heal” lifecycle into an open actionable state
- erase or weaken terminal history
- treat recovery diagnostics as lifecycle authority or retry commands

## Reconciliation boundary

```
Persisted State (repository + service owners)
        ↓
Derived Projection (generate / list / get)
        ↓
Consistency Check (service-owned; e.g. stale claim age ≥ 300s)
        ↓
Detect → Explain (failure_reason / integrity notes)
        ↓
Repair only through existing service/command paths
```

No direct database repair from a parallel reconciler. Startup may **invoke** `ExecutionLifecycleService` so existing `reconcile_stale` runs; it does not invent a second authority.

## Startup integrity sequence

1. Bootstrap shell (event bus present)
2. `InitializeWorkspace`: open DB → migrations → configuration → Ready state *(does not publish domain lifecycle events)*
3. `apply_runtime` — attach DB, register **AuditEventSubscriber**
4. Publish `WorkspaceStarted` / `WorkspaceReady` (subscriber is registered; see audit guarantees below)
5. Recovery diagnostics: append `system.recovery.startup.attempted`
6. `ExecutionLifecycleService::reconcile_stale_claims_at_startup` (fail-closed mark of stale claims)
7. Recovery diagnostics: append `system.recovery.startup.completed` **or** `system.recovery.startup.failed`
8. Observation startup trigger (existing soft-fail)

Ready continues if reconcile or diagnostic persistence fails (availability). Lifecycle truth remains in `execution_lifecycle` rows; lazy `get` / `list_recent` still apply the same stale rules.

## Startup sweep contract

| Item | Value |
|------|-------|
| Cap | `STARTUP_IN_PROGRESS_SWEEP_LIMIT` = **500** |
| Selection | `state = in_progress`, oldest `claimed_at` first |
| Action | Existing `reconcile_stale`: age ≥ 300s → `Failed`, `retry_allowed = false`, reason `stale execution claim; dispatch outcome unknown` |
| Beyond cap | Remaining in-progress rows stay durable; reconciled lazily when touched via service `get` / `list_recent` |
| Correctness | No stale claim becomes `Completed`; no row deleted; terminals do not reopen |

Rationale: bound startup latency while preserving fail-closed semantics for the oldest interrupted claims first. Lazy reconcile uses the **same** service rules — not a weaker path.

## Audit guarantees (precise)

| Guarantee | Strength |
|-----------|----------|
| Audit subscriber registration before Started/Ready publish | **Guaranteed** (ordering) |
| Domain-event / recovery-diagnostic row persistence | **Best-effort** (subscriber / diagnostic append logs on failure; Ready is not blocked) |
| Pre-dispatch `command.authorized` / permission decisions | **Fail-closed** (see Governed Audit Durability) |
| Lifecycle truth vs audit | **Lifecycle rows are authoritative**; audit is observational evidence |
| Audit wipe / update | **Rejected** (append-only triggers) |

Registration-before-publication ≠ guaranteed durable audit persistence.

Observational audit volume (startup + recovery diagnostics) must not starve
audit-derived outcome scans: `ExecutionOutcomeService` uses a minimum audit
scan floor so small `limit` queries still observe ExecuteIntentRequest evidence.

## Transactional continuity

Recommendation supersede+fresh, Decision Engine creation+overlay dual-writes, and
Reasoning Memory supersede+insert use `Database::run_in_transaction`. On second-write
failure the first write rolls back. Repositories remain transition/immutability
**guards** inside the transaction — the transaction is not a hidden lifecycle owner.

## WAL durability (file databases)

File connections set `PRAGMA journal_mode=WAL` and `synchronous=NORMAL` after foreign-key enablement. Improves crash resilience for local SQLite. In-memory databases skip WAL. This does not change authority boundaries.

## Recovery diagnostics (evidence only)

Event types:

- `system.recovery.startup.attempted`
- `system.recovery.startup.completed`
- `system.recovery.startup.failed`

Metadata includes `authority_effect: "none"`, `subsystem`, timestamps, and failure `reason` when applicable. No `command_name`. Diagnostics must not execute, retry, or mutate lifecycle.

## Intentional remaining debt

- Claim → dispatch → complete is still non-atomic by design (ambiguous completion stays fail-closed / non-retryable).
- Domain-event and recovery-diagnostic audit appends remain best-effort after registration.
- Full workspace `cargo` on Linux may require GTK (`gdk-3.0`) for the Tauri member.

## Related code

- `packages/kernel/src/services/execution_lifecycle.rs` — startup stale claim sweep
- `packages/kernel/src/commands/handler.rs` / `initialize.rs` — startup ordering + diagnostics
- `packages/kernel/src/services/audit.rs` — `record_recovery_diagnostic`
- `packages/database/src/connection.rs` — `run_in_transaction`, WAL pragmas
- `packages/domain/src/recovery_contract.rs` — shared recovery predicates (detectors only)
