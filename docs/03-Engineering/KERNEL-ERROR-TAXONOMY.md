# Kernel Error Taxonomy

| Field | Value |
|-------|-------|
| **Purpose** | Define failure classes for `KernelError` and how they map across subsystem and IPC boundaries |
| **Owner** | Platform Kernel |
| **Status** | Active (error taxonomy & failure boundary hardening) |

## Failure Classes

| Class | Variant(s) | Meaning |
|-------|------------|---------|
| Infrastructure | `Database`, `Internal` | Persistence / lock / runtime faults not caused by caller input |
| Audit durability | `AuditPersistence` | Required governed audit evidence could not be persisted (`stage` names the boundary) |
| Execution durability | `ExecutionLifecyclePersistence`, `ExecutionInProgress`, `DuplicateExecution` | Durable execution claim/completion failure or deterministic redispatch denial |
| Configuration | `Config`, `InvalidSettings` | True configuration/settings problems only |
| Authorization | `PermissionDenied`, `ApprovalRequired`, `PermissionApproval*` | Permission Gateway outcomes |
| Architectural integrity | `IntegrityViolation` | Release-safe RE/DE invariant and boundary failures |
| Projection | `ProjectionValidation` | Workspace projection construction/validation only |
| Subsystem validation | `*Validation` | Operational domain validation for a named subsystem |
| Not found / cannot execute | `DecisionEngineNotFound`, `DecisionEngineCannotExecute`, `RecommendationNotFound`, `RecommendationCannotExecute`, resource not-found variants | Explicit lifecycle/authority denials that must not collapse into generic validation |

## Boundary Rules

1. Lock poison and similar infrastructure faults use `Internal` (public code `internal_error`), never `Config`.
2. Runtime invariant failures use `IntegrityViolation` (public code `integrity_violation`), never silent recovery and never debug-only asserts.
3. Recommendation/Decision Engine domain `NotFound` / `CannotExecute` (and related authority denials) preserve dedicated kernel variants through `From` conversions.
4. Observation validation failures use `ObservationValidation`, not `Config`.
5. Permission Gateway failures remain `PermissionDenied` / `ApprovalRequired`.
6. IPC surfaces expose sanitized `PublicError` codes via `KernelError::to_public()`.
7. Fail-closed audit boundaries use `AuditPersistence` (public code `audit_persistence_error`), never silent soft-fail and never collapse into generic `Database` at those gates. See [Governed Execution Audit Durability](./GOVERNED-AUDIT-DURABILITY.md).
8. Execution claim/completion storage failures use `ExecutionLifecyclePersistence`; existing completed or unresolved claims return `DuplicateExecution` / `ExecutionInProgress` before dispatch.

## Related

- `packages/kernel/src/error.rs`
- `packages/kernel/src/services/audit.rs`
- `packages/kernel/src/services/resilience_validation.rs`
- `docs/architecture/RESILIENCE_READINESS_AUDIT.md`
- [Governed Execution Audit Durability](./GOVERNED-AUDIT-DURABILITY.md)
