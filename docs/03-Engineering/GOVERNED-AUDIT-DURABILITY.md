# Governed Execution Audit Durability

| Field | Value |
|-------|-------|
| **Purpose** | Define when governed execution audit evidence must persist, when failure is fail-closed, and the explicit post-side-effect exception |
| **Owner** | Platform Kernel |
| **Status** | Active |

## Policy

Governed execution must not proceed without durable authorization evidence. Silent audit failure is forbidden at pre-dispatch boundaries.

| Boundary | Event type(s) | Persistence policy | On failure |
|----------|---------------|--------------------|------------|
| Permission Gateway decision | `permission.allowed`, `permission.denied`, `permission.approval_required` | **Fail-closed** | Return `KernelError::AuditPersistence` (`audit_persistence_error`); no allow/deny/approval outcome is returned as executable |
| Command authorization (pre-handler) | `command.authorized` | **Fail-closed** | Handler is not invoked; no command side effects |
| Command completion (post-handler) | `command.executed`, `command.failed` | **Documented best-effort exception** | Log persistence failure; preserve original command result/error |
| Governed intent execution lifecycle | `execution_lifecycle` claim / completion | **Durable idempotency authority** | Claim failure blocks dispatch; unresolved claims block retry; completed rows remain terminal independently of completion audit |
| Domain-event subscriber / AI operational | domain / AI audit rows | **Documented non-governed-execution exception** | Log and continue; not a CommandPipeline dispatch gate |

There is no internal durable-retry queue. After storage recovers, callers retry when no execution claim exists. `ExecuteIntentRequest` creates a durable `in_progress` claim before mapped dispatch and marks it `completed` before returning success. Dispatch errors release the claim; unresolved claims remain non-dispatchable for explicit reconciliation.

## Fail-closed sequence

```text
PermissionGateway::require
  → evaluate decision
  → persist permission.* evidence   [fail-closed]
  → return Allow | Deny | ApprovalRequired

CommandPipeline (governed path)
  → gateway require
  → persist command.authorized      [fail-closed]
  → command.execute
  → persist command.executed|failed [best-effort]
```

## Why completion audit is best-effort

Once `command.execute` has run, generic rollback is unavailable across heterogeneous handlers. Blocking the caller solely because the completion row failed would hide a completed mutation behind an audit error and invite unsafe double-submit heuristics.

The durable `command.authorized` row recorded **before** dispatch is the required execution evidence. Completion rows remain desirable for outcome analytics but are not a second dispatch gate.

For `ExecuteIntentRequest`, `execution_lifecycle` is separate from secondary audit: it is the durable idempotency fact keyed by the existing `execution:{suggestion_id}` identity. A missing `command.executed` row therefore cannot make completed execution retryable. Cancellation requests persist `cancelled` in the same lifecycle before secondary audit; cancellation remains execution-retryable but cannot be requested twice.

## Recovery

1. Surface `audit_persistence_error` to the IPC caller (sanitized public code).
2. Restore audit storage (disk, SQLite integrity, lock health).
3. Retry the same command only when no durable execution claim exists. Completed claims return `duplicate_execution`; unresolved claims return `execution_in_progress`.

## Classification

`KernelError::AuditPersistence { stage, source }` is distinct from generic `Database` so operators can tell authorization-evidence loss from ordinary CRUD failures. Stages include `permission.decision`, `command.authorization`, `command.completion`, `command.failure`, `ai.operational`, and `domain.event`.

## Related

- `packages/kernel/src/services/audit.rs`
- `packages/kernel/src/security/gateway.rs`
- `packages/kernel/src/commands/pipeline.rs`
- `packages/kernel/src/error.rs`
- [Kernel Error Taxonomy](./KERNEL-ERROR-TAXONOMY.md)
