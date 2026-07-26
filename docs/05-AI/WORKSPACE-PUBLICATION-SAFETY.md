# Workspace Publication Safety Contract

Sprint 149 — safety requirements a future publication system must satisfy before activation.

**No runtime activation. Failed validation blocks publication. Rollback preserves history.**

## Principle

```
ReadyForPublication
    ↓
Validation
    ↓
MigrationPrepared
    ↓
RollbackPrepared
    ↓
ReleaseApproved
    ↓
Published (future — activation hard-fails)
```

Validation failure → `ValidationFailed` (blocks further progression; history retained).

---

## PublicationSafetyContract

| Area | Contents |
|------|----------|
| Compatibility checks | From `PublicationEnvironment.compatibility_requirements` |
| Migration requirements | Ordered prepare markers (architecture — no SQL apply) |
| Rollback requirements | Rollback target + `history_preserved` |
| Validation gates | Required named gates; any failure blocks |
| Failure handling | `block_publication` / `hard_fail` / preserve history |

Guards:

- `may_execute_commands() == false`
- `may_bypass_permission_gateway() == false`
- `may_mutate_cognition() == false`
- `may_rewrite_provenance() == false`
- `attempt_publish_activate()` → `PublicationActivationNotImplemented`

Lifecycle API: `run_validation` → `prepare_migration` → `prepare_rollback` → `approve_release`.

---

## Audit of reusable patterns

| System | Pattern | Boundary |
|--------|---------|----------|
| **SQLite MigrationRunner** | Ordered versioned steps; failed migration blocks init | Reusable prepare/order mindset — **not** cognition mutation |
| **WorkspaceState / Observation `schema_version`** | Data-shape compatibility identity | Reusable compatibility checks — environment facts only |
| **Command validation** | Precondition fail before pipeline work | Reusable validation gates |
| **Permission Gateway checks** | Allow/Deny/ApprovalRequired | **Separate** execution authority — safety contract never Allows |
| **Audit history** | Append-only event trail | Reusable preserve-history on rollback/failure |

**Do not merge:** Gateway Allow domain with publication safety gates.

---

## Forbidden

Publication safety must not:

- execute commands
- bypass Permission Gateway
- mutate cognition / scoring automatically
- rewrite provenance

---

## Related docs

- [WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md](./WORKSPACE-GOVERNANCE-FAILURE-HANDLING.md)
- [WORKSPACE-GOVERNANCE-LIFECYCLE.md](./WORKSPACE-GOVERNANCE-LIFECYCLE.md)
- [WORKSPACE-GOVERNANCE-WORKSPACE.md](./WORKSPACE-GOVERNANCE-WORKSPACE.md)
- [WORKSPACE-GOVERNANCE-EVIDENCE.md](./WORKSPACE-GOVERNANCE-EVIDENCE.md)
- [WORKSPACE-CONTROLLED-CHANGE.md](./WORKSPACE-CONTROLLED-CHANGE.md)
- [../07-Security/PERMISSION-ARCHITECTURE.md](../07-Security/PERMISSION-ARCHITECTURE.md)
- [WORKSPACE-AUTOMATION-READINESS.md](./WORKSPACE-AUTOMATION-READINESS.md)
