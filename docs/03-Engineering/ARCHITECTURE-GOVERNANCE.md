# Architecture Governance & System Integrity

| Field | Value |
|-------|-------|
| **Purpose** | Authoritative rules for architectural invariants, capability boundaries, and integrity failure modes |
| **Owner** | Platform Kernel |
| **Status** | Active |
| **Dependencies** | [Permission Architecture](../07-Security/PERMISSION-ARCHITECTURE.md), [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md), [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md), [Projection Integrity](./PROJECTION-INTEGRITY.md), [Governed Audit Durability](./GOVERNED-AUDIT-DURABILITY.md) |
| **Enforcement** | `pnpm verify:architecture-governance` · `tests/architecture-governance.test.ts` · `cargo test -p workspace-kernel governance_failure` |

## Principle

Data correctness is necessary but not sufficient. As the system grows, **every
layer must remain inside its architectural lane**. This document is the single
authority for those lanes. Related docs specialize lifecycle, persistence, and
projection detail; they do not override this map.

## Ownership model

| Layer | Owns | Must not own |
|-------|------|--------------|
| **Domain** | Canonical models, transition vocabulary, projection DTO shapes | Persistence, process spawn, permission decisions |
| **Services** | Lifecycle authority and continuity writes (after Allow) | Self-authorization, OS execution, React contracts |
| **Kernel commands** | Dispatch, capability declaration, pipeline entry | Lifecycle truth invention, repository policy |
| **Permission Gateway** | Allow / Deny / ApprovalRequired | Execution side effects |
| **Repositories** | Persistence guards, immutable identity fields | Transition choice, service calls |
| **Windows integration** | Process spawn / desktop capture | Capability grants, lifecycle state |
| **React / IPC consumers** | Projection rendering, invoke of registered commands | Database access, lifecycle mutation APIs, fabricated evidence |

Domain services remain lifecycle owners. Repositories enforce stored-state
guards as defense-in-depth. Audit remains append-only evidence.

## Authority boundaries

```
React / IPC / future AI / plugin / automation
        ↓
CommandHandler (actor + intent)
        ↓
CommandPipeline
        ↓
PermissionGateway.require
        ↓
Policy + Gate + allow-once grants
        ↓
Allow | Deny | ApprovalRequired
        ↓
Service (lifecycle owner) → Repository → Domain
        ↓
Execution (only ApplicationLaunchService / windows-integration after Allow)
```

Rules:

1. **Every mutation has a capability path** via `MutationCommand::required_capability`.
2. **Every capability maps to an authority owner** (scope owner in the capability catalog).
3. **No convenience API may bypass `PermissionGateway`**. The only documented
   production exception is shutdown, which still calls `PermissionGateway::require`
   directly because mutable teardown cannot use immutable `CommandContext`.
4. Lifecycle mutators on services are **`pub(crate)`** — callable only after the
   pipeline has authorized.
5. Recommendation Engine **must not** spawn processes or call launch integration.

Machine inventory: `scripts/generated/architecture-map.json` (written by
`pnpm verify:architecture-governance`).

## Lifecycle rules

See [Lifecycle Governance](./LIFECYCLE-GOVERNANCE.md). Summary:

- Invalid transitions fail closed; they do not mutate or emit success events.
- Terminal states do not reopen except where an explicit controlled reopen exists
  (e.g. recommendation generation reopen with changed fingerprint).
- Audit-derived projections never become mutable lifecycle owners.

## Projection rules

See [Projection Integrity](./PROJECTION-INTEGRITY.md). Summary:

- Dual channels: actionable current state + historical evidence + authoritative
  `history_count`.
- History DTOs are evidence-only — **no execute / dismiss / select / mutate /
  command-envelope fields**.
- React is projection-only. Action buttons bind only to actionable projections
  via IPC commands.
- Unknown provenance stays unknown. Never invent `native` origin or fill missing
  outcome identities.

## Persistence rules

See [Persistence Boundary Governance](./PERSISTENCE-BOUNDARY-GOVERNANCE.md). Summary:

- Repositories reject illegal transitions and immutable-artifact mutations.
- Cross-domain repository access remains read-only.
- Interrupted migrations roll back schema and ledger — no partial authority tables.
- Production writers call the owning service; repository guards do not transfer
  authority to the database layer.

## Forbidden patterns

| Pattern | Why forbidden |
|---------|---------------|
| React → Database | UI must never open SQLite or import `workspace-database` |
| React → lifecycle mutation APIs | No local authority; mutations go React → IPC → CommandPipeline |
| React → Domain / Kernel crates | Frontend consumes IPC DTOs only |
| Services bypassing CommandPipeline | Silent authority downgrade |
| Repositories → Services / Kernel | Persistence must not call lifecycle owners |
| Domain → windows-integration / process spawn | Models are not executors |
| Recommendation Engine → Process Spawn | RE records decisions; launch is a separate gated command |
| History DTO gaining `execute` / command fields | Evidence must never become executable |
| Projection inventing terminal evidence | No fabricated outcomes on stale/partial recovery |
| Production `AllowAll` / `AlwaysAllow` wiring | Test-only stubs; never on `WorkspaceKernel` |
| Unsafe retry after non-retryable failure | Execution reconciliation owns retry_allowed |

## Capability boundary audit

Enforced by `scripts/architecture-governance-lib.mjs`:

1. Enumerate every `impl MutationCommand`.
2. Resolve `required_capability()` to a catalog id (`workspace.write`, …).
3. Map each id to an authority owner (`Workspace`, `Application`, …).
4. Confirm `PermissionGateway::require` call sites are only
   `commands/pipeline.rs` and shutdown in `commands/handler.rs`.

## Failure-mode verification

Rust tests in `packages/kernel/src/commands/governance_failure_tests.rs` prove:

| Failure | Required behaviour |
|---------|-------------------|
| Database lock unavailable / poisoned | Error — no Allow |
| Empty capability set | Deny / ApprovalRequired — no silent Allow |
| Corrupted / forged history JSON | Non-commandable; missing evidence identity fails closed |
| Interrupted migration | Migration error; schema + ledger rolled back |
| Stale AllowAll stubs | Not wired on production kernel |
| Partial recovery | Must not invent history rows; `history_count` remains authoritative |

## Related automation

| Check | Command |
|-------|---------|
| Architecture governance | `pnpm verify:architecture-governance` |
| IPC contracts | `pnpm verify:ipc-contract` |
| UI ↔ Experience boundary | `pnpm verify:ui-experience-boundary` |
| Projection integrity (Vitest) | `pnpm test` (`projection-integrity.test.ts`) |
| Domain projection contract | `cargo test -p workspace-domain projection_contract` |
| Governance failure modes | `cargo test -p workspace-kernel governance_failure` |
